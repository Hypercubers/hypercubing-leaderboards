#![allow(dead_code)]
use crate::api::upload::{
    UpdateSolveCategory, UpdateSolveMoveCount, UpdateSolveProgramVersionId, UpdateSolveSpeedCs,
    UpdateSolveVideoUrl, UploadSolveExternal,
};
use crate::db::program::{Program, ProgramVersion};
use crate::db::puzzle::Puzzle;
use crate::db::puzzle::PuzzleCategory;
use crate::db::puzzle::PuzzleCategoryBase;
use crate::db::puzzle::PuzzleCategoryFlags;
use crate::db::user::PublicUser;
use crate::db::user::User;
use crate::db::EditAuthorization;
use crate::util::render_time;
use crate::AppState;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::Connection;
use sqlx::{query, query_as};
use std::collections::HashSet;

#[derive(Serialize)]
pub struct Solve {
    pub id: i32,
    pub log_file: Option<String>,
    pub user: User,
    pub upload_time: DateTime<Utc>,
    pub puzzle: Puzzle,
    pub move_count: Option<i32>,
    pub uses_macros: bool,
    pub uses_filters: bool,
    pub blind: bool,
    pub scramble_seed: Option<String>,
    pub program_version: ProgramVersion,
    pub speed_evidence: Option<SpeedEvidence>,
    pub log_file_verified: Option<bool>,
    pub solver_notes: String,
    pub moderator_notes: String,
}

#[derive(Serialize)]
pub struct SpeedEvidence {
    pub id: i32,
    pub solve_id: i32,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,
    pub video_url: Option<String>,
    pub verified: Option<bool>,
    pub verified_by: Option<i32>,
    pub moderator_notes: String,
}

#[derive(Clone, Serialize)]
pub struct FullSolve {
    pub id: i32,
    pub log_file: Option<String>,
    pub user_id: i32,
    pub upload_time: DateTime<Utc>,
    pub puzzle_id: i32,
    pub move_count: Option<i32>,
    pub uses_macros: bool,
    pub uses_filters: bool,
    pub blind: bool,
    pub scramble_seed: Option<String>,
    pub program_version_id: i32,
    pub log_file_verified: Option<bool>,
    pub solver_notes: String,
    pub display_name: Option<String>,
    pub program_id: i32,
    pub version: Option<String>,
    pub program_name: String,
    pub abbreviation: String,
    pub puzzle_name: String,
    pub primary_filters: bool,
    pub primary_macros: bool,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,
    pub video_url: Option<String>,
    pub speed_verified: Option<bool>,
    pub rank: Option<i32>,
}

macro_rules! make_leaderboard_solve {
    ( $row:expr ) => {
        FullSolve {
            id: $row.id.expect("column id not null"),
            log_file: $row.log_file,
            user_id: $row.user_id.expect("column user_id not null"),
            upload_time: $row.upload_time.expect("column upload_time not null"),
            puzzle_id: $row.puzzle_id.expect("column puzzle_id not null"),
            move_count: $row.move_count,
            uses_macros: $row.uses_macros.expect("column uses_macros not null"),
            uses_filters: $row.uses_filters.expect("column uses_filters not null"),
            blind: $row.blind.expect("column blind not null"),
            scramble_seed: $row.scramble_seed,
            program_version_id: $row
                .program_version_id
                .expect("column program_version_id not null"),
            log_file_verified: $row.log_file_verified,
            solver_notes: $row.solver_notes.expect("column solver_notes not null"),
            display_name: $row.display_name,
            program_id: $row.program_id.expect("column program_id not null"),
            version: $row.version,
            program_name: $row.program_name.expect("column program_name not null"),
            abbreviation: $row.abbreviation.expect("column abbreviation not null"),
            puzzle_name: $row.puzzle_name.expect("column puzzle_name not null"),
            primary_filters: $row
                .primary_filters
                .expect("column primary_filters not null"),
            primary_macros: $row.primary_macros.expect("column primary_macros not null"),
            speed_cs: $row.speed_cs,
            memo_cs: $row.memo_cs,
            video_url: $row.video_url,
            speed_verified: $row.speed_verified,
            rank: None,
        }
    };
}

