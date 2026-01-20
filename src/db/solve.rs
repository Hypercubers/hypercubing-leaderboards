use std::fmt;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Postgres, QueryBuilder, Row, query, query_as, query_scalar};

use super::*;
use crate::AppState;
use crate::db::EventClass;
use crate::db::audit_log_event::AuditLogEvent;
use crate::error::{AppError, AppResult, MissingField};
use crate::traits::Linkable;

macro_rules! fetch_log_fields_for_solve {
    ($transaction:expr, $solve_id:expr) => {
        query!(
            "SELECT
                    solver_id, solve_date, upload_date, solver_notes, moderator_notes,
                    puzzle_id, variant_id, program_id,
                    average, blind, filters, macros, one_handed, computer_assisted,
                    move_count, speed_cs, memo_cs,
                    log_file_name, video_url
                FROM Solve
                WHERE id = $1
            ",
            $solve_id.0,
        )
        .fetch_one($transaction)
    };
}

id_struct!(SolveId, "solve");
impl Linkable for SolveId {
    fn relative_url(&self) -> String {
        format!("/solve?id={}", self.0)
    }

    fn md_text(&self) -> String {
        format!("solve #{}", self.0)
    }
}

#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SolveFlags {
    pub average: bool,
    pub blind: bool,
    pub filters: bool,
    pub macros: bool,
    pub one_handed: bool,
    pub computer_assisted: bool,
}
impl fmt::Display for SolveFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,
        } = self;

        let s = [
            ("average", *average),
            ("blind", *blind),
            ("filters", *filters),
            ("macros", *macros),
            ("one-handed", *one_handed),
            ("computer assisted", *computer_assisted),
        ]
        .into_iter()
        .filter(|&(_, b)| b)
        .map(|(s, _)| s)
        .join(", ");

        write!(f, "{s}")
    }
}

/// View of a solve with all relevant supplementary data.
#[derive(serde::Serialize, Debug, Clone)]
pub struct FullSolve {
    pub id: SolveId,

    // Metadata
    pub solve_date: DateTime<Utc>,
    pub upload_date: DateTime<Utc>,
    pub solver_notes: Option<String>,
    pub moderator_notes: Option<String>,
    pub auto_verify_output: Option<serde_json::Value>,

    // Event
    pub puzzle: Puzzle,
    pub variant: Option<Variant>,
    pub flags: SolveFlags,
    pub program: Program,

    // Score
    pub move_count: Option<i32>,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,

    // Verification
    pub fmc_verified: Option<bool>,
    pub fmc_verified_by: Option<UserId>,
    pub speed_verified: Option<bool>,
    pub speed_verified_by: Option<UserId>,

    // Evidence
    /// Name of the log file, if there is one. `log_file` may be very big so we
    /// don't include it unless it's requested.
    pub log_file_name: Option<String>,
    pub scramble_seed: Option<String>,
    pub video_url: Option<String>,

    // Solver
    pub solver: PublicUser,
}
impl Linkable for FullSolve {
    fn relative_url(&self) -> String {
        self.id.relative_url()
    }

    fn md_text(&self) -> String {
        self.id.md_text()
    }
}
impl TryFrom<InlinedSolve> for FullSolve {
    type Error = sqlx::Error;

    fn try_from(solve: InlinedSolve) -> Result<Self, Self::Error> {
        let InlinedSolve {
            id,

            solve_date,
            upload_date,
            solver_notes,
            moderator_notes,
            auto_verify_output,

            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,

            move_count,
            speed_cs,
            memo_cs,

            fmc_verified,
            fmc_verified_by,
            speed_verified,
            speed_verified_by,

            log_file_name,
            scramble_seed,
            video_url,

            puzzle_id,
            puzzle_name,
            puzzle_primary_filters,
            puzzle_primary_macros,
            puzzle_hsc_id,
            puzzle_autoverifiable,

            variant_id,
            variant_name,
            variant_prefix,
            variant_suffix,
            variant_abbr,
            variant_material_by_default,
            variant_primary_filters,
            variant_primary_macros,

            primary_filters: _,
            primary_macros: _,

            program_id,
            program_name,
            program_abbr,
            program_material,

            solver_id,
            solver_name,
        } = solve;

        // IIFE to mimic try_block
        (|| {
            Ok(Self {
                id: id.map(SolveId).ok_or("id")?,

                solve_date: solve_date.ok_or("solve_date")?,
                upload_date: upload_date.ok_or("upload_date")?,
                solver_notes,
                moderator_notes,
                auto_verify_output,

                puzzle: Puzzle {
                    id: PuzzleId(puzzle_id.ok_or("puzzle_id")?),
                    name: puzzle_name.ok_or("puzzle_name")?,
                    primary_filters: puzzle_primary_filters.ok_or("puzzle_primary_filters")?,
                    primary_macros: puzzle_primary_macros.ok_or("puzzle_primary_macros")?,
                    hsc_id: puzzle_hsc_id,
                    autoverifiable: puzzle_autoverifiable.ok_or("puzzle_autoverifiable")?,
                },
                variant: (|| {
                    Some(Variant {
                        id: VariantId(variant_id?),
                        name: variant_name?,
                        prefix: variant_prefix?,
                        suffix: variant_suffix?,
                        abbr: variant_abbr?,
                        material_by_default: variant_material_by_default?,
                        primary_filters: variant_primary_filters?,
                        primary_macros: variant_primary_macros?,
                    })
                })(),
                flags: SolveFlags {
                    average: average.unwrap_or(false),
                    blind: blind.unwrap_or(false),
                    filters: filters.unwrap_or(false),
                    macros: macros.unwrap_or(false),
                    one_handed: one_handed.unwrap_or(false),
                    computer_assisted: computer_assisted.unwrap_or(false),
                },
                program: Program {
                    id: ProgramId(program_id.ok_or("program_id")?),
                    name: program_name.ok_or("program_name")?,
                    abbr: program_abbr.ok_or("program_abbr")?,
                    material: program_material.ok_or("program_material")?,
                },

                move_count,
                speed_cs,
                memo_cs,

                fmc_verified,
                fmc_verified_by: fmc_verified_by.map(UserId),
                speed_verified,
                speed_verified_by: speed_verified_by.map(UserId),

                log_file_name,
                scramble_seed,
                video_url,

                solver: PublicUser {
                    id: UserId(solver_id.ok_or("solver_id")?),
                    name: solver_name,
                },
            })
        })()
        .map_err(MissingField::new_sqlx_error)
    }
}
impl<'r> FromRow<'r, PgRow> for FullSolve {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        InlinedSolve::from_row(row).and_then(Self::try_from)
    }
}
impl FullSolve {
    pub fn markdown_with_puzzle_and_solver_name(&self) -> String {
        format!(
            "{} of {} by {}",
            self.md_link(false),
            self.puzzle.md_link(false),
            self.solver.md_link(false),
        )
    }

