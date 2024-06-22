use crate::AppState;
use sqlx::query_as;

pub struct Puzzle {
    pub id: i32,
    pub hsc_id: Option<String>,
    pub name: String,
    pub leaderboard: Option<i32>,
}

impl AppState {
    pub async fn get_puzzle(&self, id: i32) -> sqlx::Result<Option<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }
}