impl FullSolve {
    pub fn user(&self) -> PublicUser {
        PublicUser {
            id: self.user_id,
            display_name: self.display_name.clone(),
        }
    }

    pub fn program_version(&self) -> ProgramVersion {
        ProgramVersion {
            id: self.program_version_id,
            program: Program {
                id: self.program_id,
                name: self.program_name.clone(),
                abbreviation: self.abbreviation.clone(),
            },
            version: self.version.clone(),
        }
    }

    pub fn puzzle(&self) -> Puzzle {
        Puzzle {
            id: self.puzzle_id,
            name: self.puzzle_name.clone(),
            primary_flags: PuzzleCategoryFlags {
                uses_filters: self.primary_filters,
                uses_macros: self.primary_macros,
            },
        }
    }

    pub fn puzzle_category(&self) -> PuzzleCategory {
        PuzzleCategory {
            base: PuzzleCategoryBase {
                puzzle: self.puzzle(),
                blind: self.blind,
            },
            flags: PuzzleCategoryFlags {
                uses_filters: self.uses_filters,
                uses_macros: self.uses_macros,
            },
        }
    }

    pub fn embed_fields(
        &self,
        mut embed: serenity::all::CreateEmbed,
    ) -> serenity::all::CreateEmbed {
        embed = embed.field("Solve ID", self.id.to_string(), true);

        if let Some(speed_cs) = self.speed_cs {
            if let Some(memo_cs) = self.memo_cs {
                embed = embed.field(
                    "Time",
                    format!("{} ({})", render_time(speed_cs), render_time(memo_cs)),
                    true,
                );
            } else {
                embed = embed.field("Time", render_time(speed_cs), true);
            }
        }

        if let Some(video_url) = &self.video_url {
            embed = embed.field("Video URL", video_url.to_string(), true);
        }

        let puzzle_category = self.puzzle_category();
        embed = embed.field("Solver", self.user().name(), true).field(
            "Puzzle",
            puzzle_category.base.name() + &puzzle_category.flags.format_modifiers(),
            true,
        );

        if let Some(move_count) = self.move_count {
            embed = embed.field("Move count", move_count.to_string(), true);
        }

        embed = embed.field("Program", self.program_version().name(), true);

        if !self.solver_notes.is_empty() {
            embed = embed.field("Solver notes", self.solver_notes.clone(), true);
        }

        embed
    }

    pub fn sort_key(&self) -> impl Ord {
        (self.speed_cs.is_none(), self.speed_cs, self.upload_time)
    }

    pub fn url_path(&self) -> String {
        format!("/solve?id={}", self.id)
    }

    pub fn can_edit(&self, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if self.user_id == editor.id
            && !self.log_file_verified.unwrap_or(false)
            && !self.speed_verified.unwrap_or(false)
        {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }

    pub fn can_edit_opt(&self, editor: Option<&User>) -> Option<EditAuthorization> {
        editor.map(|editor| self.can_edit(editor)).flatten()
    }

    pub fn rank_key(&self) -> impl Ord {
        (self.speed_cs.is_none(), self.speed_cs, self.upload_time)
    }

    pub fn beats(&self, other: &Self) -> bool {
        self.rank_key() < other.rank_key()
    }
}

pub enum RecordType {
    First,
    FirstSpeed,
    Speed,
    Tie,
}

impl AppState {
    pub async fn get_full_solve(&self, id: i32) -> sqlx::Result<Option<FullSolve>> {
        Ok(query!(
            "SELECT *
                FROM FullSolve
                WHERE id = $1
            ",
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|row| make_leaderboard_solve!(row)))
    }

    pub async fn get_leaderboard_solve(&self, id: i32) -> sqlx::Result<Option<FullSolve>> {
        Ok(query!(
            "SELECT *
                FROM LeaderboardSolve
                WHERE id = $1
            ",
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|row| make_leaderboard_solve!(row)))
    }