    /// Returns whether the solve is pending review for speed or FMC.
    pub fn pending_review(&self) -> bool {
        self.speed_cs.is_some() && self.speed_verified.is_none()
            || self.move_count.is_some() && self.fmc_verified.is_none()
    }
}

/// View of a solve with all relevant supplementary data, plus its rank.
#[derive(Debug)]
pub struct RankedFullSolve {
    pub rank: i64,
    pub solve: FullSolve,
}
impl<'r> FromRow<'r, PgRow> for RankedFullSolve {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            rank: row.try_get("rank")?,
            solve: FullSolve::from_row(row)?,
        })
    }
}

/// View of a solve with all relevant data inlined.
#[derive(serde::Serialize, sqlx::FromRow, Debug, Default, Clone)]
pub struct InlinedSolve {
    pub id: Option<i32>,

    // Metadata
    pub solve_date: Option<DateTime<Utc>>,
    pub upload_date: Option<DateTime<Utc>>,
    pub solver_notes: Option<String>,
    pub moderator_notes: Option<String>,
    pub auto_verify_output: Option<serde_json::Value>,

    // Flags
    pub average: Option<bool>,
    pub blind: Option<bool>,
    pub filters: Option<bool>,
    pub macros: Option<bool>,
    pub one_handed: Option<bool>,
    pub computer_assisted: Option<bool>,

    // Score
    pub move_count: Option<i32>,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,

    // Verification
    pub fmc_verified: Option<bool>,
    pub fmc_verified_by: Option<i32>,
    pub speed_verified: Option<bool>,
    pub speed_verified_by: Option<i32>,

    // Evidence
    pub log_file_name: Option<String>,
    pub scramble_seed: Option<String>,
    pub video_url: Option<String>,

    // Puzzle
    pub puzzle_id: Option<i32>,
    pub puzzle_name: Option<String>,
    pub puzzle_primary_filters: Option<bool>,
    pub puzzle_primary_macros: Option<bool>,
    pub puzzle_hsc_id: Option<String>,
    pub puzzle_autoverifiable: Option<bool>,

    // Variant
    pub variant_id: Option<i32>,
    pub variant_name: Option<String>,
    pub variant_prefix: Option<String>,
    pub variant_suffix: Option<String>,
    pub variant_abbr: Option<String>,
    pub variant_material_by_default: Option<bool>,
    pub variant_primary_filters: Option<bool>,
    pub variant_primary_macros: Option<bool>,

    pub primary_filters: Option<bool>,
    pub primary_macros: Option<bool>,

    // Program
    pub program_id: Option<i32>,
    pub program_name: Option<String>,
    pub program_abbr: Option<String>,
    pub program_material: Option<bool>,

    // Solver
    pub solver_id: Option<i32>,
    pub solver_name: Option<String>,
}
#[allow(unused)]
fn _assert_inlined_solve_fields() {
    query_as!(InlinedSolve, "SELECT * FROM InlinedSolve");
}

/// View of a solve with only the raw database fields that can be submitted or
/// edited directly.
#[derive(Clone)]
pub struct SolveDbFields {
    // Event
    pub puzzle_id: i32,
    pub variant_id: Option<i32>,
    pub program_id: i32,

    // Metadata
    pub solver_id: i32,
    pub solve_date: DateTime<Utc>,
    pub solver_notes: String,
    pub moderator_notes: Option<String>, // set separately
    pub auto_verify_output: Option<serde_json::Value>, // set separately

    // Flags
    pub average: bool,
    pub blind: bool,
    pub filters: bool,
    pub macros: bool,
    pub one_handed: bool,
    pub computer_assisted: bool,

    // Stats
    pub move_count: Option<i32>,
    pub speed_cs: Option<i32>,
    pub memo_cs: Option<i32>,

