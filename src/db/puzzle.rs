use sqlx::{query, query_as};

use crate::db::User;
use crate::traits::Linkable;
use crate::{AppError, AppResult, AppState};

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

#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub name: String,
    pub primary_filters: bool,
    pub primary_macros: bool,
}

impl AppState {
    pub async fn get_puzzle(&self, id: PuzzleId) -> sqlx::Result<Option<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
    }

    /// Returns all puzzles, sorted by name.
    pub async fn get_all_puzzles(&self) -> sqlx::Result<Vec<Puzzle>> {
        query_as!(Puzzle, "SELECT * FROM Puzzle ORDER BY name")
            .fetch_all(&self.pool)
            .await
    }

    /// Updates an existing puzzle.
    pub async fn update_puzzle(&self, editor: &User, puzzle: Puzzle) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let Puzzle {
            id,
            name,
            primary_filters,
            primary_macros,
        } = puzzle.clone();

        query!(
            "UPDATE Puzzle
                SET name = $1, primary_filters = $2, primary_macros = $3
                WHERE id = $4
                RETURNING id",
            name,
            primary_filters,
            primary_macros,
            id.0,
        )
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(editor_id = ?editor.id.0, ?puzzle, "Updated puzzle");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** updated puzzle **{name}**. \
             See [all puzzles](<{domain_name}/categories#puzzles>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(())
    }

    /// Adds a new puzzle to the database.
    pub async fn add_puzzle(&self, editor: &User, data: PuzzleData) -> AppResult<PuzzleId> {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let PuzzleData {
            name,
            primary_filters,
            primary_macros,
        } = data.clone();

        let puzzle_id = query!(
            "INSERT INTO Puzzle (name, primary_filters, primary_macros)
                VALUES ($1, $2, $3)
                RETURNING id",
            name,
            primary_filters,
            primary_macros
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        tracing::info!(editor_id = ?editor.id.0, ?puzzle_id, ?data, "Added puzzle");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** added a new puzzle **{name}**. \
             See [all puzzles](<{domain_name}/categories#puzzles>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(PuzzleId(puzzle_id))
    }
}
