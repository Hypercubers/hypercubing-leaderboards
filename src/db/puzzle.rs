use sqlx::query_as;

use crate::traits::Linkable;
use crate::AppState;

id_struct!(PuzzleId, Puzzle);
#[derive(serde::Serialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub name: String,
    pub primary_filters: bool,
    pub primary_macros: bool,
}
impl Linkable for Puzzle {
    fn relative_url(&self) -> String {
        format!("/puzzle?id={}", self.id.0)
    }

    fn md_text(&self) -> String {
        self.name.clone()
    }
}

impl AppState {
    pub async fn get_puzzle(&self, id: PuzzleId) -> sqlx::Result<Option<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_puzzles(&self) -> sqlx::Result<Vec<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle")
            .fetch_all(&self.pool)
            .await
    }
}