    // Evidence
    pub log_file: Option<Option<(String, Vec<u8>)>>, // set separately
    pub video_url: Option<String>,
}
impl fmt::Debug for SolveDbFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SolveDbFields")
            .field("puzzle_id", &self.puzzle_id)
            .field("variant_id", &self.variant_id)
            .field("program_id", &self.program_id)
            .field("solver_id", &self.solver_id)
            .field("solve_date", &self.solve_date)
            .field("solver_notes", &self.solver_notes)
            .field("moderator_notes", &self.moderator_notes)
            .field("auto_verify_output", &self.auto_verify_output)
            .field("average", &self.average)
            .field("blind", &self.blind)
            .field("filters", &self.filters)
            .field("macros", &self.macros)
            .field("one_handed", &self.one_handed)
            .field("computer_assisted", &self.computer_assisted)
            .field("move_count", &self.move_count)
            .field("speed_cs", &self.speed_cs)
            .field("memo_cs", &self.memo_cs)
            .field(
                "log_file",
                &self
                    .log_file
                    .as_ref()
                    .map(|opt| opt.as_ref().map(|(file_name, _file_contents)| file_name)),
            )
            .field("video_url", &self.video_url)
            .finish()
    }
}
impl SolveDbFields {
    /// Removes fields that cannot be set using the given authorization.
    pub fn filter_for_auth(&mut self, auth: EditAuthorization, old_solver_id: UserId) {
        match auth {
            EditAuthorization::Moderator => (),
            EditAuthorization::IsSelf => {
                self.solver_id = old_solver_id.0; // keep unchanged
                self.moderator_notes = None; // do not set
                self.auto_verify_output = None; // do not set
            }
        }
    }
}
impl From<FullSolve> for SolveDbFields {
    fn from(solve: FullSolve) -> Self {
        SolveDbFields {
            puzzle_id: solve.puzzle.id.0,
            variant_id: solve.variant.map(|v| v.id.0),
            program_id: solve.program.id.0,
            solver_id: solve.solver.id.0,
            solve_date: solve.solve_date,
            solver_notes: solve.solver_notes.unwrap_or_default(),
            moderator_notes: solve.moderator_notes,
            auto_verify_output: solve.auto_verify_output,
            average: solve.flags.average,
            blind: solve.flags.blind,
            filters: solve.flags.filters,
            macros: solve.flags.macros,
            one_handed: solve.flags.one_handed,
            computer_assisted: solve.flags.computer_assisted,
            move_count: solve.move_count,
            speed_cs: solve.speed_cs,
            memo_cs: solve.memo_cs,
            log_file: None, // no change
            video_url: solve.video_url,
        }
    }
}

impl FullSolve {
    /// Returns a SQL fragment of the fields by which to separate categories.
    pub const CATEGORY_PARTITIONING: &str = "puzzle_id, variant_id, program_material";

    /// Returns a SQL fragment of the fields by which to order speedsolving
    /// leaderboards.
    pub const SPEED_ORDER: &str = "speed_cs ASC NULLS LAST, solve_date, upload_date";
    /// Returns the key by which to sort solves in speed leaderboards.
    #[allow(dead_code)]
    pub fn speed_sort_key(&self) -> impl Ord {
        // Sort by speed first and use solve date and upload time as
        // tiebreakers.
        (
            self.speed_cs.is_none(),
            self.speed_cs,
            self.solve_date,
            self.upload_date,
        )
    }

    /// Returns a SQL fragment of the fields by which to order FMC leaderboards.
    pub const FMC_ORDER: &str = "move_count ASC NULLS LAST, solve_date, upload_date";
    /// Returns the key by which to sort solves in FMC leaderboards.
    #[allow(dead_code)]
    pub fn fmc_sort_key(&self) -> impl Ord {
        // Sort by move count and use solve date and upload time as a
        // tiebreaker.
        (
            self.move_count.is_none(),
            self.move_count,
            self.solve_date,
            self.upload_date,
        )
    }

