use crate::AppState;
use sqlx::query_as;
use std::cmp::Ordering;

#[derive(PartialEq, Clone)]
pub struct Puzzle {
    pub id: i32,
    pub name: String,
    pub primary_filters: bool,
    pub primary_macros: bool,
}

impl AppState {
    pub async fn get_puzzle(&self, id: i32) -> sqlx::Result<Option<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_puzzles(&self) -> sqlx::Result<Vec<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle")
            .fetch_all(&self.pool)
            .await
    }
}

#[derive(PartialEq, Debug)]
pub struct PuzzleCategory {
    pub puzzle_id: i32,
    pub blind: bool,
    pub uses_filters: bool,
    pub uses_macros: bool,
}

impl PartialOrd<Self> for PuzzleCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.puzzle_id != other.puzzle_id || self.blind != other.blind {
            return None;
        }
        if self.uses_filters == other.uses_filters && self.uses_macros == self.uses_macros {
            return Some(Ordering::Equal);
        }
        if (!self.uses_filters || other.uses_filters) && (!self.uses_macros || other.uses_macros) {
            return Some(Ordering::Less);
        }
        if (self.uses_filters || !other.uses_filters) && (self.uses_macros || !other.uses_macros) {
            return Some(Ordering::Greater);
        }
        None
    }
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
impl PuzzleCategory {
    pub fn subcategories(&self) -> Vec<PuzzleCategory> {
        let mut out = vec![];
        for uses_filters in to_false(self.uses_filters) {
            for uses_macros in to_false(self.uses_macros) {
                out.push(PuzzleCategory {
                    puzzle_id: self.puzzle_id,
                    blind: self.blind,
                    uses_filters,
                    uses_macros,
                });
            }
        }
        out
    }

    pub fn supercategories(&self) -> Vec<PuzzleCategory> {
        let mut out = vec![];
        for uses_filters in to_true(self.uses_filters) {
            for uses_macros in to_true(self.uses_macros) {
                out.push(PuzzleCategory {
                    puzzle_id: self.puzzle_id,
                    blind: self.blind,
                    uses_filters,
                    uses_macros,
                });
            }
        }
        out
    }

    pub fn format_modifiers(&self) -> String {
        let mut name = "".to_string();
        if self.blind {
            name += "🙈";
        }
        if self.uses_filters {
            name += "⚗️";
        }
        if self.uses_macros {
            name += "👾";
        }
        name
    }
}
