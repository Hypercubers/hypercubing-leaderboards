#![allow(dead_code)]
use crate::api::upload::{UpdateSolveVideoUrl, UploadSolveExternal};
use crate::db::program::{Program, ProgramVersion};
use crate::db::puzzle::Puzzle;
use crate::db::puzzle::PuzzleCategory;
use crate::db::puzzle::PuzzleCategoryBase;
use crate::db::puzzle::PuzzleCategoryFlags;
use crate::db::user::PublicUser;
use crate::db::user::User;
use crate::util::render_time;
use crate::AppState;
use chrono::{DateTime, Utc};
use sqlx::Connection;
use sqlx::{query, query_as};
use std::collections::HashSet;

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
    pub valid_log_file: Option<bool>,
    pub solver_notes: String,
    pub moderator_notes: String,
}

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

#[derive(Clone)]
pub struct LeaderboardSolve {
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
    pub speed_evidence_id: Option<i32>,
    pub valid_log_file: Option<bool>,
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
    pub verified: Option<bool>,
    pub valid_solve: bool,
    pub rank: Option<i32>,
}

macro_rules! make_leaderboard_solve {
    ( $row:expr ) => {
        LeaderboardSolve {
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
            speed_evidence_id: $row.speed_evidence_id,
            valid_log_file: $row.valid_log_file,
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
            verified: $row.verified,
            valid_solve: $row.valid_solve.expect("column valid_solve not null"),
            rank: None,
        }
    };
}

impl LeaderboardSolve {
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

        if let Some(speed_evidence_id) = self.speed_evidence_id {
            embed = embed.field("Speed evidence ID", speed_evidence_id.to_string(), true);
        }

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

        if self.solver_notes.len() > 0 {
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
}

impl AppState {
    pub async fn get_leaderboard_solve(&self, id: i32) -> sqlx::Result<Option<LeaderboardSolve>> {
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
    ) -> sqlx::Result<Vec<LeaderboardSolve>> {
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
                            AND valid_solve
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
        solves.sort_by_key(LeaderboardSolve::sort_key);
        solves.sort_by_key(|solve| solve.user_id);
        solves.dedup_by_key(|solve| solve.user_id);
        solves.sort_by_key(LeaderboardSolve::sort_key);

        Ok(solves)
    }

