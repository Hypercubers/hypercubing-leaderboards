use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Postgres, Transaction, query, query_as};

use crate::AppState;
use crate::db::{PublicUser, User, UserId};
use crate::traits::Linkable;

#[derive(Debug, Clone)]
pub struct LogEntry {
    timestamp: DateTime<Utc>,
    editor_id: UserId,
    editor_name: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LogEntryDisplay {
    timestamp: DateTime<Utc>,
    editor_name: String,
    editor_url: String,
    description: String,
}
impl From<LogEntry> for LogEntryDisplay {
    fn from(entry: LogEntry) -> Self {
        let editor = PublicUser {
            id: entry.editor_id,
            name: entry.editor_name,
        };
        LogEntryDisplay {
            timestamp: entry.timestamp,
            editor_name: editor.display_name(),
            editor_url: editor.relative_url(),
            description: entry.description.unwrap_or_default(),
        }
    }
}

impl AppState {
    /// Returns all general log entries in reverse-chronological order
    pub async fn get_all_general_log_entries(&self) -> sqlx::Result<Vec<LogEntryDisplay>> {
        query_as!(
            LogEntry,
            "SELECT GeneralLog.timestamp,
                    GeneralLog.editor_id,
                    UserAccount.name AS editor_name,
                    GeneralLog.description
                FROM GeneralLog
                LEFT JOIN UserAccount ON GeneralLog.editor_id = UserAccount.id
                ORDER BY timestamp DESC
            "
        )
        .map(|log_entry| log_entry.into())
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_general_log_entry(
        transaction: &mut Transaction<'_, Postgres>,
        editor: &User,
        description: String,
    ) -> sqlx::Result<()> {
        query!(
            "INSERT INTO GeneralLog (editor_id, description) VALUES ($1, $2)",
            editor.id.0,
            description,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}
