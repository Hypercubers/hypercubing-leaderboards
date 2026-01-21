use crate::db::{Category, Event, EventClass, FullSolve, SolveId, User};
use crate::traits::Linkable;
use crate::{AppResult, AppState};

pub struct MdSolveTime<'a>(pub &'a FullSolve);
impl Linkable for MdSolveTime<'_> {
    fn relative_url(&self) -> String {
        self.0.relative_url()
    }

    fn md_text(&self) -> String {
        match self.0.speed_cs {
            Some(cs) => crate::util::render_time(cs),
            None => self.0.md_text(),
        }
    }
}

pub struct MdSolveMoveCount<'a>(pub &'a FullSolve);
impl Linkable for MdSolveMoveCount<'_> {
    fn relative_url(&self) -> String {
        self.0.relative_url()
    }

    fn md_text(&self) -> String {
        match self.0.move_count {
            Some(move_count) => format!("{move_count} STM"),
            None => self.0.md_text(),
        }
    }
}

pub struct MdSolveInEvent<'a>(pub &'a FullSolve, pub EventClass);
impl Linkable for MdSolveInEvent<'_> {
    fn relative_url(&self) -> String {
        self.0.relative_url()
    }

    fn md_text(&self) -> String {
        match self.1 {
            EventClass::Speed => MdSolveTime(self.0).md_text(),
            EventClass::Fmc => MdSolveMoveCount(self.0).md_text(),
        }
    }
}

impl AppState {
    pub async fn send_private_discord_update(&self, message: String) {
        let Ok(discord) = self.try_discord() else {
            return;
        };
        let result = crate::env::PRIVATE_UPDATES_CHANNEL_ID
            .say(discord, message)
            .await;

        if let Err(err) = result {
            tracing::warn!(%err, "Failed to alert discord to solve update");
        }
    }

    async fn alert_discord_if_no_pending_solves(&self) {
        if self
            .get_pending_submissions()
            .await
            .is_ok_and(|list| list.is_empty())
        {
            self.send_private_discord_update(
                ":partying_face: No pending submissions! Good work, team! :dancer:".to_string(),
            )
            .await;
        }
    }

    pub async fn alert_discord_of_solve(
        &self,
        editor: &User,
        solve_id: SolveId,
        updated: bool,
        will_be_auto_verified: bool,
    ) {
        let Ok(solve) = self.get_solve(solve_id).await else {
            return;
        };

        let emoji = if will_be_auto_verified {
            ":inbox_tray:"
        } else if solve.pending_review() {
            ":person_raising_hand:"
        } else {
            ":information_source:"
        };
        let event = if updated {
            "Solve updated"
        } else if will_be_auto_verified {
            "Solve submitted for auto-verification"
        } else {
            "Solve submitted for manual verification"
        };
        let solve_markdown = solve.markdown_with_puzzle_and_solver_name();
        let by_whom = if editor.id == solve.solver.id {
            String::new()
        } else {
            format!(" by {}", editor.to_public().md_link(false))
        };

        self.send_private_discord_update(format!("{emoji} {event}: {solve_markdown}{by_whom}"))
            .await;
    }

    pub async fn alert_discord_of_manual_verification(
        &self,
        editor: &User,
        solve_id: SolveId,
        event_class: EventClass,
    ) {
        let Ok(solve) = self.get_solve(solve_id).await else {
            return;
        };

        let solve_markdown = solve.markdown_with_puzzle_and_solver_name();

        let verb_prefix = match event_class {
            EventClass::Speed => "speed-",
            EventClass::Fmc => "FMC-",
        };

        let status = match event_class {
            EventClass::Speed => solve.speed_verified,
            EventClass::Fmc => solve.fmc_verified,
        };
        let (emoji, verbed) = match status {
            Some(true) => (":ballot_box_with_check:", "accepted"),
            Some(false) => (":x:", "rejected"),
            None => (":new_moon_with_face:", "unverified"),
        };

        let editor_name = editor.to_public().display_name();

        self.send_private_discord_update(format!(
            "{emoji} {solve_markdown} {verb_prefix}{verbed} by {editor_name}",
        ))
        .await;

        self.alert_discord_if_no_pending_solves().await;
    }

    /// Alerts the private Discord channel that a solve has been analyzed by the
    /// auto-verification system, even if it completely failed.
    ///
    /// If `editor` is `None`, then it is assumed to be the autoverifier.
    pub async fn alert_discord_of_auto_verification(&self, solve_id: SolveId) {
        let Ok(solve) = self.get_solve(solve_id).await else {
            return;
        };

        let solve_markdown = solve.markdown_with_puzzle_and_solver_name();

        let needs_manual_review = solve.pending_review();
        let speed_status = solve.speed_verified;
        let fmc_status = solve.fmc_verified;
        let accepted = speed_status == Some(true) || fmc_status == Some(true);
        let rejected = speed_status == Some(false) || fmc_status == Some(false);
        let (emoji, verbed) = if rejected && !needs_manual_review {
            (":x:", "rejected")
        } else if accepted & !needs_manual_review {
            (":ballot_box_with_check:", "accepted")
        } else {
            (":warning:", "needs manual review")
        };

        self.send_private_discord_update(format!(":robot: {emoji} {solve_markdown} {verbed}"))
            .await;
    }

