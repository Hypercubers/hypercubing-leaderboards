use chrono::{DateTime, Utc};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

use crate::api::upload::{
    UpdateSolveCategory, UpdateSolveMoveCount, UpdateSolveProgramVersionId, UpdateSolveSpeedCs,
    UpdateSolveVideoUrl, UploadSolveExternal,
};
use crate::db::program::{Program, ProgramId, ProgramVersion, ProgramVersionId};
use crate::db::puzzle::{
    MdSpeedCategory, Puzzle, PuzzleCategory, PuzzleCategoryBase, PuzzleCategoryFlags, PuzzleId,
};
use crate::db::user::{PublicUser, User, UserId};
use crate::db::EditAuthorization;
use crate::error::MissingField;
use crate::traits::Linkable;
use crate::util::render_time;
use crate::AppState;

id_struct!(SolveId, "solve");

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

/// View of a solve with all relevant supplementary data.
#[derive(Serialize, Debug, Clone)]
pub struct FullSolve {
    pub id: SolveId,
    /// Whether `log_file` is non-NULL. `log_file` may be very big so we don't
    /// include it unless it's requested.
    pub has_log_file: bool,
    pub upload_time: DateTime<Utc>,
    pub move_count: Option<i32>,
    pub scramble_seed: Option<String>,
    pub log_file_verified: Option<bool>,
    pub log_file_verified_by: Option<UserId>,
    pub solver_notes: String,
    pub moderator_notes: String,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,
    pub video_url: Option<String>,
    pub speed_verified: Option<bool>,
    pub speed_verified_by: Option<UserId>,

    pub user: PublicUser,
    pub program_version: ProgramVersion,
    pub category: PuzzleCategory,
}
impl Linkable for FullSolve {
    fn relative_url(&self) -> String {
        format!("/solve?id={}", self.id.0)
    }

    fn md_text(&self) -> String {
        format!("solve #{}", self.id.0)
    }
}
impl FullSolve {
    pub fn try_from_opt(optional_solve: Option<InlinedSolve>) -> sqlx::Result<Option<FullSolve>> {
        match optional_solve {
            Some(solve) => Self::try_from(solve).map(Some),
            None => Ok(None),
        }
    }
}
impl TryFrom<InlinedSolve> for FullSolve {
    type Error = sqlx::Error;

    fn try_from(solve: InlinedSolve) -> Result<Self, Self::Error> {
        let InlinedSolve {
            id,
            has_log_file,
            user_id,
            upload_time,
            puzzle_id,
            move_count,
            uses_macros,
            uses_filters,
            computer_assisted,
            blind,
            scramble_seed,
            program_version_id,
            log_file_verified,
            log_file_verified_by,
            solver_notes,
            moderator_notes,
            user_display_name,
            program_id,
            program_version,
            program_name,
            program_abbreviation,
            puzzle_name,
            puzzle_primary_filters,
            puzzle_primary_macros,
            speed_cs,
            memo_cs,
            video_url,
            speed_verified,
            speed_verified_by,
            rank: _,
        } = solve;

        // IIFE to mimic try_block
        (|| {
            Ok(Self {
                id: id.map(SolveId).ok_or("id")?,
                has_log_file: has_log_file.ok_or("has_log_file")?,
                upload_time: upload_time.ok_or("upload_time")?,
                move_count,
                scramble_seed,
                log_file_verified,
                log_file_verified_by: log_file_verified_by.map(UserId),
                solver_notes: solver_notes.ok_or("solver_notes")?,
                moderator_notes: moderator_notes.ok_or("moderator_notes")?,
                speed_cs,
                memo_cs,
                video_url,
                speed_verified,
                speed_verified_by: speed_verified_by.map(UserId),

                user: PublicUser {
                    id: user_id.map(UserId).ok_or("user_id")?,
                    display_name: user_display_name,
                },
                program_version: ProgramVersion {
                    id: program_version_id
                        .map(ProgramVersionId)
                        .ok_or("program_version_id")?,
                    program: Program {
                        id: program_id.map(ProgramId).ok_or("program_id")?,
                        name: program_name.ok_or("program_name")?,
                        abbreviation: program_abbreviation.ok_or("program_abbreviation")?,
                    },
                    version: program_version,
                },
                category: PuzzleCategory {
                    base: PuzzleCategoryBase {
                        puzzle: Puzzle {
                            id: puzzle_id.map(PuzzleId).ok_or("puzzle_id")?,
                            name: puzzle_name.ok_or("puzzle_name")?,
                            primary_flags: PuzzleCategoryFlags {
                                uses_filters: puzzle_primary_filters
                                    .ok_or("puzzle_primary_filters")?,
                                uses_macros: puzzle_primary_macros
                                    .ok_or("puzzle_primary_macros")?,
                                computer_assisted: false,
                            },
                        },
                        blind: blind.ok_or("blind")?,
                    },
                    flags: PuzzleCategoryFlags {
                        uses_macros: uses_macros.ok_or("uses_macros")?,
                        uses_filters: uses_filters.ok_or("uses_filters")?,
                        computer_assisted: computer_assisted.ok_or("computer_assisted")?,
                    },
                },
            })
        })()
        .map_err(MissingField::new_sqlx_error)
    }
}

