use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::AppState;

id_struct!(VariantId, Variant);
/// Puzzle variant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variant {
    pub id: VariantId,
    /// Full name. (e.g., "Physical")
    pub name: String,
    /// Prefix. (e.g., "Physical ")
    pub prefix: String,
    /// Suffix. (e.g., " with 1D vision")
    pub suffix: String,
    /// Abbreviated name. (e.g., "phys")
    pub abbr: String,

    /// Whether the variant is primarily for puzzles existing in the real world.
    pub material_by_default: bool,
    /// Whether the variant allows filters by default.
    pub primary_filters: bool,
    /// Whether the variant allows macros by default.
    pub primary_macros: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum VariantQuery {
    All,
    #[default]
    Default,
    Named(String),
}
impl fmt::Display for VariantQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariantQuery::All => write!(f, "all"),
            VariantQuery::Default => write!(f, "default"),
            VariantQuery::Named(name) => write!(f, "{name}"),
        }
    }
}
impl FromStr for VariantQuery {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "" | "default" => Ok(Self::Default),
            other => Ok(Self::Named(other.to_string())),
        }
    }
}
impl serde::Serialize for VariantQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for VariantQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.parse() {
            Ok(ret) => Ok(ret),
        }
    }
}

impl AppState {
    /// Returns all puzzle variants.
    pub async fn get_all_variants(&self) -> sqlx::Result<Vec<Variant>> {
        query_as!(Variant, "SELECT * FROM Variant")
            .fetch_all(&self.pool)
            .await
    }
}