    pub async fn alert_discord_to_speed_record(&self, solve_id: SolveId) {
        // async block to mimic try block
        let send_result = async {
            let discord = self.try_discord()?;

            let solve = self.get_solve(solve_id).await?;

            let mut wr_event = None;
            let mut displaced_wr = None;

            let event = solve.speed_event();

            let mut primary_event = event.clone();
            if let Category::Speed {
                filters, macros, ..
            } = &mut primary_event.category
            {
                *filters = solve.puzzle.primary_filters;
                *macros = solve.puzzle.primary_macros;
            }

            // Prefer reporting for the primary category
            if solve.counts_for_primary_speed_category() {
                if let Some(old_wr) = self.world_record_excluding(&primary_event, &solve).await? {
                    if solve.speed_cs <= old_wr.speed_cs {
                        wr_event = Some(&primary_event);
                        displaced_wr = Some(old_wr);
                    }
                } else {
                    wr_event = Some(&primary_event);
                }
            }
            // If it's not a WR in the primary category, try reporting for its
            // own category
            if wr_event.is_none() {
                if let Some(old_wr) = self.world_record_excluding(&event, &solve).await? {
                    if solve.speed_cs <= old_wr.speed_cs {
                        wr_event = Some(&event);
                        displaced_wr = Some(old_wr);
                    }
                } else {
                    wr_event = Some(&event);
                }
            }

            let Some(wr_event) = wr_event else {
                return Ok(()); // not a world record; nothing to report
            };

            let msg = build_wr_msg(&solve, displaced_wr.as_ref(), wr_event);

            crate::env::PUBLIC_UPDATES_CHANNEL_ID
                .say(discord, msg)
                .await?;

            Ok::<_, Box<dyn std::error::Error>>(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(?solve_id, err, "Failed to alert discord to new record");
        }
    }

    pub async fn alert_discord_to_fmc_record(&self, solve_id: SolveId) {
        // async block to mimic try block
        let send_result: AppResult = async {
            let discord = self.try_discord()?;

            let solve = self.get_solve(solve_id).await?;

            let mut displaced_wr = None;

            let event = solve.fmc_event();

            if let Some(old_wr) = self.world_record_excluding(&event, &solve).await? {
                if solve.move_count <= old_wr.move_count {
                    displaced_wr = Some(old_wr);
                } else {
                    return Ok(()); // not a world record; nothing to report
                }
            }

            let msg = build_wr_msg(&solve, displaced_wr.as_ref(), &event);
            crate::env::PUBLIC_UPDATES_CHANNEL_ID
                .say(discord, msg)
                .await?;

            Ok(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(?solve_id, %err, "Failed to alert discord to new record");
        }
    }
}

fn build_wr_msg(solve: &FullSolve, displaced_wr: Option<&FullSolve>, wr_event: &Event) -> String {
    let mut msg = crate::sy::MessageBuilder::new();

    let event_class = wr_event.category.class();

    msg.push("### 🏆 ")
        .push(solve.solver.md_link(false))
        .push(" set a ")
        .push(MdSolveInEvent(solve, event_class).md_link(false))
        .push(" ")
        .push(event_class.long_name())
        .push(" record for ")
        .push(wr_event.md_link(false))
        .push_line("!");

    match displaced_wr {
        None => {
            msg.push_line("This is the first solve in the category! 🎉");
        }
        Some(old_wr) => {
            let tied = match &wr_event.category {
                Category::Speed { .. } => old_wr.speed_cs == solve.speed_cs,
                Category::Fmc { .. } => old_wr.move_count == solve.move_count,
            };
            msg.push("They have ");
            msg.push(if tied { "tied" } else { "defeated" });
            if old_wr.solver.id == solve.solver.id {
                msg.push(" their previous record of ")
                    .push(MdSolveInEvent(old_wr, event_class).md_link(false))
                    .push(".");
            } else {
                msg.push(" the previous record of ")
                    .push(MdSolveInEvent(old_wr, event_class).md_link(false))
                    .push(" by ")
                    .push(old_wr.solver.md_link(false))
                    .push(".");
            }
        }
    }
    msg.push_line("");
    msg.push_line(match &solve.video_url {
        None => "".to_string(),
        Some(url) => format!("-# [Video link]({url})"),
    });
    msg.build()
}