pub struct RankedFullSolve {
    pub rank: i64,
    pub solve: FullSolve,
}
impl RankedFullSolve {
    pub fn try_from_opt(
        optional_solve: Option<InlinedSolve>,
    ) -> sqlx::Result<Option<RankedFullSolve>> {
        match optional_solve {
            Some(solve) => Self::try_from(solve).map(Some),
            None => Ok(None),
        }
    }
}
impl TryFrom<InlinedSolve> for RankedFullSolve {
    type Error = sqlx::Error;

    fn try_from(solve: InlinedSolve) -> Result<Self, Self::Error> {
        Ok(Self {
            rank: solve.rank.0.ok_or(MissingField::new_sqlx_error("rank"))?,
            solve: FullSolve::try_from(solve)?,
        })
    }
}

/// View of a solve with all relevant data inlined.
///
/// This is not stored in the database; it is constructed from a [`Solve`].
#[derive(Serialize, Debug, Clone)]
pub struct InlinedSolve {
    pub id: Option<i32>,
    pub has_log_file: Option<bool>,
    pub user_id: Option<i32>,
    pub upload_time: Option<DateTime<Utc>>,
    pub puzzle_id: Option<i32>,
    pub move_count: Option<i32>,
    pub uses_macros: Option<bool>,
    pub uses_filters: Option<bool>,
    pub computer_assisted: Option<bool>,
    pub blind: Option<bool>,
    pub scramble_seed: Option<String>,
    pub program_version_id: Option<i32>,
    pub log_file_verified: Option<bool>,
    pub log_file_verified_by: Option<i32>,
    pub solver_notes: Option<String>,
    pub moderator_notes: Option<String>,

    pub user_display_name: Option<String>,
    pub program_id: Option<i32>,
    pub program_version: Option<String>,
    pub program_name: Option<String>,
    pub program_abbreviation: Option<String>,
    pub puzzle_name: Option<String>,
    pub puzzle_primary_filters: Option<bool>,
    pub puzzle_primary_macros: Option<bool>,

    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,
    pub video_url: Option<String>,
    pub speed_verified: Option<bool>,
    pub speed_verified_by: Option<i32>,

    pub rank: SolveRank,
}

#[derive(Serialize, Debug, Clone)]
pub struct SolveRank(Option<i64>);
impl From<Option<String>> for SolveRank {
    fn from(_value: Option<String>) -> Self {
        Self(None)
    }
}
impl From<Option<i64>> for SolveRank {
    fn from(value: Option<i64>) -> Self {
        Self(value)
    }
}

