use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use sqlx::{query, Decode, Encode};

use crate::AppState;

id_struct!(PuzzleId, Puzzle);
#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub name: String,
    pub primary_flags: PuzzleCategoryFlags,
}

impl AppState {
    pub async fn get_puzzle(&self, id: PuzzleId) -> sqlx::Result<Option<Puzzle>> {
        Ok(query!("SELECT * FROM Puzzle WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await?
            .map(|row| Puzzle {
                id: PuzzleId(row.id),
                name: row.name,
                primary_flags: PuzzleCategoryFlags {
                    uses_filters: row.primary_filters,
                    uses_macros: row.primary_macros,
                },
            }))
    }

    pub async fn get_all_puzzles(&self) -> sqlx::Result<Vec<Puzzle>> {
        Ok(query!("SELECT * FROM Puzzle")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| Puzzle {
                id: PuzzleId(row.id),
                name: row.name,
                primary_flags: PuzzleCategoryFlags {
                    uses_filters: row.primary_filters,
                    uses_macros: row.primary_macros,
                },
            })
            .collect())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleCategory {
    pub base: PuzzleCategoryBase,
    pub flags: PuzzleCategoryFlags,
}

impl PuzzleCategory {
    pub fn subcategories(&self) -> Vec<Self> {
        self.flags
            .subcategories()
            .into_iter()
            .map(|flags| Self {
                base: self.base.clone(),
                flags,
            })
            .collect()
    }

    pub fn supercategories(&self) -> Vec<Self> {
        self.flags
            .supercategories()
            .into_iter()
            .map(|flags| Self {
                base: self.base.clone(),
                flags,
            })
            .collect()
    }

    pub fn url_path(&self) -> String {
        format!("{}{}", self.base.url_path(), self.flags.url_params())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleCategoryBase {
    pub puzzle: Puzzle,
    pub blind: bool,
}

impl PuzzleCategoryBase {
    pub fn name(&self) -> String {
        format!(
            "{}{}",
            self.puzzle.name,
            if self.blind { " Blind" } else { "" }
        )
    }

    pub fn url_path(&self) -> String {
        format!(
            "/puzzle?id={}{}",
            self.puzzle.id.0,
            if self.blind { "&blind" } else { "" }
        )
    }
}

/// Flags for what program features the solver used.
///
/// Each puzzle has a default set of flags.
#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PuzzleCategoryFlags {
    pub uses_filters: bool,
    pub uses_macros: bool,
}

fn to_true(a: bool) -> Vec<bool> {
    if a {
        vec![true]
    } else {
        vec![false, true]
    }
}

fn to_false(a: bool) -> Vec<bool> {
    if a {
        vec![false, true]
    } else {
        vec![false]
    }
}

impl PuzzleCategoryFlags {
    /// Categories with a subset of these flags.
    pub fn subcategories(&self) -> Vec<Self> {
        let mut out = vec![];
        for uses_filters in to_false(self.uses_filters) {
            for uses_macros in to_false(self.uses_macros) {
                out.push(PuzzleCategoryFlags {
                    uses_filters,
                    uses_macros,
                });
            }
        }
        out
    }

    /// Categories with a superset of these flags.
    pub fn supercategories(&self) -> Vec<Self> {
        let mut out = vec![];
        for uses_filters in to_true(self.uses_filters) {
            for uses_macros in to_true(self.uses_macros) {
                out.push(PuzzleCategoryFlags {
                    uses_filters,
                    uses_macros,
                });
            }
        }
        out
    }

    /// Returns a string of emojis representing the flags.
    pub fn emoji_str(&self) -> String {
        let mut name = "".to_string();
        if self.uses_filters {
            name += "🔎";
        }
        if self.uses_macros {
            name += "⏩";
        }
        name
    }

    /// Returns the URL parameters to filter for this category.
    pub fn url_params(&self) -> String {
        let Self {
            uses_filters,
            uses_macros,
        } = self;
        format!("&uses_filters={uses_filters}&uses_macros={uses_macros}")
    }

    /// whether self solve is in the category of other
    pub fn in_category(&self, other: &Self) -> bool {
        (!self.uses_filters || other.uses_filters) && (!self.uses_macros || other.uses_macros)
    }
}
