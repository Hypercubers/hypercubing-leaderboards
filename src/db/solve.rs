#![allow(dead_code)]
use crate::db::program::{Program, ProgramVersion};
use crate::db::puzzle::Puzzle;
use crate::db::user::User;
use crate::AppState;
use chrono::{DateTime, Utc};
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
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,
    pub blind: bool,
    pub scramble_seed: Option<String>,
    pub program_version: ProgramVersion,
    pub speed_evidence: Option<SpeedEvidence>,
    pub valid_solve: Option<bool>,
    pub solver_notes: String,
    pub moderator_notes: String,
}

pub struct SpeedEvidence {
    pub id: i32,
    pub solve_id: i32,
    pub video_url: Option<String>,
    pub verified: Option<bool>,
    pub verified_by: i32,
    pub moderator_notes: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct PuzzleLeaderboard {
    pub id: i32,
    pub blind: Option<String>,
    pub no_filters: Option<String>,
    pub no_macros: Option<String>,
}

impl AppState {
    pub async fn get_leaderboard_puzzle(
        &self,
        leaderboard: PuzzleLeaderboard,
    ) -> sqlx::Result<Vec<Solve>> {
        Ok(query!(
            "SELECT * FROM (SELECT DISTINCT ON (Puzzle.leaderboard)
                    Solve.id,
                    Solve.log_file,
                    Solve.user_id,
                    Solve.upload_time,
                    Solve.puzzle_id,
                    Solve.move_count,
                    Solve.uses_macros,
                    Solve.uses_filters,
                    Solve.speed_cs,
                    Solve.memo_cs,
                    Solve.blind,
                    Solve.scramble_seed,
                    Solve.program_version_id,
                    Solve.speed_evidence_id,
                    Solve.valid_solve,
                    Solve.solver_notes,
                    Solve.moderator_notes as solve_moderator_notes,
                    Solve.rank,
                    UserAccount.email,
                    UserAccount.display_name,
                    UserAccount.moderator,
                    UserAccount.moderator_notes as user_moderator_notes,
                    UserAccount.dummy,
                    ProgramVersion.program_id,
                    ProgramVersion.version,
                    Program.name as program_name,
                    Program.abbreviation,  
                    Puzzle.hsc_id,
                    Puzzle.name as puzzle_name,
                    Puzzle.leaderboard,
                    SpeedEvidence.video_url,
                    SpeedEvidence.verified,
                    SpeedEvidence.verified_by,
                    SpeedEvidence.moderator_notes as evidence_moderator_notes
                FROM Solve
                JOIN UserAccount ON Solve.user_id = UserAccount.id
                JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
                JOIN Program ON ProgramVersion.program_id = Program.id
                JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
                JOIN SpeedEvidence ON SpeedEvidence.id = Solve.speed_evidence_id
                WHERE speed_cs IS NOT NULL
                    AND Puzzle.leaderboard = $1
                    AND Solve.blind = $2
                    AND (NOT (Solve.uses_filters AND $3))
                    AND (NOT (Solve.uses_macros AND $4))
                    AND SpeedEvidence.verified
                ORDER BY Puzzle.leaderboard, Solve.speed_cs ASC)
            ORDER BY speed_cs ASC
            ",
            leaderboard.id,
            leaderboard.blind.is_some(),
            leaderboard.no_filters.is_some(),
            leaderboard.no_macros.is_some()
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| Solve {
            id: row.id,
            log_file: row.log_file,
            user: User {
                id: row.user_id,
                email: row.email,
                display_name: row.display_name,
                moderator: row.moderator,
                moderator_notes: row.user_moderator_notes,
                dummy: row.dummy,
            },
            upload_time: row.upload_time,
            puzzle: Puzzle {
                id: row.puzzle_id,
                hsc_id: row.hsc_id,
                name: row.puzzle_name,
                leaderboard: row.leaderboard,
            },
            move_count: row.move_count,
            uses_macros: row.uses_macros,
            uses_filters: row.uses_filters,
            speed_cs: row.speed_cs,
            memo_cs: row.memo_cs,
            blind: row.blind,
            scramble_seed: row.scramble_seed,
            program_version: ProgramVersion {
                id: row.program_version_id,
                program: Program {
                    id: row.program_id,
                    name: row.program_name,
                    abbreviation: row.abbreviation,
                },
                version: row.version,
            },
            speed_evidence: Some(SpeedEvidence {
                id: row.speed_evidence_id.expect("must be verified"),
                solve_id: row.id,
                video_url: row.video_url,
                verified: row.verified,
                verified_by: row.verified_by,
                moderator_notes: row.evidence_moderator_notes,
            }),
            valid_solve: row.valid_solve,
            solver_notes: row.solver_notes,
            moderator_notes: row.solve_moderator_notes,
        })
        .collect())
    }