impl FullSolve {
    /// Returns the puzzle that was solved.
    pub fn puzzle(&self) -> &Puzzle {
        &self.category.base.puzzle
    }
    /// Returns whether the solve was a blindsolve.
    pub fn blind(&self) -> bool {
        self.category.base.blind
    }
    /// Returns the flags for the solve.
    pub fn flags(&self) -> PuzzleCategoryFlags {
        self.category.flags
    }

    /// Returns the Discord embed fields for the solve.
    pub fn embed_fields(
        &self,
        mut embed: serenity::all::CreateEmbed,
    ) -> serenity::all::CreateEmbed {
        embed = embed.field("Solve ID", self.id.0.to_string(), true);

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

        embed = embed.field("Solver", self.user.name(), true).field(
            "Puzzle",
            self.category.base.name() + &self.flags().emoji_string(),
            true,
        );

        if let Some(move_count) = self.move_count {
            embed = embed.field("Move count", move_count.to_string(), true);
        }
        embed = embed.field("Program", self.program_version.name(), true);

        if !self.solver_notes.is_empty() {
            embed = embed.field("Solver notes", self.solver_notes.clone(), true);
        }

        embed
    }

    /// Returns the key by which to sort solves in speed leaderboards.
    pub fn speed_sort_key(&self) -> impl Ord {
        // Sort by speed first and use move count and upload time as tiebreakers.
        (
            self.speed_cs.is_none(),
            self.speed_cs,
            self.move_count.is_none(),
            self.move_count,
            self.upload_time,
        )
    }
    /// Returns the key by which to sort solves in speed leaderboards.
    pub fn fmc_sort_key(&self) -> impl Ord {
        // Sort by move count and use upload time as a tiebreaker; ignore speed.
        (self.move_count.is_none(), self.move_count, self.upload_time)
    }

    pub fn url_path(&self) -> String {
        format!("/solve?id={}", self.id.0)
    }

