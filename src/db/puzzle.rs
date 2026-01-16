use hyperspeedcube_cli_types::puzzle_info::TagValue;
use sqlx::{query, query_as};

use crate::db::{AuditLogEvent, User};
use crate::traits::Linkable;
use crate::{AppError, AppResult, AppState};

id_struct!(PuzzleId, Puzzle);
#[derive(serde::Serialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Puzzle {
    pub id: PuzzleId,
    pub name: String,
    pub primary_filters: bool,
    pub primary_macros: bool,
    pub hsc_id: Option<String>,
    pub autoverifiable: bool,
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
    pub hsc_id: Option<String>,
    pub autoverifiable: bool,
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
    pub async fn update_puzzle(
        &self,
        editor: &User,
        new_data: Puzzle,
        audit_log_comment: &str,
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let Puzzle {
            id,
            name,
            primary_filters,
            primary_macros,
            hsc_id,
            autoverifiable,
        } = new_data.clone();

        let mut transaction = self.pool.begin().await?;

        let old_data = query_as!(Puzzle, "SELECT * FROM Puzzle WHERE id = $1", id.0)
            .fetch_one(&mut *transaction)
            .await?;

        query!(
            "UPDATE Puzzle
                SET name = $1, primary_filters = $2, primary_macros = $3, hsc_id = $4, autoverifiable = $5
                WHERE id = $6
                RETURNING id",
            name,
            primary_filters,
            primary_macros,
            hsc_id.filter(|s| !s.is_empty()),
            autoverifiable,
            id.0,
        )
        .fetch_one(&mut *transaction)
        .await?;

        let fields =
            changed_fields_map!(old_data, new_data, [name, primary_filters, primary_macros]);
        let event = AuditLogEvent::Updated {
            object: Some(updated_object!(Puzzle, old_data)),
            fields,
            comment: Some(audit_log_comment.trim().to_string()).filter(|s| !s.is_empty()),
        };
        Self::add_general_log_entry(&mut transaction, editor, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id.0, ?new_data, "Updated puzzle");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** updated puzzle **{name}**. \
             See [all puzzles](<{domain_name}/categories#puzzles>) \
             or [audit log](<{domain_name}/audit-log/general>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(())
    }

    /// Adds a new puzzle to the database.
    pub async fn add_puzzle(&self, editor: &User, data: PuzzleData) -> AppResult<PuzzleId> {
        let mut transaction = self.pool.begin().await?;
        let puzzle_id = self
            .add_puzzle_with_transaction(editor, data, &mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(puzzle_id)
    }

    /// Adds a new puzzle to the database.
    pub async fn add_puzzle_with_transaction(
        &self,
        editor: &User,
        data: PuzzleData,
        transaction: &mut sqlx::PgTransaction<'_>,
    ) -> AppResult<PuzzleId> {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let PuzzleData {
            name,
            primary_filters,
            primary_macros,
            hsc_id,
            autoverifiable,
        } = data.clone();

        let puzzle_id = query!(
            "INSERT INTO Puzzle (name, primary_filters, primary_macros, hsc_id, autoverifiable)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id",
            name,
            primary_filters,
            primary_macros,
            hsc_id.filter(|s| !s.is_empty()),
            autoverifiable,
        )
        .fetch_one(&mut **transaction)
        .await?
        .id;

        let event = AuditLogEvent::Added {
            object: Some(updated_object!(Puzzle, puzzle_id, data)),
            fields: fields_map!(data, [name, primary_filters, primary_macros]),
        };
        Self::add_general_log_entry(&mut *transaction, editor, event).await?;

        tracing::info!(editor_id = ?editor.id.0, ?puzzle_id, ?data, "Added puzzle");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** added a new puzzle **{name}**. \
             See [all puzzles](<{domain_name}/categories#puzzles>) \
             or [audit log](<{domain_name}/audit-log/general>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(PuzzleId(puzzle_id))
    }

    pub async fn get_puzzle_with_hsc_id(&self, hsc_puzzle_id: &str) -> AppResult<Option<PuzzleId>> {
        Ok(self
            .hsc_puzzle_metadata(hsc_puzzle_id, &mut self.pool.begin().await?)
            .await?
            .ok())
    }

    pub async fn get_or_create_puzzle_with_hsc_id(
        &self,
        hsc_puzzle_id: &str,
    ) -> AppResult<PuzzleId> {
        let mut transaction = self.pool.begin().await?;

        match self
            .hsc_puzzle_metadata(hsc_puzzle_id, &mut transaction)
            .await?
        {
            Ok(id) => Ok(id),
            Err(puzzle_data) => {
                let puzzle_id = self
                    .add_puzzle_with_transaction(
                        &self.get_hsc_auto_verify_dummy_user().await?,
                        puzzle_data,
                        &mut transaction,
                    )
                    .await?;

                transaction.commit().await?;

                Ok(puzzle_id)
            }
        }
    }

    /// If the puzzle already exists, returns its ID wrapped in `Ok()`.
    ///
    /// Otherwise, returns `PuzzleData` to add.
    async fn hsc_puzzle_metadata(
        &self,
        hsc_puzzle_id: &str,
        transaction: &mut sqlx::PgTransaction<'_>,
    ) -> AppResult<Result<PuzzleId, PuzzleData>> {
        // Get puzzle metadata
        let puzzle_metadatas: Vec<hyperspeedcube_cli_types::puzzle_info::PuzzleListMetadata> =
            serde_json::from_slice(
                &async_process::Command::new(&*crate::env::HSC2_PATH)
                    .arg("puzzle")
                    .arg(hsc_puzzle_id)
                    .output()
                    .await?
                    .stdout,
            )?;

        let puzzle_metadata = puzzle_metadatas.get(0).ok_or_else(|| {
            AppError::Other("empty puzzle metadata response from `hyperspeedcube`".to_string())
        })?;

        // Canonicalize ID, shadowing the old ID variable
        let hsc_puzzle_id =
            if let Some(TagValue::Str(canonical_id)) = puzzle_metadata.tags.get("canonical_id") {
                canonical_id.clone()
            } else {
                puzzle_metadata.id.clone()
            };

        if let Some(row) = query!("SELECT id FROM Puzzle WHERE hsc_id = $1", hsc_puzzle_id)
            .fetch_optional(&mut **transaction)
            .await?
        {
            // Puzzle already exists
            return Ok(Ok(PuzzleId(row.id)));
        }

        if puzzle_metadata.tags.get("external/leaderboard") != Some(&TagValue::Bool(true)) {
            return Err(AppError::Other(
                "puzzle is not valid on leaderboards".to_string(),
            ));
        }

        Ok(Err(PuzzleData {
            name: puzzle_metadata.name.clone(),
            primary_filters: true,
            primary_macros: false,
            hsc_id: Some(hsc_puzzle_id),
            autoverifiable: true,
        }))
    }
}