    /// Returns whether a user is allowed to edit the solve.
    pub fn can_edit(&self, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if self.solver.id == editor.id
            && self.fmc_verified != Some(true)
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

    /// Returns whether `viewer` is allowed to view the solve.
    pub fn can_view_opt(&self, viewer: Option<&User>) -> bool {
        self.speed_verified == Some(true)
            || self.fmc_verified == Some(true)
            || viewer.is_some_and(|u| u.moderator)
            || viewer.is_some_and(|u| u.id == self.solver.id)
    }
    /// Returns whether `viewer` is allowed to view speed solve info.
    pub fn can_view_speed(&self, viewer: Option<&User>) -> bool {
        self.speed_verified == Some(true)
            || viewer.is_some_and(|u| u.moderator)
            || viewer.is_some_and(|u| u.id == self.solver.id)
    }
    /// Returns whether `viewer` is allowed to view fewest-moves solve info.
    pub fn can_view_fmc(&self, viewer: Option<&User>) -> bool {
        self.fmc_verified == Some(true)
            || viewer.is_some_and(|u| u.moderator)
            || viewer.is_some_and(|u| u.id == self.solver.id)
    }

    pub fn counts_for_primary_speed_category(&self) -> bool {
        self.flags.filters <= self.puzzle.primary_filters
            && self.flags.macros <= self.puzzle.primary_macros
    }

    pub fn speed_event(&self) -> Event {
        Event {
            puzzle: self.puzzle.clone(),
            category: Category::new_speed(self.flags, self.variant.clone(), self.program.material),
        }
    }
    pub fn fmc_event(&self) -> Event {
        Event {
            puzzle: self.puzzle.clone(),
            category: Category::new_fmc(self.flags),
        }
    }
    /// Returns either `speed_event()` or `fmc_event()` based on heuristics
    /// about whether the solve is primarily a speedsolve or FMC solve.
    pub fn primary_event(&self) -> Event {
        match self.primary_event_class() {
            EventClass::Speed => self.speed_event(),
            EventClass::Fmc => self.fmc_event(),
        }
    }

    fn primary_event_class(&self) -> EventClass {
        if self.speed_verified == Some(true) {
            EventClass::Speed
        } else if self.fmc_verified == Some(true) {
            EventClass::Fmc
        } else if self.speed_cs.is_some() {
            EventClass::Speed
        } else if self.move_count.is_some() {
            EventClass::Fmc
        } else {
            EventClass::Speed
        }
    }

    pub fn primary_category_query(&self) -> CategoryQuery {
        match self.primary_event_class() {
            EventClass::Speed => CategoryQuery::Speed {
                average: self.flags.average,
                blind: self.flags.blind,
                filters: Some(self.flags.filters).filter(|&b| b != self.puzzle.primary_filters),
                macros: Some(self.flags.macros).filter(|&b| b != self.puzzle.primary_macros),
                one_handed: self.flags.one_handed,
                variant: match &self.variant {
                    Some(variant) => VariantQuery::Named(variant.name.clone()),
                    None => VariantQuery::Default,
                },
                program: match self.program.material {
                    true => ProgramQuery::Material,
                    false => ProgramQuery::Virtual,
                },
            },
            EventClass::Fmc => CategoryQuery::Fmc {
                computer_assisted: self.flags.computer_assisted,
            },
        }
    }
}

impl AppState {
    pub async fn get_opt_solve(&self, id: SolveId) -> sqlx::Result<Option<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT * FROM InlinedSolve WHERE id = $1",
            id.0,
        )
        .try_map(FullSolve::try_from)
        .fetch_optional(&self.pool)
        .await
    }
    pub async fn get_solve(&self, id: SolveId) -> AppResult<FullSolve> {
        self.get_opt_solve(id).await?.ok_or(AppError::InvalidSolve)
    }

    fn sql_from_verified_solves_in_category<'q>(
        &self,
        q: &mut QueryBuilder<'q, Postgres>,
        puzzle: Option<PuzzleId>,
        category: &'q CategoryQuery,
        require_verified: bool,
    ) {
        match category {
            CategoryQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                program,
            } => {
                q.push(" FROM InlinedSolve WHERE speed_cs IS NOT NULL");
                if require_verified {
                    q.push(" AND speed_verified IS TRUE");
                } else {
                    q.push(" AND speed_verified IS NOT FALSE");
                }
                if let Some(puzzle) = puzzle {
                    q.push(" AND puzzle_id = ").push_bind(puzzle.0);
                }
                q.push(" AND average = ").push_bind(*average);
                q.push(" AND blind = ").push_bind(*blind);
                match filters {
                    Some(filters) => q.push(" AND filters <= ").push_bind(filters),
                    None => q.push(" AND filters <= primary_filters"),
                };
                match macros {
                    Some(macros) => q.push(" AND macros <= ").push_bind(macros),
                    None => q.push(" AND macros <= primary_macros"),
                };
                q.push(" AND one_handed = ").push_bind(*one_handed);
                match variant {
                    VariantQuery::All => &mut *q,
                    VariantQuery::Default => q.push(" AND variant_id IS NULL"),
                    VariantQuery::Named(variant_abbr) => {
                        q.push(" AND variant_abbr = ").push_bind(variant_abbr)
                    }
                };
                match program {
                    ProgramQuery::Default => q.push(
                        " AND program_material = COALESCE(variant_material_by_default, false)",
                    ),
                    ProgramQuery::Material => q.push(" AND program_material"),
                    ProgramQuery::Virtual => q.push(" AND NOT program_material"),
                    ProgramQuery::All => &mut *q,
                    ProgramQuery::Programs(items) => q
                        .push(" AND program_abbr = ANY(")
                        .push_bind(items)
                        .push(")"),
                };
            }
            CategoryQuery::Fmc { computer_assisted } => {
                q.push(" FROM VerifiedFmcSolve WHERE move_count IS NOT NULL");
                if require_verified {
                    q.push(" AND fmc_verified IS TRUE");
                } else {
                    q.push(" AND fmc_verified IS NOT FALSE");
                }
                if let Some(puzzle) = puzzle {
                    q.push(" AND puzzle_id = ").push_bind(puzzle.0);
                }
                q.push(" AND computer_assisted <= ")
                    .push_bind(*computer_assisted);
            }
        }
    }

    #[allow(clippy::useless_format)]
    fn sql_select_ranked_leaderboards_from_category<'q>(
        &self,
        q: &mut QueryBuilder<'q, Postgres>,
        puzzle: Option<PuzzleId>,
        category: &'q CategoryQuery,
    ) {
        let score = category.sql_order_fields();
        let partitioning = FullSolve::CATEGORY_PARTITIONING;

        q.push("     SELECT");
        q.push(format!(" *, RANK() OVER ("));
        q.push(format!("     PARTITION BY ({partitioning})"));
        q.push(format!("     ORDER BY {score}"));
        q.push("         ) AS rank");
        q.push("         FROM (");
        q.push(format!("     SELECT"));
        q.push(format!("         DISTINCT ON (solver_id, {partitioning})"));
        q.push(format!("         *"));
        self.sql_from_verified_solves_in_category(q, puzzle, category, true);
        q.push(format!("     ORDER BY solver_id, {partitioning}, {score}"));
        q.push("         ) as s");
    }

    pub async fn get_all_puzzles_counts(
        &self,
        query: &CategoryQuery,
    ) -> sqlx::Result<Vec<(MainPageCategory, i64)>> {
        let partitioning = FullSolve::CATEGORY_PARTITIONING;
        let mut q = QueryBuilder::new(format!(
            "SELECT {partitioning}, COUNT(DISTINCT solver_id) as count",
        ));
        self.sql_from_verified_solves_in_category(&mut q, None, query, true);
        q.push(format!(" GROUP BY {partitioning}"));

        q.build()
            .try_map(|row| {
                let main_page_category = match query {
                    CategoryQuery::Speed { .. } => MainPageCategory::Speed {
                        puzzle: PuzzleId(row.try_get("puzzle_id")?),
                        variant: row.try_get::<Option<_>, _>("variant_id")?.map(VariantId),
                        material: row.try_get("program_material")?,
                    },
                    CategoryQuery::Fmc { .. } => MainPageCategory::Fmc {
                        puzzle: PuzzleId(row.try_get("puzzle_id")?),
                    },
                };
                Ok((main_page_category, row.try_get("count")?))
            })
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_score_leaderboard(
        &self,
        score: ScoreQuery,
    ) -> sqlx::Result<Vec<(i64, PublicUser, String)>> {
        match score {
            ScoreQuery::Distinct => self.get_distinct_puzzles_leaderboard().await,
        }
    }

    pub async fn get_distinct_puzzles_leaderboard(
        &self,
    ) -> sqlx::Result<Vec<(i64, PublicUser, String)>> {
        query!(
            "SELECT
                solver_id, solver_name,
                COUNT(DISTINCT puzzle_id) AS score,
                RANK() OVER (ORDER BY COUNT(DISTINCT puzzle_id) DESC) as rank
                FROM VerifiedSolve
                GROUP BY solver_id, solver_name
                ORDER BY rank ASC, solver_id ASC
            "
        )
        .try_map(|row| {
            // IIFE to mimic try_block
            (|| {
                Ok((
                    row.rank.ok_or("rank")?,
                    PublicUser {
                        id: UserId(row.solver_id.ok_or("solver_id")?),
                        name: row.solver_name,
                    },
                    row.score.ok_or("score")?.to_string(),
                ))
            })()
            .map_err(MissingField::new_sqlx_error)
        })
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_event_leaderboard(
        &self,
        puzzle: &Puzzle,
        category: &CategoryQuery,
    ) -> sqlx::Result<Vec<RankedFullSolve>> {
        let mut q = QueryBuilder::default();
        self.sql_select_ranked_leaderboards_from_category(&mut q, Some(puzzle.id), category);
        q.build_query_as::<RankedFullSolve>()
            .fetch_all(&self.pool)
            .await
    }

    /// Returns the world record for every combination of puzzle, variant,
    /// materialness.
    pub async fn get_all_puzzles_leaderboard(
        &self,
        query: &CategoryQuery,
    ) -> sqlx::Result<Vec<(Event, FullSolve)>> {
        let mut q =
            QueryBuilder::new("SELECT DISTINCT ON (puzzle_id, variant_id, program_material) *");
        self.sql_from_verified_solves_in_category(&mut q, None, query, true);
        q.push(format!(
            " ORDER BY puzzle_id, variant_id, program_material, {}",
            match query {
                CategoryQuery::Speed { .. } => FullSolve::SPEED_ORDER,
                CategoryQuery::Fmc { .. } => FullSolve::FMC_ORDER,
            }
        ));

        q.build()
            .try_map(|row| {
                let solve = FullSolve::from_row(&row)?;
                let event = Event {
                    puzzle: solve.puzzle.clone(),
                    category: match query {
                        CategoryQuery::Speed {
                            average,
                            blind,
                            filters,
                            macros,
                            one_handed,
                            variant: _,
                            program: _,
                        } => Category::Speed {
                            average: *average,
                            blind: *blind,
                            filters: filters.unwrap_or(match &solve.variant {
                                Some(v) => v.primary_filters,
                                None => solve.puzzle.primary_filters,
                            }),
                            macros: macros.unwrap_or(match &solve.variant {
                                Some(v) => v.primary_macros,
                                None => solve.puzzle.primary_macros,
                            }),
                            one_handed: *one_handed,
                            variant: solve.variant.clone(),
                            material: solve.program.material,
                        },

                        CategoryQuery::Fmc { computer_assisted } => Category::Fmc {
                            computer_assisted: *computer_assisted,
                        },
                    },
                };
                Ok((event, solve))
            })
            .fetch_all(&self.pool)
            .await
    }

    /// Returns all variants that have solves.
    pub async fn get_puzzle_combined_variants(
        &self,
        puzzle: PuzzleId,
    ) -> sqlx::Result<Vec<CombinedVariant>> {
        query!(
            "SELECT DISTINCT
                variant_id, variant_name, variant_abbr, variant_material_by_default, program_material,
                (program_material <> COALESCE(variant_material_by_default, FALSE)) as xor_result
                FROM VerifiedSpeedSolve
                WHERE puzzle_id = $1
                ORDER BY variant_id NULLS FIRST, xor_result
            ",
            puzzle.0
        )
        .try_map(|row| {
            let program_material = row
                .program_material
                .ok_or("program_material")
                .map_err(MissingField::new_sqlx_error)?;
            Ok(CombinedVariant::new(
                row.variant_name,
                row.variant_abbr,
                row.variant_material_by_default,
                program_material,
            ))
        })
        .fetch_all(&self.pool)
        .await
    }

    /// Returns all solves of for a puzzle category query, in order.
    pub async fn get_solve_history(
        &self,
        puzzle: &Puzzle,
        category_query: &CategoryQuery,
    ) -> sqlx::Result<Vec<FullSolve>> {
        let mut q = QueryBuilder::new("SELECT *");
        self.sql_from_verified_solves_in_category(&mut q, Some(puzzle.id), category_query, true);
        q.push(" ORDER BY solve_date, upload_date");
        q.build()
            .try_map(|row| FullSolve::from_row(&row))
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_record_history(
        &self,
        puzzle: &Puzzle,
        category_query: &CategoryQuery,
    ) -> sqlx::Result<Vec<FullSolve>> {
        let all_solves = self
            .get_solve_history(puzzle, category_query)
            .await?
            .into_iter();
        let mut ret = match category_query {
            CategoryQuery::Speed { .. } => {
                let mut best_time = i32::MAX;
                all_solves
                    .filter(|solve| {
                        let better_time = solve.speed_cs.filter(|&it| it <= best_time);
                        better_time.inspect(|&it| best_time = it).is_some()
                    })
                    .collect_vec()
            }
            CategoryQuery::Fmc { .. } => {
                let mut best_count = i32::MAX;
                all_solves
                    .filter(|solve| {
                        let better_count = solve.move_count.filter(|&it| it <= best_count);
                        better_count.inspect(|&it| best_count = it).is_some()
                    })
                    .collect_vec()
            }
        };
        ret.reverse();
        Ok(ret)
    }

    pub async fn get_solver_pbs(
        &self,
        user_id: UserId,
        category: &CategoryQuery,
    ) -> sqlx::Result<Vec<(MainPageCategory, RankedFullSolve)>> {
        let mut q = QueryBuilder::default();
        q.push(" SELECT * FROM (");
        self.sql_select_ranked_leaderboards_from_category(&mut q, None, category);
        q.push("     ) as ss");
        q.push("     WHERE solver_id = ").push_bind(user_id.0);
        Ok(q.build_query_as::<RankedFullSolve>()
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|ranked_solve| {
                let RankedFullSolve { solve, .. } = &ranked_solve;
                let main_page_category = match category {
                    CategoryQuery::Speed { .. } => MainPageCategory::Speed {
                        puzzle: solve.puzzle.id,
                        variant: solve.variant.as_ref().map(|v| v.id),
                        material: solve.program.material,
                    },
                    CategoryQuery::Fmc { .. } => MainPageCategory::Fmc {
                        puzzle: solve.puzzle.id,
                    },
                };
                (main_page_category, ranked_solve)
            })
            .collect())
    }

    pub async fn get_solver_submissions(&self, user_id: UserId) -> sqlx::Result<Vec<FullSolve>> {
        query_as!(
            InlinedSolve,
            "SELECT * FROM InlinedSolve WHERE solver_id = $1 ORDER BY upload_date DESC",
            user_id.0,
        )
        .try_map(FullSolve::try_from)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_pending_submissions(&self) -> sqlx::Result<Vec<FullSolve>> {
        query_as!(InlinedSolve, "SELECT * FROM PendingSolve")
            .try_map(FullSolve::try_from)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_pending_submissions_count(&self) -> sqlx::Result<Option<i64>> {
        query_scalar!("SELECT COUNT(*) FROM PendingSolve")
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_pending_submissions_count_for_user(
        &self,
        user_id: UserId,
    ) -> sqlx::Result<Option<i64>> {
        query_scalar!(
            "SELECT COUNT(*) FROM PendingSolve WHERE solver_id = $1",
            user_id.0
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Returns the world record solve in a category, excluding the given solve
    /// (or `None` if there are no other solves in the category).
    pub async fn world_record_excluding(
        &self,
        event: &Event,
        excluding_solve: &FullSolve,
    ) -> sqlx::Result<Option<FullSolve>> {
        match &event.category {
            Category::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                material,
            } => {
                query_as!(
                    InlinedSolve,
                    "SELECT *
                        FROM VerifiedSpeedSolve
                        WHERE puzzle_id = $1
                            AND average = $2
                            AND blind = $3
                            AND filters <= $4
                            AND macros <= $5
                            AND one_handed >= $6
                            AND (variant_id = $7 OR ($7 IS NULL AND variant_id IS NULL))
                            AND program_material = $8
                            AND id <> $9
                        ORDER BY speed_cs ASC NULLS LAST, solve_date, upload_date
                        LIMIT 1
                    ",
                    event.puzzle.id.0,
                    average,
                    blind,
                    filters,
                    macros,
                    one_handed,
                    variant.as_ref().map(|v| v.id.0),
                    material,
                    excluding_solve.id.0,
                )
                .try_map(FullSolve::try_from)
                .fetch_optional(&self.pool)
                .await
            }

            Category::Fmc { computer_assisted } => {
                query_as!(
                    InlinedSolve,
                    "SELECT *
                        FROM VerifiedFmcSolve
                        WHERE puzzle_id = $1
                            AND computer_assisted = $2
                            AND id <> $3
                        ORDER BY move_count ASC NULLS LAST, solve_date, upload_date
                        LIMIT 1
                    ",
                    event.puzzle.id.0,
                    computer_assisted,
                    excluding_solve.id.0,
                )
                .try_map(FullSolve::try_from)
                .fetch_optional(&self.pool)
                .await
            }
        }
    }

    pub async fn add_solve_external(
        &self,
        editor: &User,
        mut data: SolveDbFields,
        will_be_auto_verified: bool,
    ) -> AppResult<SolveId> {
        let auth = if editor.moderator {
            EditAuthorization::Moderator
        } else {
            data.solver_id = editor.id.0;
            EditAuthorization::IsSelf
        };
        data.filter_for_auth(auth, editor.id);

        if data.speed_cs == Some(0) {
            data.speed_cs = None;
        }
        if data.move_count == Some(0) {
            data.move_count = None;
        }

        self.check_allow_submissions()?;

        let SolveDbFields {
            solver_id,
            puzzle_id,
            variant_id,
            program_id,
            solve_date,
            solver_notes,
            moderator_notes,
            auto_verify_output,
            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,
            move_count,
            speed_cs,
            memo_cs,
            log_file,
            video_url,
        } = data.clone();

        let (log_file_name, log_file_contents) = log_file.flatten().unzip();

        let mut transaction = self.pool.begin().await?;

        let solve_id = query!(
            "INSERT INTO Solve
                    (solver_id, solve_date,
                     puzzle_id, variant_id, program_id,
                     average, blind, filters, macros, one_handed, computer_assisted,
                     move_count, speed_cs, memo_cs,
                     log_file_name, log_file_contents, video_url,
                     solver_notes, moderator_notes, auto_verify_output)
                VALUES ($1, $2,
                        $3, $4, $5,
                        $6, $7, $8, $9, $10, $11,
                        $12, $13, $14,
                        $15, $16, $17,
                        $18, $19, $20)
                RETURNING id
            ",
            //
            solver_id,
            solve_date,
            //
            puzzle_id,
            variant_id,
            program_id,
            //
            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,
            //
            move_count,
            speed_cs,
            memo_cs,
            //
            log_file_name,
            log_file_contents,
            video_url,
            //
            solver_notes,
            moderator_notes.unwrap_or_default(),
            auto_verify_output,
        )
        .fetch_one(&mut *transaction)
        .await?
        .id;

        let solve_id = SolveId(solve_id);

        let stored_data = fetch_log_fields_for_solve!(&mut *transaction, solve_id).await?;

        let fields = fields_map!(
            stored_data,
            [
                solver_id,
                solve_date,
                upload_date,
                solver_notes,
                moderator_notes,
                puzzle_id,
                variant_id,
                program_id,
                average,
                blind,
                filters,
                macros,
                one_handed,
                computer_assisted,
                move_count,
                speed_cs,
                memo_cs,
                log_file_name,
                video_url,
            ],
        );
        let object = None;
        let event = if data.solver_id == editor.id.0 {
            AuditLogEvent::Submitted { object, fields }
        } else {
            AuditLogEvent::Added { object, fields }
        };
        Self::add_solve_log_entry(&mut transaction, editor, solve_id, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id, solve = ?solve_id, ?data, "Manual solve submission added.");
        self.alert_discord_of_solve(editor, solve_id, false, will_be_auto_verified)
            .await;

        Ok(solve_id)
    }

    pub async fn update_solve(
        &self,
        id: SolveId,
        mut new_data: SolveDbFields,
        editor: &User,
        audit_log_comment: &str,
    ) -> AppResult {
        let old_solve = self.get_solve(id).await?;
        let auth = editor.try_edit_auth(&old_solve)?;
        new_data.filter_for_auth(auth, old_solve.solver.id);

        if new_data.speed_cs == Some(0) {
            new_data.speed_cs = None;
        }
        if new_data.move_count == Some(0) {
            new_data.move_count = None;
        }

        self.check_allow_edit(editor)?;

        let mut transaction = self.pool.begin().await?;

        let old_stored_data = fetch_log_fields_for_solve!(&mut *transaction, id).await?;

        // Disallow sub-day changes to solve date because the form isn't granular enough
        if old_stored_data.solve_date.date_naive() == new_data.solve_date.date_naive() {
            new_data.solve_date = old_stored_data.solve_date;
        }

        let SolveDbFields {
            solver_id,
            puzzle_id,
            variant_id,
            program_id,
            solve_date,
            solver_notes,
            moderator_notes,
            auto_verify_output,
            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,
            move_count,
            speed_cs,
            memo_cs,
            log_file,
            video_url,
        } = new_data.clone();

        query!(
            "UPDATE Solve
                SET solver_id = $1, solve_date = $2,
                    puzzle_id = $3, variant_id = $4, program_id = $5,
                    average = $6, blind = $7, filters = $8, macros = $9, one_handed = $10, computer_assisted = $11,
                    move_count = $12, speed_cs = $13, memo_cs = $14,
                    video_url = $15,
                    solver_notes = $16
                WHERE Solve.id = $17
                RETURNING Solve.id",
            //
            solver_id,
            solve_date,
            //
            puzzle_id,
            variant_id,
            program_id,
            //
            average,
            blind,
            filters,
            macros,
            one_handed,
            computer_assisted,
            //
            move_count,
            speed_cs,
            memo_cs,
            //
            video_url,
            //
            solver_notes,
            //
            id.0,
        )
        .fetch_one(&mut *transaction)
        .await?;

        if let Some(moderator_notes) = moderator_notes {
            query!(
                "UPDATE Solve
                    SET moderator_notes = $1
                    WHERE Solve.id = $2
                    RETURNING Solve.id",
                moderator_notes,
                id.0,
            )
            .fetch_one(&mut *transaction)
            .await?;
        }

        if let Some(auto_verify_output) = auto_verify_output {
            query!(
                "UPDATE Solve
                    SET auto_verify_output = $1
                    WHERE Solve.id = $2
                    RETURNING Solve.id",
                auto_verify_output,
                id.0,
            )
            .fetch_one(&mut *transaction)
            .await?;
        }

        let changed_log_file = log_file.is_some();
        if let Some(log_file) = log_file {
            let (log_file_name, log_file_contents) = log_file.unzip();
            query!(
                "UPDATE Solve
                    SET log_file_name = $1, log_file_contents = $2
                    WHERE Solve.id = $3
                    RETURNING Solve.id",
                log_file_name,
                log_file_contents,
                id.0,
            )
            .fetch_one(&mut *transaction)
            .await?;
        }

        let new_stored_data = fetch_log_fields_for_solve!(&mut *transaction, id).await?;

        let mut audit_log_msg = audit_log_msg!(
            old_stored_data => new_stored_data,
            [
                solver_id,
                solve_date,
                upload_date,
                solver_notes,
                moderator_notes,
                puzzle_id,
                variant_id,
                program_id,
                average,
                blind,
                filters,
                macros,
                one_handed,
                computer_assisted,
                move_count,
                speed_cs,
                memo_cs,
                video_url,
            ],
        );
        if changed_log_file {
            audit_log_msg += "\n\tChanged log file";
        }
        let mut fields = changed_fields_map!(
            old_stored_data,
            new_stored_data,
            [
                solver_id,
                solve_date,
                upload_date,
                solver_notes,
                moderator_notes,
                puzzle_id,
                variant_id,
                program_id,
                average,
                blind,
                filters,
                macros,
                one_handed,
                computer_assisted,
                move_count,
                speed_cs,
                memo_cs,
                video_url,
            ],
        );
        if changed_log_file {
            fields.insert(
                "log_file".to_string(),
                [
                    format!("{:?}", old_stored_data.log_file_name),
                    format!("{:?}", new_stored_data.log_file_name),
                ],
            );
        }
        let event = AuditLogEvent::Updated {
            object: None,
            fields,
            comment: Some(audit_log_comment.trim().to_string()).filter(|s| !s.is_empty()),
        };
        Self::add_solve_log_entry(&mut transaction, editor, id, event).await?;

        transaction.commit().await?;

        if !editor.moderator {
            self.alert_discord_of_solve(editor, id, true, false).await;
        }

        tracing::info!(editor_id = ?editor.id, solve_id = ?id, ?new_data, "Solve updated.");

        Ok(())
    }

    pub async fn verify_speed(
        &self,
        editor: &User,
        solve_id: SolveId,
        verified: Option<bool>,
        audit_log_comment: &str,
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        self.check_allow_moderator_actions()?;

        let solve = self.get_solve(solve_id).await?;
        if verified.is_some() && solve.speed_cs.is_none() {
            return Err(AppError::Other("Not a speed solve".to_string()));
        }
        if verified == solve.speed_verified {
            return Err(AppError::Other("No change".to_string()));
        }

        let mut transaction = self.pool.begin().await?;

        let old_stored_data = query!("SELECT speed_verified FROM Solve WHERE id = $1", solve_id.0,)
            .fetch_one(&mut *transaction)
            .await?;

        query!(
            "UPDATE Solve
                SET
                    speed_verified_by = $1,
                    speed_verified = $2
                WHERE id = $3
                RETURNING id
            ",
            verified.is_some().then_some(editor.id.0),
            verified,
            solve_id.0,
        )
        .fetch_one(&mut *transaction)
        .await?;

        let new_stored_data = query!("SELECT speed_verified FROM Solve WHERE id = $1", solve_id.0,)
            .fetch_one(&mut *transaction)
            .await?;

        let event = AuditLogEvent::SpeedVerified {
            old: old_stored_data.speed_verified,
            new: new_stored_data.speed_verified,
            comment: Some(audit_log_comment.trim().to_string()).filter(|s| !s.is_empty()),
        };
        Self::add_solve_log_entry(&mut transaction, editor, solve_id, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id.0, ?solve_id, ?verified, "Updated solve speed verification.");

        self.alert_discord_of_verification(Some(editor), solve_id, Some(EventClass::Speed))
            .await;

        if verified == Some(true) {
            self.alert_discord_to_speed_record(solve_id).await;
        }

        Ok(())
    }

    pub async fn verify_fmc(
        &self,
        editor: &User,
        solve_id: SolveId,
        verified: Option<bool>,
        audit_log_comment: &str,
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        self.check_allow_moderator_actions()?;

        let solve = self.get_solve(solve_id).await?;
        if verified.is_some() && solve.move_count.is_none() {
            return Err(AppError::Other("Not a fewest-moves solve".to_string()));
        }
        if verified == solve.fmc_verified {
            return Err(AppError::Other("No change".to_string()));
        }

        let mut transaction = self.pool.begin().await?;

        let old_stored_data = query!("SELECT fmc_verified FROM Solve WHERE id = $1", solve_id.0,)
            .fetch_one(&mut *transaction)
            .await?;

        query!(
            "UPDATE Solve
                SET
                    fmc_verified_by = $1,
                    fmc_verified = $2
                WHERE id = $3
                RETURNING id
            ",
            verified.is_some().then_some(editor.id.0),
            verified,
            solve_id.0,
        )
        .fetch_one(&mut *transaction)
        .await?;

        let new_stored_data = query!("SELECT fmc_verified FROM Solve WHERE id = $1", solve_id.0)
            .fetch_one(&mut *transaction)
            .await?;

        let event = AuditLogEvent::FmcVerified {
            old: old_stored_data.fmc_verified,
            new: new_stored_data.fmc_verified,
            comment: Some(audit_log_comment.trim().to_string()).filter(|s| !s.is_empty()),
        };
        Self::add_solve_log_entry(&mut transaction, editor, solve_id, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id.0, ?solve_id, ?verified, "Updated solve FMC verification.");

        self.alert_discord_of_verification(Some(editor), solve_id, Some(EventClass::Fmc))
            .await;

        if verified == Some(true) {
            self.alert_discord_to_fmc_record(solve_id).await;
        }

        Ok(())
    }

    pub async fn get_log_file_contents(
        &self,
        id: SolveId,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> sqlx::Result<Option<Vec<u8>>> {
        query_scalar!("SELECT log_file_contents FROM Solve WHERE id = $1", id.0)
            .fetch_one(executor)
            .await
    }

    pub async fn pb_in_category(
        &self,
        solver: UserId,
        puzzle: PuzzleId,
        category: &CategoryQuery,
        require_verified: bool,
    ) -> sqlx::Result<Option<FullSolve>> {
        let mut q = QueryBuilder::new("SELECT *");
        self.sql_from_verified_solves_in_category(&mut q, Some(puzzle), category, require_verified);
        q.push(" AND solver_id = ").push_bind(solver.0);
        q.push(format!(" ORDER BY {} LIMIT 1", category.sql_order_fields()));
        q.build_query_as::<FullSolve>()
            .fetch_optional(&self.pool)
            .await
    }
}