    pub async fn get_leaderboard_solver(
        &self,
        user_id: i32,
        blind: bool,
        no_filters: bool,
        no_macros: bool,
    ) -> sqlx::Result<Vec<Solve>> {
        Ok(query!(
            "SELECT DISTINCT ON (Solve.user_id)
                Solve.id,
                Solve.log_file,
                Solve.user_id,
                Solve.upload_time,
                Solve.puzzle_id,
                Solve.move_count,
                Solve.uses_macros,
                Solve.uses_filters,
                Solve.speed_cs,
                Solve.memo_cs,
                Solve.blind,
                Solve.scramble_seed,
                Solve.program_version_id,
                Solve.speed_evidence_id,
                Solve.valid_solve,
                Solve.solver_notes,
                Solve.moderator_notes as solve_moderator_notes,
                Solve.rank,
                UserAccount.email,
                UserAccount.display_name,
                UserAccount.moderator,
                UserAccount.moderator_notes as user_moderator_notes,
                UserAccount.dummy,
                ProgramVersion.program_id,
                ProgramVersion.version,
                Program.name as program_name,
                Program.abbreviation,  
                Puzzle.hsc_id,
                Puzzle.name as puzzle_name,
                Puzzle.leaderboard,
                SpeedEvidence.video_url,
                SpeedEvidence.verified,
                SpeedEvidence.verified_by,
                SpeedEvidence.moderator_notes as evidence_moderator_notes
            FROM Solve
            JOIN UserAccount ON Solve.user_id = UserAccount.id
            JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
            JOIN Program ON ProgramVersion.program_id = Program.id
            JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
            JOIN SpeedEvidence ON SpeedEvidence.id = Solve.speed_evidence_id
            WHERE speed_cs IS NOT NULL
                AND Solve.user_id = $1
                AND Solve.blind = $2
                AND (NOT (Solve.uses_filters AND $3))
                AND (NOT (Solve.uses_macros AND $4))
                AND SpeedEvidence.verified
            ORDER BY Solve.user_id, Solve.speed_cs ASC
            ",
            user_id,
            blind,
            no_filters,
            no_macros
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| Solve {
            id: row.id,
            log_file: row.log_file,
            user: User {
                id: row.user_id,
                email: row.email,
                display_name: row.display_name,
                moderator: row.moderator,
                moderator_notes: row.user_moderator_notes,
                dummy: row.dummy,
            },
            upload_time: row.upload_time,
            puzzle: Puzzle {
                id: row.puzzle_id,
                hsc_id: row.hsc_id,
                name: row.puzzle_name,
                leaderboard: row.leaderboard,
            },
            move_count: row.move_count,
            uses_macros: row.uses_macros,
            uses_filters: row.uses_filters,
            speed_cs: row.speed_cs,
            memo_cs: row.memo_cs,
            blind: row.blind,
            scramble_seed: row.scramble_seed,
            program_version: ProgramVersion {
                id: row.program_version_id,
                program: Program {
                    id: row.program_id,
                    name: row.program_name,
                    abbreviation: row.abbreviation,
                },
                version: row.version,
            },
            speed_evidence: Some(SpeedEvidence {
                id: row.speed_evidence_id.expect("must be verified"),
                solve_id: row.id,
                video_url: row.video_url,
                verified: row.verified,
                verified_by: row.verified_by,
                moderator_notes: row.evidence_moderator_notes,
            }),
            valid_solve: row.valid_solve,
            solver_notes: row.solver_notes,
            moderator_notes: row.solve_moderator_notes,
        })
        .collect())
    }

    pub async fn get_rank(
        &self,
        puzzle_id: i32,
        blind: bool,
        no_filters: bool,
        no_macros: bool,
        speed_cs: i32,
    ) -> sqlx::Result<i32> {
        Ok((query!(
            "SELECT COUNT(*) FROM (SELECT DISTINCT ON (user_id)
                user_id
            FROM Solve
            JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
            WHERE speed_cs IS NOT NULL
                AND Puzzle.leaderboard = $1
                AND blind = $2
                AND (NOT (uses_filters AND $3))
                AND (NOT (uses_macros AND $4))
                AND speed_cs < $5
            ORDER BY user_id, speed_cs ASC)
            ",
            puzzle_id,
            blind,
            no_filters,
            no_macros,
            speed_cs
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .expect("count should not be null")
            + 1) as i32)
    }
}