    pub async fn get_leaderboard_puzzle(
        &self,
        puzzle_category: &PuzzleCategory,
    ) -> sqlx::Result<Vec<FullSolve>> {
        let mut solves = vec![];
        for puzzle_category in puzzle_category.subcategories() {
            solves.extend(
                query!(
                    "SELECT DISTINCT ON (user_id) *
                        FROM LeaderboardSolve
                        WHERE puzzle_id = $1
                            AND blind = $2
                            AND uses_filters = $3
                            AND uses_macros = $4
                        ORDER BY user_id, speed_cs ASC NULLS LAST, upload_time
                    ",
                    puzzle_category.base.puzzle.id,
                    puzzle_category.base.blind,
                    puzzle_category.flags.uses_filters,
                    puzzle_category.flags.uses_macros
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|row| make_leaderboard_solve!(row)),
            )
        }

        // make sure the fastest solve is the one kept when dedup by user id
        solves.sort_by_key(FullSolve::sort_key);
        solves.sort_by_key(|solve| solve.user_id);
        solves.dedup_by_key(|solve| solve.user_id);
        solves.sort_by_key(FullSolve::sort_key);

        Ok(solves)
    }

    pub async fn get_leaderboard_global(&self) -> sqlx::Result<Vec<FullSolve>> {
        Ok(query!(
            "SELECT DISTINCT ON (puzzle_id, blind, uses_filters, uses_macros) *
                FROM LeaderboardSolve
                ORDER BY puzzle_id, blind, uses_filters, uses_macros, speed_cs ASC NULLS LAST, upload_time
            ",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| make_leaderboard_solve!(row))
        .collect())
    }

    pub async fn get_records_puzzle(&self, puzzle_id: i32) -> sqlx::Result<Vec<FullSolve>> {
        Ok(query!(
            "SELECT DISTINCT ON (blind, uses_filters, uses_macros) *
                FROM LeaderboardSolve
                WHERE puzzle_id = $1
                ORDER BY blind, uses_filters, uses_macros, speed_cs ASC NULLS LAST, upload_time
            ",
            puzzle_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| make_leaderboard_solve!(row))
        .collect())
    }

    pub async fn get_leaderboard_solver(&self, user_id: i32) -> sqlx::Result<Vec<FullSolve>> {
        Ok(query!(
            "SELECT DISTINCT ON (puzzle_id, blind, uses_filters, uses_macros) *
                FROM LeaderboardSolve
                WHERE user_id = $1
                ORDER BY puzzle_id, blind, uses_filters, uses_macros, speed_cs ASC NULLS LAST, upload_time
            ",
            user_id,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| make_leaderboard_solve!(row))
        .collect())
    }

    pub async fn get_rank(
        &self,
        puzzle_category: &PuzzleCategory,
        solve: &FullSolve,
    ) -> sqlx::Result<i32> {
        // TODO: replace with RANK()
        let mut users_less = HashSet::<i32>::new();
        for puzzle_category in puzzle_category.subcategories() {
            users_less.extend(
                query!(
                    "SELECT DISTINCT ON (user_id) user_id
                    FROM LeaderboardSolve
                    WHERE puzzle_id = $1
                        AND blind = $2
                        AND uses_filters = $3
                        AND uses_macros = $4
                        AND (
                            ((speed_cs < $5) IS TRUE)
                            OR ((speed_cs IS NOT NULL) and ($5 IS NOT NULL) AND (upload_time < $6))
                            OR (($5 IS NULL) AND NOT (speed_cs IS NULL))
                            OR (($5 IS NULL) AND (speed_cs IS NULL) AND (upload_time < $6))
                        )
                    ORDER BY user_id, speed_cs ASC NULLS LAST, upload_time
                    ",
                    puzzle_category.base.puzzle.id,
                    puzzle_category.base.blind,
                    puzzle_category.flags.uses_filters,
                    puzzle_category.flags.uses_macros,
                    solve.speed_cs,
                    solve.upload_time
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .filter_map(|r| r.user_id),
            );
            //dbg!(puzzle_category.puzzle_id, &users_less);
        }

        Ok(users_less.len() as i32 + 1)
    }

    pub async fn is_record(&self, solve: &FullSolve) -> sqlx::Result<Option<RecordType>> {
        if solve.speed_cs.is_none() {
            // for now, do not alert for non-timed solves
            // how should this be handled when the solve is verified before the speed?
            // this will happen when hsc2 verifies log files
            return Ok(None);
        }

        let records: Vec<_> = self
            .get_records_puzzle(solve.puzzle_id)
            .await?
            .into_iter()
            .filter(|record| {
                record
                    .puzzle_category()
                    .flags
                    .in_category(&solve.puzzle_category().flags)
            })
            .collect();

        if records
            .iter()
            .all(|record| solve.rank_key() <= record.rank_key())
        {
            let mut is_first = true;
            // no any since it's async
            for category in solve.puzzle_category().subcategories() {
                // it's possible that a solve in a narrower category beat this one to first
                let count_all = query!(
                    "SELECT COUNT(*) 
                       FROM LeaderboardSolve
                       WHERE puzzle_id = $1
                           AND blind = $2
                           AND uses_filters = $3
                           AND uses_macros = $4
                           AND id <> $5
                           AND speed_cs IS NOT NULL
                       LIMIT 1
                   ",
                    solve.puzzle_id,
                    solve.blind,
                    category.flags.uses_filters,
                    category.flags.uses_macros,
                    solve.id
                )
                .fetch_one(&self.pool)
                .await?
                .count
                .expect("count cannot be null");

                if count_all > 0 {
                    is_first = false;
                    break;
                }
            }

            if is_first {
                Ok(Some(RecordType::FirstSpeed))
            } else {
                Ok(Some(RecordType::Speed))
            }
        } else if records
            .into_iter()
            .all(|record| solve.speed_cs <= record.speed_cs)
        {
            Ok(Some(RecordType::Tie))
        } else {
            Ok(None)
        }
    }

    pub async fn alert_discord_to_verify(&self, solve_id: i32, updated: bool) {
        let send_result: Result<(), Box<dyn std::error::Error>> = async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self
                .get_leaderboard_solve(solve_id)
                .await?
                .ok_or("no solve")?;

            // send solve for verification
            let embed = CreateEmbed::new()
                .title(if updated {
                    "Updated solve"
                } else {
                    "New solve"
                })
                .url(format!(
                    "{}{}",
                    dotenvy::var("DOMAIN_NAME")?,
                    solve.url_path()
                ));
            let embed = solve.embed_fields(embed);
            let builder = CreateMessage::new().embed(embed);

            let channel = ChannelId::new(dotenvy::var("VERIFICATION_CHANNEL_ID")?.parse()?);
            channel.send_message(discord.clone(), builder).await?;
            Ok(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(solve_id, err, "failed to alert discord to new solve");
        }
    }

    pub async fn add_solve_external(
        &self,
        user_id: i32,
        item: UploadSolveExternal,
    ) -> sqlx::Result<i32> {
        //let item = item.clone(); // may be inefficient if log file is large
        let solve_id = self
            .pool
            .acquire()
            .await?
            .detach()
            .transaction(move |txn| {
                Box::pin(async move {
                    let solve_id = query!(
                        "INSERT INTO Solve
                                (log_file, user_id, puzzle_id, move_count,
                                uses_macros, uses_filters,
                                blind, program_version_id,
                                speed_cs, memo_cs, video_url) 
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                            RETURNING id",
                        item.log_file,
                        user_id,
                        item.puzzle_id,
                        item.move_count,
                        item.uses_macros,
                        item.uses_filters,
                        item.blind,
                        item.program_version_id,
                        item.speed_cs,
                        if item.blind { item.memo_cs } else { None },
                        item.video_url
                    )
                    .fetch_optional(&mut **txn)
                    .await?
                    .expect("upload should work")
                    .id;

                    Ok::<i32, sqlx::Error>(solve_id)
                })
            })
            .await?;

        self.alert_discord_to_verify(solve_id, false).await;

        tracing::info!(user_id, solve_id, "uploaded external solve");

        Ok(solve_id)
    }

    pub async fn update_video_url(&self, item: &UpdateSolveVideoUrl) -> sqlx::Result<()> {
        query!(
            "UPDATE Solve
                SET video_url = $1
                WHERE Solve.id = $2",
            item.video_url,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;

        tracing::info!(solve_id = item.solve_id, "updated video_url on solve");
        Ok(())
    }

    pub async fn update_speed_cs(&self, item: &UpdateSolveSpeedCs) -> sqlx::Result<()> {
        query!(
            "UPDATE Solve
                SET speed_cs = $1
                WHERE Solve.id = $2",
            item.speed_cs,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;

        tracing::info!(solve_id = item.solve_id, "updated video_url on solve");
        Ok(())
    }

    pub async fn update_solve_category(&self, item: &UpdateSolveCategory) -> sqlx::Result<()> {
        query!(
            "UPDATE Solve
                SET
                    puzzle_id = $1,
                    blind = $2,
                    uses_filters = $3,
                    uses_macros = $4
                WHERE Solve.id = $5",
            item.puzzle_id,
            item.blind,
            item.uses_filters,
            item.uses_macros,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;

        tracing::info!(solve_id = item.solve_id, "updated puzzle category on solve");
        Ok(())
    }

    pub async fn update_move_count(&self, item: &UpdateSolveMoveCount) -> sqlx::Result<()> {
        query!(
            "UPDATE Solve
                SET move_count = $1
                WHERE Solve.id = $2",
            item.move_count,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;

        tracing::info!(solve_id = item.solve_id, "updated move_count on solve");
        Ok(())
    }

    pub async fn update_solve_program_version_id(
        &self,
        item: &UpdateSolveProgramVersionId,
    ) -> sqlx::Result<()> {
        query!(
            "UPDATE Solve
                SET program_version_id = $1
                WHERE Solve.id = $2",
            item.program_version_id,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;

        tracing::info!(
            solve_id = item.solve_id,
            "updated program_version_id on solve"
        );
        Ok(())
    }

    pub async fn verify_speed(&self, id: i32, mod_id: i32) -> sqlx::Result<Option<()>> {
        let solve_id = query!(
            "UPDATE Solve
                SET speed_verified_by = $2
                WHERE id = $1
                RETURNING id
            ",
            id,
            mod_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.id);

        let Some(solve_id) = solve_id else {
            return Ok(None);
        };

        // async block to mimic try block
        let send_result = async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self
                .get_leaderboard_solve(solve_id)
                .await?
                .ok_or("no solve")?;

            if let Some(record_type) = self.is_record(&solve).await? {
                let mut builder = MessageBuilder::new();
                match record_type {
                    RecordType::Speed => {
                        builder
                            .push("🏆")
                            .push_bold_safe(solve.user().name())
                            .push(" has broken the record for ")
                            .push_bold_safe(solve.puzzle_category().base.name());

                        builder.push(solve.puzzle_category().flags.format_modifiers());
                    }
                    RecordType::Tie => {
                        builder
                            .push("🏅")
                            .push_bold_safe(solve.user().name())
                            .push(" has tied the record for ")
                            .push_bold_safe(solve.puzzle_category().base.name());

                        builder.push(solve.puzzle_category().flags.format_modifiers());
                    }
                    RecordType::FirstSpeed => {
                        builder
                            .push("🎉")
                            .push_bold_safe(solve.user().name())
                            .push(" is the first to speedsolve ")
                            .push_bold_safe(solve.puzzle_category().base.name());

                        builder.push(solve.puzzle_category().flags.format_modifiers());
                    }
                    RecordType::First => {}
                }

                if let Some(speed_cs) = solve.speed_cs {
                    builder
                        .push(" with a time of ")
                        .push_bold_safe(render_time(speed_cs));
                }
                builder.push_line("!");

                if let Some(ref video_url) = solve.video_url {
                    builder.push_safe(format!("[Video link]({}) • ", video_url));
                }

                builder.push_safe(format!(
                    "[Solve link]({}{})",
                    dotenvy::var("DOMAIN_NAME")?,
                    solve.url_path()
                ));

                let channel = ChannelId::new(dotenvy::var("UPDATE_CHANNEL_ID")?.parse()?);
                channel.say(discord, builder.build()).await?;
            }

            Ok::<_, Box<dyn std::error::Error>>(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(solve_id, err, "failed to alert discord to new record");
        }

        tracing::info!(mod_id, solve_id, "uploaded external solve");

        Ok(Some(()))
    }
}
