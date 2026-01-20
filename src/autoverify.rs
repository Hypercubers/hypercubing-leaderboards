use std::{
    collections::{BTreeMap, VecDeque},
    time::Duration,
};

use chrono::TimeDelta;
use hyperspeedcube_cli_types::verification::{Durations, SolveVerification};
use itertools::Itertools;
use tokio::{sync::Mutex, time::timeout};
use tokio_condvar::Condvar;

use crate::{
    AppError, AppResult, AppState,
    db::{SolveDbFields, SolveId},
};

const VERIFICATION_TIMEOUT: Duration = Duration::from_secs(60); // 60 seconds

/// Maximum network latency to allow before requiring manual review of a
/// speedsolve submission.
const MAX_TRUSTED_NETWORK_LATENCY: TimeDelta = TimeDelta::seconds(5);
/// Maximum inspection time to allow before requiring manual review of a
/// speedsolve submission.
const MAX_INSPECTION_TIME: TimeDelta = TimeDelta::seconds(60);
/// Maximum time allowed for scrambling a puzzle before requiring manual review
/// of a speedsolve submission.
const MAX_SCRAMBLE_APPLICATION_TIME: TimeDelta = TimeDelta::seconds(5);
/// Maximum time to allow between a solve's completion and its upload before
/// requiring manual review of a speedsolve or FMC submission, unless the solve
/// completion was timestamped.
const MAX_UPLOAD_GAP: Duration = Duration::from_hours(48); // 2 days

#[derive(Default)]
pub struct SolveAutoVerifier {
    queue: Mutex<VecDeque<SolveId>>,
    condvar: Condvar,
}

impl SolveAutoVerifier {
    pub async fn enqueue(&self, solve: SolveId) {
        let mut queue = self.queue.lock().await;
        if !queue.contains(&solve) {
            tracing::info!("Enqueueing solve {solve} for autoverification");
            queue.push_back(solve);
            self.condvar.notify_one();
        } else {
            tracing::info!("Solve {solve} is already queued for autoverification");
        }
    }

    pub async fn index_of(&self, solve: SolveId) -> Option<usize> {
        self.queue.lock().await.iter().position(|&id| id == solve)
    }

    pub async fn queue_snapshot(&self) -> Vec<SolveId> {
        self.queue.lock().await.iter().copied().collect()
    }

    pub async fn wait_for_next(&self) -> SolveId {
        let mut queue = self.queue.lock().await;
        loop {
            match queue.front() {
                Some(&id) => break id,
                None => queue = self.condvar.wait(queue).await,
            }
        }
    }

    pub async fn pop_next(&self) {
        self.queue.lock().await.pop_front();
    }
}

