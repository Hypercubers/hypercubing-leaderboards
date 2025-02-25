use std::fmt;
use std::str::FromStr;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::AppState;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ProgramQuery {
    /// Default for the variant.
    #[default]
    Default,
    /// Any material program.
    Material,
    /// Any virtual program.
    Virtual,
    /// Any program.
    All,
    /// Specific programs, listed by abbreviation.
    Programs(Vec<String>),
}
impl fmt::Display for ProgramQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramQuery::Default => write!(f, "default"),
            ProgramQuery::Material => write!(f, "material"),
            ProgramQuery::Virtual => write!(f, "virtual"),
            ProgramQuery::All => write!(f, "all"),
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
            "all" => Ok(ProgramQuery::All),
            other => Ok(ProgramQuery::Programs(
                other.split(',').map(str::to_owned).collect(),
            )),
        }
    }
}
impl Serialize for ProgramQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ProgramQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.parse() {
            Ok(ret) => Ok(ret),
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