    pub async fn get_leaderboard_solver(
        &self,
        user_id: i32,
    ) -> sqlx::Result<Vec<LeaderboardSolve>> {
        Ok(query!(
            "SELECT DISTINCT ON (puzzle_id, uses_filters, uses_macros) *
                FROM LeaderboardSolve
                WHERE user_id = $1
                    AND valid_solve
                ORDER BY puzzle_id, uses_filters, uses_macros, speed_cs ASC NULLS LAST, upload_time
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
        speed_cs: Option<i32>,
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
                        AND ((speed_cs < $5) IS TRUE OR ($5 IS NULL AND NOT (speed_cs IS NULL)))
                    ORDER BY user_id, speed_cs ASC NULLS LAST, upload_time
                    ",
                    puzzle_category.base.puzzle.id,
                    puzzle_category.base.blind,
                    puzzle_category.flags.uses_filters,
                    puzzle_category.flags.uses_macros,
                    speed_cs
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

    pub async fn is_record(&self, solve: &LeaderboardSolve) -> sqlx::Result<bool> {
        let count = query!(
            "SELECT COUNT(*) FROM (SELECT DISTINCT ON (user_id) *
                FROM LeaderboardSolve
                WHERE puzzle_id = $1
                    AND blind = $2
                    AND uses_filters = $3
                    AND uses_macros = $4
                    AND ((speed_cs <= $5) IS TRUE OR ($5 IS NULL))
                ORDER BY user_id
                LIMIT 2
            )
            ",
            solve.puzzle_id,
            solve.blind,
            solve.uses_filters,
            solve.uses_macros,
            solve.speed_cs
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .expect("count cannot be null");

        Ok(count == 1)
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
                                blind, program_version_id) 
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                            RETURNING id",
                        item.log_file,
                        user_id,
                        item.puzzle_id,
                        item.move_count,
                        item.uses_macros,
                        item.uses_filters,
                        item.blind,
                        item.program_version_id,
                    )
                    .fetch_optional(&mut **txn)
                    .await?
                    .expect("upload should work")
                    .id;

                    if item.speed_cs.is_some() || item.video_url.is_some() {
                        let speed_evidence_id = query!(
                            "INSERT INTO SpeedEvidence
                                (solve_id, speed_cs, memo_cs, video_url) 
                            VALUES ($1, $2, $3, $4)
                            RETURNING id",
                            solve_id,
                            item.speed_cs,
                            if item.blind { item.memo_cs } else { None },
                            item.video_url
                        )
                        .fetch_optional(&mut **txn)
                        .await?
                        .expect("upload should work")
                        .id;

                        query!(
                            "UPDATE Solve
                            SET speed_evidence_id = $1
                            WHERE id = $2",
                            speed_evidence_id,
                            solve_id
                        )
                        .execute(&mut **txn)
                        .await?;
                    }

                    Ok::<i32, sqlx::Error>(solve_id)
                })
            })
            .await?;

        // IIFE to mimic try_block
        let _send_result = (|| async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self
                .get_leaderboard_solve(solve_id)
                .await?
                .ok_or("no solve")?;

            // send solve for verification
            let embed = CreateEmbed::new().title("New speedsolve").url(format!(
                "{}{}",
                dotenvy::var("DOMAIN_NAME")?,
                solve.url_path()
            ));
            let embed = solve.embed_fields(embed);
            let builder = CreateMessage::new().embed(embed);

            let channel = ChannelId::new(dotenvy::var("VERIFICATION_CHANNEL_ID")?.parse()?);
            channel.send_message(discord.clone(), builder).await?;
            Ok::<_, Box<dyn std::error::Error>>(())
        })()
        .await;

        /*if let Err(err) = send_result {
            println!("{:?}", err);
        }*/

        Ok(solve_id)
    }

    pub async fn update_video_url(&self, item: UpdateSolveVideoUrl) -> sqlx::Result<()> {
        query!(
            "UPDATE SpeedEvidence
                SET video_url = $1
                FROM Solve
                WHERE SpeedEvidence.id = Solve.speed_evidence_id
                AND Solve.id = $2",
            item.video_url,
            item.solve_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_speed_evidence_primary(
        &self,
        solve_id: i32,
        video_url: String,
    ) -> sqlx::Result<()> {
        query!(
            "INSERT INTO SpeedEvidence
                    (solve_id, video_url) 
                VALUES ($1, $2)
                RETURNING *",
            solve_id,
            Some(video_url)
        )
        .fetch_optional(&self.pool)
        .await?
        .expect("upload should work");
        Ok(())
    }

    pub async fn verify_speed_evidence(
        &self,
        id: i32,
        verified: bool,
        mod_id: i32,
    ) -> sqlx::Result<Option<()>> {
        let solve_id = query!(
            "UPDATE SpeedEvidence
                SET verified = $2, verified_by = $3
                WHERE id = $1
                RETURNING solve_id
            ",
            id,
            verified,
            mod_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|r| r.solve_id);

        let Some(solve_id) = solve_id else {
            return Ok(None);
        };

        // IIFE to mimic try_block
        let _send_result = (|| async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self
                .get_leaderboard_solve(solve_id)
                .await?
                .ok_or("no solve")?;

            if self.is_record(&solve).await? {
                let mut builder = MessageBuilder::new();
                builder
                    .push("🏆")
                    .push_bold_safe(solve.user().name())
                    .push(" has gotten a record on ")
                    .push_bold_safe(solve.puzzle_category().base.name());

                builder.push(solve.puzzle_category().flags.format_modifiers());
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
        })()
        .await;

        Ok(Some(()))
    }
}
