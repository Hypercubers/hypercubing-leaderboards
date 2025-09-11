use std::fmt;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use sqlx::postgres::PgRow;
use sqlx::{query, query_as, FromRow, Postgres, QueryBuilder, Row};

use super::*;
use crate::db::category::EventClass;
use crate::error::{AppError, AppResult, MissingField};
use crate::html::puzzle_leaderboard::CombinedVariant;
use crate::traits::Linkable;
use crate::util::render_time;
use crate::AppState;

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
    /// Whether `log_file` is non-NULL. `log_file` may be very big so we don't
    /// include it unless it's requested.
    pub has_log_file: bool,
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

            has_log_file,
            scramble_seed,
            video_url,

            puzzle_id,
            puzzle_name,
            puzzle_primary_filters,
            puzzle_primary_macros,

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

                puzzle: Puzzle {
                    id: PuzzleId(puzzle_id.ok_or("puzzle_id")?),
                    name: puzzle_name.ok_or("puzzle_name")?,
                    primary_filters: puzzle_primary_filters.ok_or("puzzle_primary_filters")?,
                    primary_macros: puzzle_primary_macros.ok_or("puzzle_primary_macros")?,
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

                has_log_file: has_log_file.ok_or("has_log_file")?,
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

/// View of a solve with all relevant supplementary data, plus its rank.
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
    pub has_log_file: Option<bool>,
    pub scramble_seed: Option<String>,
    pub video_url: Option<String>,

    // Puzzle
    pub puzzle_id: Option<i32>,
    pub puzzle_name: Option<String>,
    pub puzzle_primary_filters: Option<bool>,
    pub puzzle_primary_macros: Option<bool>,

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
#[derive(Debug)]
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
impl SolveDbFields {
    /// Removes fields that cannot be set using the given authorization.
    pub fn filter_for_auth(&mut self, auth: EditAuthorization, old_solver_id: UserId) {
        match auth {
            EditAuthorization::Moderator => (),
            EditAuthorization::IsSelf => {
                self.solver_id = old_solver_id.0; // keep unchanged
                self.moderator_notes = None; // do not set
            }
        }
    }
}

impl FullSolve {
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

        embed = embed
            .field("Solver", self.solver.display_name(), true)
            .field("Puzzle", &self.puzzle.name, true);

        if let Some(variant) = &self.variant {
            embed = embed.field("Variant", &variant.name, true);
        }

        embed = embed.field("Flags", self.flags.to_string(), true);

        embed = embed.field("Program", &self.program.name, true);

        if let Some(move_count) = self.move_count {
            embed = embed.field("Move count", move_count.to_string(), true);
        }

        if let Some(notes) = &self.solver_notes {
            embed = embed.field("Solver notes", notes, true);
        }

        embed
    }

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

    pub fn url_path(&self) -> String {
        format!("/solve?id={}", self.id.0)
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
                q.push(" FROM VerifiedSpeedSolve WHERE TRUE");
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
                q.push(" FROM VerifiedFmcSolve WHERE");
                q.push(" computer_assisted <= ")
                    .push_bind(*computer_assisted);
            }
        }
    }

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
        self.sql_from_verified_solves_in_category(q, puzzle, category);
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
        self.sql_from_verified_solves_in_category(&mut q, None, query);
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
        self.sql_from_verified_solves_in_category(&mut q, None, query);
        q.push(format!(
            " ORDER BY puzzle_id, variant_id, program_material, {}",
            FullSolve::SPEED_ORDER,
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
        self.sql_from_verified_solves_in_category(&mut q, Some(puzzle.id), category_query);
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
        query_as!(
            InlinedSolve,
            "SELECT * FROM InlinedSolve
                WHERE (speed_cs > 0 AND speed_verified IS NULL)
                    OR (move_count > 0 AND fmc_verified IS NULL)
                    OR (speed_verified IS NULL AND fmc_verified IS NULL)
                ORDER BY upload_date DESC
            ",
        )
        .try_map(FullSolve::try_from)
        .fetch_all(&self.pool)
        .await
    }

    // pub async fn get_rank(
    //     &self,
    //     puzzle_category: &PuzzleCategory,
    //     solve: &FullSolve,
    // ) -> sqlx::Result<Option<i64>> {
    //     Ok(query!(
    //         "SELECT rank FROM (
    //             SELECT
    //                 id,
    //                 RANK() OVER (PARTITION BY (puzzle_id, blind) ORDER BY speed_cs) AS rank
    //                 FROM (
    //                     SELECT DISTINCT ON (user_id, puzzle_id) *
    //                         FROM VerifiedSpeedSolve
    //                         WHERE puzzle_id = $1
    //                             AND blind = $2
    //                             AND uses_filters <= $3
    //                             AND uses_macros <= $4
    //                         ORDER BY
    //                             user_id, puzzle_id,
    //                             speed_cs
    //                 ) AS s
    //             ) AS ss
    //             WHERE id = $5
    //         ",
    //         puzzle_category.base.puzzle.id.0,
    //         puzzle_category.base.blind,
    //         puzzle_category.flags.uses_filters,
    //         puzzle_category.flags.uses_macros,
    //         solve.id.0,
    //     )
    //     .fetch_one(&self.pool)
    //     .await?
    //     .rank)
    // }

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
                            AND variant_id = $7
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
        user: &User,
        mut data: SolveDbFields,
    ) -> sqlx::Result<SolveId> {
        let auth = if user.moderator {
            EditAuthorization::Moderator
        } else {
            data.solver_id = user.id.0;
            EditAuthorization::IsSelf
        };
        data.filter_for_auth(auth, user.id);

        let SolveDbFields {
            solver_id,
            puzzle_id,
            variant_id,
            program_id,
            solve_date,
            solver_notes,
            moderator_notes,
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
        } = data;

        let (log_file_name, log_file_contents) = log_file.flatten().unzip();

        let solve_id = query!(
            "INSERT INTO Solve
                    (solver_id, solve_date,
                    puzzle_id, variant_id, program_id,
                    average, blind, filters, macros, one_handed, computer_assisted,
                    move_count, speed_cs, memo_cs,
                    log_file_name, log_file_contents, video_url,
                    solver_notes, moderator_notes)
                VALUES ($1, $2,
                        $3, $4, $5,
                        $6, $7, $8, $9, $10, $11,
                        $12, $13, $14,
                        $15, $16, $17,
                        $18, $19)
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
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        let solve_id = SolveId(solve_id);
        self.alert_discord_to_verify(solve_id, false).await;

        tracing::info!(user_id = ?user.id, solve = ?solve_id, "Manual solve submission added.");

        Ok(solve_id)
    }

    pub async fn update_solve(
        &self,
        id: SolveId,
        mut data: SolveDbFields,
        editor: &User,
    ) -> AppResult {
        let old_solve = self.get_solve(id).await?;
        let auth = editor.try_edit_auth(&old_solve)?;
        data.filter_for_auth(auth, old_solve.solver.id);

        let mut transaction = self.pool.begin().await?;

        let SolveDbFields {
            solver_id,
            puzzle_id,
            variant_id,
            program_id,
            solve_date,
            solver_notes,
            moderator_notes,
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
        } = data;

        query!(
            "UPDATE Solve
                SET solver_id = $1, solve_date = $2,
                    puzzle_id = $3, variant_id = $4, program_id = $5,
                    average = $6, blind = $7, filters = $8, macros = $9, one_handed = $10, computer_assisted = $11,
                    move_count = $12, speed_cs = $13, memo_cs = $14,
                    video_url = $15,
                    solver_notes = $16
                WHERE Solve.id = $17",
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
        .execute(&mut *transaction)
        .await?;

        if let Some(moderator_notes) = moderator_notes {
            query!(
                "UPDATE Solve
                    SET moderator_notes = $1
                    WHERE Solve.id = $2",
                moderator_notes,
                id.0,
            )
            .execute(&mut *transaction)
            .await?;
        }

        if let Some(log_file) = log_file {
            let (log_file_name, log_file_contents) = log_file.unzip();
            query!(
                "UPDATE Solve
                    SET log_file_name = $1, log_file_contents = $2
                    WHERE Solve.id = $3",
                log_file_name,
                log_file_contents,
                id.0,
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id, solve_id = ?id, "Solve updated.");

        Ok(())
    }

    pub async fn verify_speed(
        &self,
        editor: &User,
        solve_id: SolveId,
        verified: Option<bool>,
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        if verified.is_some() && self.get_solve(solve_id).await?.speed_cs.is_none() {
            return Err(AppError::Other("Not a speed solve".to_string()))?;
        }

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
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(editor_id = ?editor.id.0, ?solve_id, ?verified, "Updated solve speed verification.");

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
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        if verified.is_some() && self.get_solve(solve_id).await?.move_count.is_none() {
            return Err(AppError::Other("Not a fewest-moves solve".to_string()))?;
        }

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
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(editor_id = ?editor.id.0, ?solve_id, ?verified, "Updated solve FMC verification.");

        if verified == Some(true) {
            self.alert_discord_to_fmc_record(solve_id).await;
        }

        Ok(())
    }
}
