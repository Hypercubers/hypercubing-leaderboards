#![allow(dead_code)]
use crate::api::upload::UploadSolveExternal;
use crate::db::program::{Program, ProgramVersion};
use crate::db::puzzle::Puzzle;
use crate::db::user::User;
use crate::AppState;
use chrono::{DateTime, Utc};
use sqlx::Connection;
use sqlx::{query, query_as};

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

pub fn format_modifiers(blind: bool, uses_macros: bool, uses_filters: bool) -> String {
    let mut name = "".to_string();
    if blind {
        name += "🙈";
    }
    if uses_filters {
        name += "⚗️";
    }
    if uses_macros {
        name += "👾";
    }
    name
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
    pub fn user_html_name(&self) -> String {
        User::make_html_name(&self.display_name, self.id)
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
                    format!(
                        "{} ({})",
                        crate::html::boards::render_time(speed_cs),
                        crate::html::boards::render_time(memo_cs)
                    ),
                    true,
                );
            } else {
                embed = embed.field("Time", crate::html::boards::render_time(speed_cs), true);
            }
        }

        if let Some(video_url) = &self.video_url {
            embed = embed.field("Video URL", video_url.to_string(), true);
        }

        embed = embed.field("Solver", self.user_html_name(), true).field(
            "Puzzle",
            self.puzzle_name.clone()
                + &format_modifiers(self.blind, self.uses_filters, self.uses_macros),
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
        id: i32,
        blind: bool,
        uses_filters: bool,
        uses_macros: bool,
    ) -> sqlx::Result<Vec<LeaderboardSolve>> {
        Ok(query!(
            "SELECT DISTINCT ON (user_id) *
                FROM LeaderboardSolve
                WHERE speed_cs IS NOT NULL
                    AND puzzle_id = $1
                    AND blind = $2
                    AND (NOT (uses_filters AND $3))
                    AND (NOT (uses_macros AND $4))
                    AND valid_solve
                ORDER BY user_id, speed_cs ASC
            ",
            id,
            blind,
            !uses_filters,
            !uses_macros
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| make_leaderboard_solve!(row))
        .collect())
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
                ORDER BY puzzle_id, uses_filters, uses_macros, speed_cs ASC NULLS LAST
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
        puzzle_id: i32,
        blind: bool,
        uses_filters: bool,
        uses_macros: bool,
        speed_cs: i32,
    ) -> sqlx::Result<i32> {
        // TODO: replace with RANK()
        Ok((query!(
            "SELECT COUNT(*) FROM (SELECT DISTINCT ON (user_id) *
                FROM LeaderboardSolve
                WHERE speed_cs IS NOT NULL
                    AND puzzle_id = $1
                    AND blind = $2
                    AND (NOT (uses_filters AND $3))
                    AND (NOT (uses_macros AND $4))
                    AND speed_cs < $5
                ORDER BY user_id, speed_cs ASC
            )
            ",
            puzzle_id,
            blind,
            !uses_filters,
            !uses_macros,
            speed_cs
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .expect("count should not be null")
            + 1) as i32)
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

                    Ok::<i32, sqlx::Error>(solve_id)
                })
            })
            .await?;

        // IIFE to mimic try_block
        let send_result: Result<_, Box<dyn std::error::Error>> = (|| async {
            use poise::serenity_prelude::*;
            let discord = self.discord.clone().ok_or("no discord")?;
            let solve = self
                .get_leaderboard_solve(dbg!(solve_id))
                .await?
                .ok_or("no solve")?;

            // send solve for verification
            let embed = CreateEmbed::new().title("New speedsolve");
            let embed = solve.embed_fields(embed);
            let builder = CreateMessage::new().embed(embed);

            let channel = ChannelId::new(dotenvy::var("VERIFICATION_CHANNEL_ID")?.parse()?);
            channel.send_message(discord, builder).await?;
            Ok(())
        })()
        .await;

        if let Err(err) = send_result {
            println!("{:?}", err);
        }

        Ok(solve_id)
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
}
