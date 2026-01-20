use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::db::{CategoryQuery, Event, FullSolve, ProgramQuery, VariantQuery};
use crate::traits::Linkable;

#[derive(serde::Serialize, Debug, Clone)]
pub struct SolveTableRow {
    pub rank: Option<i64>,

    pub puzzle_name: String,
    pub puzzle_url: String,
    #[serde(skip)]
    pub puzzle_hsc_id: Option<String>,
    pub uses_filters_icon: bool,
    pub uses_macros_icon: bool,
    pub uses_computer_assisted_icon: bool,
    pub allows_filters_icon: bool,
    pub allows_macros_icon: bool,
    pub allows_computer_assisted_icon: bool,

    pub solver_name: String,
    pub solver_url: String,

    pub speed_cs: Option<i32>,
    pub speed_verified: Option<bool>,
    pub move_count: Option<i32>,
    pub fmc_verified: Option<bool>,
    pub solve_url: String,

    pub solve_date: DateTime<Utc>,

    pub program_abbreviation: String,

    pub total_solvers: Option<i64>,
}
impl SolveTableRow {
    pub fn new(
        event: &Event,
        solve: &FullSolve,
        rank: Option<i64>,
        total_solvers: Option<i64>,
        category_query: &CategoryQuery,
    ) -> Self {
        let puzzle_cat_q = match category_query {
            CategoryQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant: _,
                program,
            } => {
                let default_material = match &solve.variant {
                    Some(v) => v.material_by_default,
                    None => false,
                };

                CategoryQuery::Speed {
                    average: *average,
                    blind: *blind,
                    filters: *filters,
                    macros: *macros,
                    one_handed: *one_handed,
                    variant: VariantQuery::from(&solve.variant),
                    program: match program {
                        ProgramQuery::All => {
                            if solve.program.material == default_material {
                                ProgramQuery::Default
                            } else {
                                match solve.program.material {
                                    true => ProgramQuery::Material,
                                    false => ProgramQuery::Virtual,
                                }
                            }
                        }
                        other => other.clone(),
                    },
                }
            }

            CategoryQuery::Fmc { .. } => category_query.clone(),
        };

        Self {
            rank,

            puzzle_name: event.name(),
            puzzle_url: event.puzzle.relative_url() + &puzzle_cat_q.url_query_params(true),
            puzzle_hsc_id: event.puzzle.hsc_id.clone(),
            uses_filters_icon: false,             // TODO
            uses_macros_icon: false,              // TODO
            uses_computer_assisted_icon: false,   // TODO
            allows_filters_icon: false,           // TODO
            allows_macros_icon: false,            // TODO
            allows_computer_assisted_icon: false, // TODO

            solver_name: solve.solver.display_name(),
            solver_url: solve.solver.relative_url() + &category_query.url_query_params(false),

            speed_cs: solve.speed_cs,
            speed_verified: solve.speed_verified,
            move_count: solve.move_count,
            fmc_verified: solve.fmc_verified,
            solve_url: solve.relative_url(),

            solve_date: solve.solve_date,

            program_abbreviation: solve.program.abbr.clone(),

            total_solvers,
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct UserTableRow {
    pub rank: i64,

    pub solver_name: String,
    pub solver_url: String,

    pub score: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct SolvesTablesResponse {
    pub tables: Vec<SolvesTable>,
}
impl From<SolvesTable> for SolvesTablesResponse {
    fn from(table: SolvesTable) -> Self {
        Self {
            tables: vec![table],
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct SolvesTable {
    pub heading: Option<String>,
    pub table_rows: LeaderboardTableRows,
    pub columns: LeaderboardTableColumns,
}
impl SolvesTable {
    /// Splits the table into multiple tables with headings.
    pub fn grouped(self) -> SolvesTablesResponse {
        if self.table_rows.is_empty() {
            return self.into();
        }

        let LeaderboardTableRows::Solves(solves) = self.table_rows else {
            return self.into();
        };

        let mut heading_to_solves = HashMap::<&str, Vec<_>>::new();

        let columns = self.columns;
        for solve in solves {
            let hsc_id = solve.puzzle_hsc_id.as_deref();
            let hsc_puzzle_or_generator_id = hsc_id
                .and_then(|s| s.split_once(':'))
                .map(|(generator, _params)| generator)
                .or(hsc_id)
                .unwrap_or_default();
            let &group_name = crate::PUZZLE_GROUPS
                .hsc_id_to_group_name
                .get(hsc_puzzle_or_generator_id)
                .unwrap_or(&crate::PUZZLE_GROUPS.default_group_name);
            heading_to_solves.entry(group_name).or_default().push(solve);
        }

        SolvesTablesResponse {
            tables: crate::PUZZLE_GROUPS
                .group_names_in_order
                .iter()
                .map(|&group_name| {
                    let solves = heading_to_solves.remove(group_name).unwrap_or_default();

                    SolvesTable {
                        heading: Some(group_name.to_string()),
                        table_rows: solves.into(),
                        columns: columns.clone(),
                    }
                })
                .filter(|table| !table.table_rows.is_empty())
                .collect(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum LeaderboardTableRows {
    Solves(Vec<SolveTableRow>),
    Users(Vec<UserTableRow>),
}
impl From<Vec<SolveTableRow>> for LeaderboardTableRows {
    fn from(solves: Vec<SolveTableRow>) -> Self {
        Self::Solves(solves)
    }
}
impl LeaderboardTableRows {
    pub fn is_empty(&self) -> bool {
        match self {
            LeaderboardTableRows::Solves(rows) => rows.is_empty(),
            LeaderboardTableRows::Users(rows) => rows.is_empty(),
        }
    }
}

/// Which columns to display in a solve table.
#[derive(serde::Serialize, Debug, Clone)]
pub struct LeaderboardTableColumns {
    pub puzzle: bool,
    pub rank: bool,
    pub solver: bool,
    pub record_holder: bool,
    pub speed_cs: bool,
    pub move_count: bool,
    pub verified: bool,
    pub date: bool,
    pub program: bool,
    pub total_solvers: bool,
    pub score: bool,
}