impl AppState {
    pub async fn autoverify_solve_immediately(&self, solve_id: SolveId) -> AppResult<()> {
        let editor = self.get_hsc_auto_verify_dummy_user().await?;

        tracing::info!("Autoverifying solve {solve_id} ...");

        match self.compute_autoverify_status(solve_id).await {
            Ok(auto_verify_output) => {
                let data = self.get_solve(solve_id).await?;

                let puzzle_id = match self
                    .get_or_create_puzzle_with_hsc_id(&auto_verify_output.puzzle_canonical_id)
                    .await
                {
                    Ok(id) => id,
                    Err(e @ AppError::PuzzleIsNotLeaderboardEligible(_)) => {
                        tracing::info!("Autoverifier rejected solve {solve_id}: {e}");
                        let audit_log_comment = e.to_string();
                        let mut has_move_count = data.move_count.is_some();
                        let has_speed = data.speed_cs.is_some();
                        if !has_move_count && !has_speed {
                            // Ensure there's some data to reject
                            let mut new_data = SolveDbFields::from(data);
                            new_data.move_count = Some(1);
                            has_move_count = true;
                            self.update_solve(
                                solve_id,
                                new_data,
                                &editor,
                                "Adding move count so the solve can be rejected",
                            )
                            .await?;
                        }
                        if has_move_count {
                            self.verify_fmc(&editor, solve_id, Some(false), &audit_log_comment)
                                .await?;
                        }
                        if has_speed {
                            self.verify_speed(&editor, solve_id, Some(false), &audit_log_comment)
                                .await?;
                        }
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                };

                let Durations {
                    scramble_network_latency,
                    scramble_application,
                    inspection,
                    speedsolve,
                    memo,
                    blindsolve,
                    timestamp_network_latency,
                } = auto_verify_output.durations;

                let mut reasons_to_not_autoverify_anything = vec![];
                let mut reasons_to_not_autoverify_speed = vec![];

                let fields = SolveDbFields {
                    puzzle_id: puzzle_id.0,
                    variant_id: data.variant.map(|v| {
                        reasons_to_not_autoverify_anything
                            .push("Variants require manual review".to_string());
                        v.id.0
                    }),
                    program_id: {
                        if data.program.abbr != "HSC2" {
                            reasons_to_not_autoverify_anything
                                .push("Programs other than HSC2 require manual review".to_string());
                        }
                        data.program.id.0
                    },
                    solver_id: data.solver.id.0,
                    solve_date: {
                        auto_verify_output
                            .verified_timestamps
                            .completion
                            .unwrap_or_else(|| {
                                if data.solve_date < data.upload_date - MAX_UPLOAD_GAP {
                                    reasons_to_not_autoverify_anything.push(format!(
                                        "Solve was uploaded {} days after claimed completion date",
                                        (data.upload_date - data.solve_date).num_days(),
                                    ));
                                }
                                data.solve_date.min(data.upload_date)
                            })
                    },
                    solver_notes: data
                        .solver_notes
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            reasons_to_not_autoverify_anything
                                .push("Solver note requires manual review".to_string());
                            s
                        })
                        .unwrap_or_default(),
                    moderator_notes: data.moderator_notes,
                    auto_verify_output: Some(serde_json::to_value(&auto_verify_output)?),
                    average: {
                        if data.flags.average {
                            return Err(AppError::Other("averages cannot be autoverified".into()));
                        }
                        data.flags.average
                    },
                    blind: blindsolve.is_some(),
                    filters: auto_verify_output.used_filters,
                    macros: auto_verify_output.used_macros,
                    one_handed: {
                        if data.flags.one_handed {
                            reasons_to_not_autoverify_anything
                                .push("One-handed solve requires manual review".to_string());
                        }
                        data.flags.one_handed
                    },
                    computer_assisted: data.flags.computer_assisted, // we trust
                    move_count: auto_verify_output.solution_stm.try_into().ok(),
                    speed_cs: blindsolve
                        .or(speedsolve)
                        .and_then(duration_to_cs)
                        .or_else(|| {
                            if data.speed_cs.is_some() {
                                reasons_to_not_autoverify_speed
                                    .push("Speedsolve autoverification failed".to_string());
                            }
                            data.speed_cs
                        }),
                    memo_cs: memo.and_then(duration_to_cs).or_else(|| {
                        if data.memo_cs.is_some() {
                            reasons_to_not_autoverify_speed
                                .push("Memorization time requires manual review".to_string());
                        }
                        data.memo_cs
                    }),
                    log_file: None, // don't change
                    video_url: {
                        if data.video_url.is_some() {
                            reasons_to_not_autoverify_anything
                                .push("Video requires manual review".to_string());
                        }
                        data.video_url
                    },
                };

                for (name, duration, max_time) in [
                    (
                        "Scramble network latency",
                        scramble_network_latency,
                        MAX_TRUSTED_NETWORK_LATENCY,
                    ),
                    (
                        "Scramble application time",
                        scramble_application,
                        MAX_SCRAMBLE_APPLICATION_TIME,
                    ),
                    ("Inspection time", inspection, MAX_INSPECTION_TIME),
                    (
                        "Timestamp network latency",
                        timestamp_network_latency,
                        MAX_TRUSTED_NETWORK_LATENCY,
                    ),
                ] {
                    match duration {
                        Some(dur) => {
                            if dur > max_time {
                                reasons_to_not_autoverify_speed.push(format!("{name} was {dur}"));
                            }
                        }
                        None => reasons_to_not_autoverify_speed.push(format!("{name} unknown")),
                    }
                }

                let mut audit_log_comment = itertools::chain!(
                    &reasons_to_not_autoverify_anything,
                    &reasons_to_not_autoverify_speed,
                    &auto_verify_output.errors,
                )
                .join("\n");
                if !audit_log_comment.is_empty() {
                    audit_log_comment = format!("Unable to autoverify:\n{audit_log_comment}");
                }

                let verify_fmc =
                    fields.move_count.is_some() && reasons_to_not_autoverify_anything.is_empty();
                let verify_speed = fields.speed_cs.is_some()
                    && reasons_to_not_autoverify_anything.is_empty()
                    && reasons_to_not_autoverify_speed.is_empty();

                self.update_solve(solve_id, fields, &editor, &audit_log_comment)
                    .await?;

                if verify_fmc {
                    self.verify_fmc(&editor, solve_id, Some(true), "").await?;
                }
                if verify_speed {
                    self.verify_speed(&editor, solve_id, Some(true), "").await?;
                }

                tracing::info!("Autoverification of solve {solve_id} succeeded");
            }
            Err(e) => {
                let mut transaction = self.pool.begin().await?;
                Self::add_solve_log_entry(
                    &mut transaction,
                    &editor,
                    solve_id,
                    crate::db::AuditLogEvent::Updated {
                        object: None,
                        fields: BTreeMap::new(),
                        comment: Some(e.to_string()),
                    },
                )
                .await?;
                transaction.commit().await?;

                tracing::info!("Autoverification of solve {solve_id} failed");
            }
        }

        Ok(())
    }

    async fn compute_autoverify_status(&self, solve: SolveId) -> AppResult<SolveVerification> {
        let log_file_contents = self
            .get_log_file_contents(solve, &self.pool)
            .await?
            .unwrap_or_default();

        let f = tempfile::NamedTempFile::new()?;
        std::fs::write(&f, log_file_contents)?;

        let output = timeout(
            VERIFICATION_TIMEOUT,
            async_process::Command::new(&*crate::env::HSC2_PATH)
                .arg("verify")
                .arg(f.path())
                .output(),
        )
        .await
        .map_err(|_| AppError::SolveVerificationTimeout)??;

        drop(f);

        let facts: Vec<SolveVerification> = serde_json::from_slice(&output.stdout)?;

        facts.into_iter().next().ok_or_else(|| {
            AppError::Other(format!(
                "no verification output. stdout:\n{}",
                String::from_utf8_lossy(&output.stderr),
            ))
        })
    }
}

fn duration_to_cs(dur: TimeDelta) -> Option<i32> {
    (dur.num_milliseconds() / 10).try_into().ok()
}
