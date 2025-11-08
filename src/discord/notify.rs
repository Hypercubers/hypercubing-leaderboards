use crate::db::{Category, Event, FullSolve, SolveId};
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
            Some(move_count) => move_count.to_string(),
            None => self.0.md_text(),
        }
    }
}

impl AppState {
    pub async fn send_private_discord_update(&self, msg: String) {
        // async block to mimic try block
        let send_result: AppResult = async {
            crate::env::PRIVATE_UPDATES_CHANNEL_ID
                .say(self.try_discord()?, msg)
                .await?;
            Ok(())
        }
        .await;
        if let Err(err) = send_result {
            tracing::warn!(%err, "Error sending private Discord update");
        }
    }

    pub async fn alert_discord_to_verify(&self, solve_id: SolveId, updated: bool) {
        let send_result: AppResult = async {
            use poise::serenity_prelude::*;
            let discord = self.try_discord()?;
            let solve = self.get_solve(solve_id).await?;

            // send solve for verification
            let embed = CreateEmbed::new()
                .title(if updated {
                    "Updated solve"
                } else {
                    "New solve"
                })
                .url(format!("{}{}", *crate::env::DOMAIN_NAME, solve.url_path()));
            let embed = solve.embed_fields(embed);
            let builder = CreateMessage::new().embed(embed);

            crate::env::PRIVATE_UPDATES_CHANNEL_ID
                .send_message(discord.clone(), builder)
                .await?;

            Ok(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(?solve_id, %err, "Failed to alert discord to new solve");
        }
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

    msg.push("### 🏆 ")
        .push(solve.solver.md_link(false))
        .push(" set a ")
        .push(match wr_event.category {
            Category::Speed { .. } => {
                format!("{} speed", MdSolveTime(solve).md_link(false))
            }
            Category::Fmc { .. } => {
                format!("{} fewest-moves", MdSolveMoveCount(solve).md_link(false))
            }
        })
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
                    .push(MdSolveTime(old_wr).md_link(false))
                    .push(".");
            } else {
                msg.push(" the previous record of ")
                    .push(MdSolveTime(old_wr).md_link(false))
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