    /// Returns whether a user is allowed to edit the solve.
    pub fn can_edit(&self, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if self.user.id == editor.id
            && self.log_file_verified != Some(true)
            && self.speed_verified != Some(true)
        {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }

    /// Helper method for `editor.and_then(|editor| self.can_edit(editor))`.
    pub fn can_edit_opt(&self, editor: Option<&User>) -> Option<EditAuthorization> {
        editor.and_then(|editor| self.can_edit(editor))
    }
}

impl AppState {
    pub async fn get_solve(&self, id: SolveId) -> sqlx::Result<Option<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT *, NULL as rank FROM InlinedSolve WHERE id = $1",
            id.0,
        )
        .fetch_optional(&self.pool)
        .await
        .and_then(FullSolve::try_from_opt)
    }

    pub async fn get_puzzle_speed_leaderboard(
        &self,
        puzzle_category: &PuzzleCategory,
    ) -> sqlx::Result<Vec<RankedFullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT *, RANK () OVER (ORDER BY speed_cs) AS rank FROM (
                SELECT DISTINCT ON (user_id) *
                    FROM VerifiedSpeedSolve
                    WHERE puzzle_id = $1
                        AND blind = $2
                        AND uses_filters <= $3
                        AND uses_macros <= $4
                        AND computer_assisted <= $5
                    ORDER BY
                        user_id,
                        speed_cs ASC NULLS LAST, upload_time
            ) as s
            ",
            puzzle_category.base.puzzle.id.0,
            puzzle_category.base.blind,
            puzzle_category.flags.uses_filters,
            puzzle_category.flags.uses_macros,
            puzzle_category.flags.computer_assisted,
        )
        .try_map(RankedFullSolve::try_from)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_all_speed_records(&self) -> sqlx::Result<Vec<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT DISTINCT ON (puzzle_id, blind, uses_filters, uses_macros)
                *, NULL as rank
                FROM VerifiedSpeedSolve
                ORDER BY
                    puzzle_id, blind, uses_filters, uses_macros,
                    speed_cs ASC NULLS LAST, upload_time
            ",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(FullSolve::try_from)
        .collect()
    }

    pub async fn get_puzzle_speed_records(
        &self,
        puzzle_id: PuzzleId,
    ) -> sqlx::Result<Vec<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT DISTINCT ON (blind, uses_filters, uses_macros)
                *, NULL as rank
                FROM VerifiedSpeedSolve
                WHERE puzzle_id = $1
                ORDER BY
                    blind, uses_filters, uses_macros,
                    speed_cs ASC NULLS LAST, upload_time
            ",
            puzzle_id.0
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(FullSolve::try_from)
        .collect()
    }

    pub async fn get_solver_speed_pbs(
        &self,
        user_id: UserId,
    ) -> sqlx::Result<Vec<RankedFullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT * FROM (
                SELECT
                    *,
                    RANK () OVER (PARTITION BY (puzzle_id, blind) ORDER BY speed_cs) AS rank
                    FROM (
                        SELECT DISTINCT ON (user_id, puzzle_id)
                            *
                            FROM VerifiedSpeedSolveInPrimaryCategory
                            ORDER BY user_id, puzzle_id, speed_cs
                    ) AS s
                ) AS ss
                WHERE user_id = $1
            ",
            user_id.0,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(RankedFullSolve::try_from)
        .collect()
    }

    pub async fn get_rank(
        &self,
        puzzle_category: &PuzzleCategory,
        solve: &FullSolve,
    ) -> sqlx::Result<Option<i64>> {
        Ok(query!(
            "SELECT rank FROM (
                SELECT
                    id,
                    RANK () OVER (PARTITION BY (puzzle_id, blind) ORDER BY speed_cs) AS rank
                    FROM (
                        SELECT DISTINCT ON (user_id, puzzle_id) *
                            FROM VerifiedSpeedSolve
                            WHERE puzzle_id = $1
                                AND blind = $2
                                AND uses_filters <= $3
                                AND uses_macros <= $4
                            ORDER BY
                                user_id, puzzle_id,
                                speed_cs
                    ) AS s
                ) AS ss
                WHERE id = $5
            ",
            puzzle_category.base.puzzle.id.0,
            puzzle_category.base.blind,
            puzzle_category.flags.uses_filters,
            puzzle_category.flags.uses_macros,
            solve.id.0,
        )
        .fetch_one(&self.pool)
        .await?
        .rank)
    }

    /// Returns the world record solve in a category, excluding the given solve
    /// (or `None` if there are no other solves in the category).
    pub async fn world_record_speed_solve_excluding(
        &self,
        category: &PuzzleCategory,
        excluding_solve: &FullSolve,
    ) -> sqlx::Result<Option<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT *, NULL as rank
                FROM VerifiedSpeedSolve
                WHERE puzzle_id = $1
                    AND blind = $2
                    AND uses_filters <= $3
                    AND uses_macros <= $4
                    AND id <> $5
                ORDER BY speed_cs, upload_time
                LIMIT 1
            ",
            category.base.puzzle.id.0,
            category.base.blind,
            category.flags.uses_filters,
            category.flags.uses_macros,
            excluding_solve.id.0,
        )
        .try_map(FullSolve::try_from)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn alert_discord_to_verify(&self, solve_id: SolveId, updated: bool) {
        let send_result: Result<(), Box<dyn std::error::Error>> = async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self.get_solve(solve_id).await?.ok_or("no solve")?;

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
            tracing::warn!(?solve_id, err, "failed to alert discord to new solve");
        }
    }

    pub async fn add_solve_external(
        &self,
        user_id: UserId,
        item: UploadSolveExternal,
    ) -> sqlx::Result<SolveId> {
        let solve_id = query!(
            "INSERT INTO Solve
                    (log_file, user_id, puzzle_id, move_count,
                    uses_macros, uses_filters, computer_assisted,
                    blind, program_version_id,
                    speed_cs, memo_cs, video_url)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING id
            ",
            item.log_file,
            user_id.0,
            item.puzzle_id,
            item.move_count,
            item.uses_macros,
            item.uses_filters,
            item.computer_assisted,
            item.blind,
            item.program_version_id,
            item.speed_cs,
            if item.blind { item.memo_cs } else { None },
            item.video_url,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        let solve_id = SolveId(solve_id);
        self.alert_discord_to_verify(solve_id, false).await;

        tracing::info!(?user_id, ?solve_id, "uploaded external solve");

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

    pub async fn verify_speed(
        &self,
        id: SolveId,
        mod_id: UserId,
        verified: bool,
    ) -> sqlx::Result<Option<()>> {
        let solve_id = query!(
            "UPDATE Solve
                SET
                    speed_verified_by = $2,
                    speed_verified = $3
                WHERE id = $1
                RETURNING id
            ",
            id.0,
            mod_id.0,
            verified,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.id);

        let Some(solve_id) = solve_id else {
            return Ok(None);
        };
        let solve_id = SolveId(solve_id);

        tracing::info!(?mod_id, ?solve_id, "uploaded external solve");

        if verified {
            self.alert_discord_to_speed_record(solve_id).await;
        }

        Ok(Some(()))
    }

    pub async fn alert_discord_to_speed_record(&self, solve_id: SolveId) {
        // async block to mimic try block
        let send_result = async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;

            let solve = self.get_solve(solve_id).await?.ok_or("no solve")?;

            let primary_category = solve.puzzle().primary_category();
            let mut wr_category = None;
            let mut displaced_wr = None;

            // Prefer reporting for the primary category
            if solve.category.counts_for_primary_category() {
                if let Some(old_wr) = self
                    .world_record_speed_solve_excluding(&primary_category, &solve)
                    .await?
                {
                    if solve.speed_cs <= old_wr.speed_cs {
                        wr_category = Some(&primary_category);
                        displaced_wr = Some(old_wr);
                    }
                } else {
                    wr_category = Some(&primary_category);
                }
            }
            // If it's not a WR in the primary category, try reporting for its
            // own category
            if wr_category.is_none() {
                if let Some(old_wr) = self
                    .world_record_speed_solve_excluding(&solve.category, &solve)
                    .await?
                {
                    if solve.speed_cs <= old_wr.speed_cs {
                        wr_category = Some(&solve.category);
                        displaced_wr = Some(old_wr);
                    }
                } else {
                    wr_category = Some(&solve.category);
                }
            }

            let Some(wr_category) = wr_category else {
                return Ok(()); // not a world record; nothing to report
            };

            let mut msg = MessageBuilder::new();

            msg.push("### 🏆 ")
                .push(solve.user.md_link(false))
                .push(" set a ")
                .push(MdSolveTime(&solve).md_link(false))
                .push(" speed record for ")
                .push(MdSpeedCategory(wr_category).md_link(false))
                .push_line("!");

            match displaced_wr {
                None => {
                    msg.push_line("This is the first solve in the category! 🎉");
                }
                Some(old_wr) => {
                    match old_wr.speed_cs == solve.speed_cs {
                        true => msg.push("They have tied"),
                        false => msg.push("They have defeated"),
                    };
                    if old_wr.user.id == solve.user.id {
                        msg.push(" their previous record of ")
                            .push(MdSolveTime(&old_wr).md_link(false))
                            .push(".");
                    } else {
                        msg.push(" the previous record of ")
                            .push(MdSolveTime(&old_wr).md_link(false))
                            .push(" by ")
                            .push(old_wr.user.md_link(false))
                            .push(".");
                    }
                }
            }

            let channel = ChannelId::new(dotenvy::var("UPDATE_CHANNEL_ID")?.parse()?);
            channel.say(discord, msg.build()).await?;

            Ok::<_, Box<dyn std::error::Error>>(())
        }
        .await;

        if let Err(err) = send_result {
            tracing::warn!(?solve_id, err, "failed to alert discord to new record");
        }
    }
}
