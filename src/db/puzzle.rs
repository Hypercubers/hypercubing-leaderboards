use itertools::Itertools;
use sqlx::query;

use crate::traits::Linkable;
use crate::AppState;

id_struct!(PuzzleId, Puzzle);
#[derive(serde::Serialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub name: String,
    pub primary_flags: PuzzleCategoryFlags,
}
impl Puzzle {
    pub fn primary_category(&self) -> PuzzleCategory {
        PuzzleCategory {
            base: PuzzleCategoryBase {
                puzzle: self.clone(),
                blind: false,
            },
            flags: self.primary_flags,
        }
    }
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
                    computer_assisted: false,
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
                    computer_assisted: false,
                },
            })
            .collect())
    }
}

pub struct MdSpeedCategory<'a>(pub &'a PuzzleCategory);
impl Linkable for MdSpeedCategory<'_> {
    fn relative_url(&self) -> String {
        self.0.speed_relative_url()
    }

    fn md_text(&self) -> String {
        self.0.speed_name()
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PuzzleCategory {
    pub base: PuzzleCategoryBase,
    pub flags: PuzzleCategoryFlags,
}

impl PuzzleCategory {
    pub fn speed_name(&self) -> String {
        let primary_flags = self.base.puzzle.primary_flags;
        let flags = [
            (primary_flags.uses_filters != self.flags.uses_filters).then_some(
                match self.flags.uses_filters {
                    true => "filters",
                    false => "no filters",
                },
            ),
            (primary_flags.uses_macros != self.flags.uses_macros).then_some(
                match self.flags.uses_macros {
                    true => "macros",
                    false => "no macros",
                },
            ),
        ]
        .into_iter()
        .flatten()
        .join(", ");
        let base_name = self.base.name();
        if flags.is_empty() {
            base_name
        } else {
            format!("{base_name} ({flags})")
        }
    }

    pub fn speed_subcategories(&self) -> Vec<Self> {
        self.flags
            .speed_subcategories()
            .into_iter()
            .map(|flags| Self {
                base: self.base.clone(),
                flags,
            })
            .collect()
    }

    pub fn speed_supercategories(&self) -> Vec<Self> {
        self.flags
            .speed_supercategories()
            .into_iter()
            .map(|flags| Self {
                base: self.base.clone(),
                flags,
            })
            .collect()
    }

    pub fn speed_relative_url(&self) -> String {
        format!("{}{}", self.base.url_path(), self.flags.url_params())
    }
    // TODO: remove this (ambiguous speed vs. FMC)
    pub fn url_path(&self) -> String {
        format!("{}{}", self.base.url_path(), self.flags.url_params())
    }

    pub fn counts_for_primary_category(&self) -> bool {
        let this = self.flags;
        let primary = self.base.puzzle.primary_flags;

        this.uses_filters <= primary.uses_filters
            && this.uses_macros <= primary.uses_macros
            && this.computer_assisted <= primary.computer_assisted
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn speed_url_path(&self) -> String {
        format!(
            "/puzzle?id={}{}", // TODO: consider changing this path?
            self.puzzle.id.0,
            if self.blind { "&blind" } else { "" }
        )
    }

    // TODO: remove this
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
#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PuzzleCategoryFlags {
    pub uses_filters: bool,
    pub uses_macros: bool,
    pub computer_assisted: bool,
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
    pub fn speed_subcategories(&self) -> Vec<Self> {
        let mut out = vec![];
        for uses_filters in to_false(self.uses_filters) {
            for uses_macros in to_false(self.uses_macros) {
                out.push(PuzzleCategoryFlags {
                    uses_filters,
                    uses_macros,
                    computer_assisted: false,
                });
            }
        }
        out
    }

    /// Categories with a superset of these flags.
    pub fn speed_supercategories(&self) -> Vec<Self> {
        let mut out = vec![];
        for uses_filters in to_true(self.uses_filters) {
            for uses_macros in to_true(self.uses_macros) {
                out.push(PuzzleCategoryFlags {
                    uses_filters,
                    uses_macros,
                    computer_assisted: false,
                });
            }
        }
        out
    }

    /// Returns a string of emojis representing the flags.
    pub fn emoji_string(&self) -> String {
        let mut name = "".to_string();
        if self.uses_filters {
            name += "🔎";
        }
        if self.uses_macros {
            name += "⏩";
        }
        if self.computer_assisted {
            name += "🤖";
        }
        name
    }

    /// Returns the URL parameters to filter for this category.
    pub fn url_params(&self) -> String {
        let Self {
            uses_filters,
            uses_macros,
            computer_assisted,
        } = self;
        format!("&uses_filters={uses_filters}&uses_macros={uses_macros}&computer_assisted={computer_assisted}")
    }

    /// whether self solve is in the category of other
    pub fn in_category(&self, other: &Self) -> bool {
        (!self.uses_filters || other.uses_filters) && (!self.uses_macros || other.uses_macros)
    }
}
