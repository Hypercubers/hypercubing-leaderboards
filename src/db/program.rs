use std::{fmt, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::AppState;

#[derive(serde::Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ProgramQuery {
    /// Default for the variant.
    #[default]
    Default,
    /// Any material program.
    Material,
    /// Any virtual program.
    Virtual,
    /// Any program.
    Any,
    /// Specific programs, listed by abbreviation.
    Programs(Vec<String>),
}
impl fmt::Display for ProgramQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramQuery::Default => write!(f, "default"),
            ProgramQuery::Material => write!(f, "material"),
            ProgramQuery::Virtual => write!(f, "virtual"),
            ProgramQuery::Any => write!(f, "any"),
            ProgramQuery::Programs(items) => write!(f, "{}", items.iter().join(",")),
        }
    }
}
impl FromStr for ProgramQuery {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(ProgramQuery::Default),
            "material" => Ok(ProgramQuery::Material),
            "virtual" => Ok(ProgramQuery::Virtual),
            "any" => Ok(ProgramQuery::Any),
            other => Ok(ProgramQuery::Programs(
                other.split(',').map(str::to_owned).collect(),
            )),
        }
    }
}

id_struct!(ProgramId, Program);
/// Program.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub id: ProgramId,
    /// Full name. (e.g., "Hyperspeedcube 2")
    pub name: String,
    /// Abbreviated name. (e.g., "HSC2")
    pub abbr: String,

    /// Whether the "program" is actually material (not software).
    pub material: bool,
}

impl AppState {
    /// Returns all programs.
    pub async fn get_all_programs(&self) -> sqlx::Result<Vec<Program>> {
        query_as!(Program, "SELECT * FROM Program")
            .fetch_all(&self.pool)
            .await
    }
}
