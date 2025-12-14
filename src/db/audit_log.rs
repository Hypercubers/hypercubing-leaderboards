use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction, query, query_as};

use crate::AppState;
use crate::db::{AuditLogEvent, PublicUser, SolveId, User, UserId};
use crate::traits::Linkable;

#[derive(Deserialize, Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub editor_id: UserId,
    pub editor_name: Option<String>,
    pub event: AuditLogEvent,
}
impl AuditLogEntry {
    pub fn editor(&self) -> PublicUser {
        PublicUser {
            id: self.editor_id,
            name: self.editor_name.clone(),
        }
    }

    pub fn display_public(&self) -> Option<RenderedAuditLogEntry> {
        let editor = self.editor();
        Some(RenderedAuditLogEntry {
            timestamp: self.timestamp,
            editor_name: editor.display_name(),
            editor_url: editor.relative_url(),
            description: self.event.display_public()?,
        })
    }

    pub fn display_full(&self) -> RenderedAuditLogEntry {
        let editor = self.editor();
        RenderedAuditLogEntry {
            timestamp: self.timestamp,
            editor_name: editor.display_name(),
            editor_url: editor.relative_url(),
            description: self.event.display_full(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct RenderedAuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub editor_name: String,
    pub editor_url: String,
    pub description: String,
}

impl AppState {
    /// Returns all general log entries in reverse-chronological order
    pub async fn get_all_general_log_entries(&self) -> sqlx::Result<Vec<AuditLogEntry>> {
        query_as!(
            AuditLogEntry,
            "SELECT GeneralLog.timestamp,
                    GeneralLog.editor_id,
                    UserAccount.name AS editor_name,
                    GeneralLog.json_data AS event
                FROM GeneralLog
                LEFT JOIN UserAccount ON GeneralLog.editor_id = UserAccount.id
                ORDER BY timestamp DESC
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Returns all solve log entries in reverse-chronological order
    pub async fn get_all_solve_log_entries(
        &self,
        solve_id: SolveId,
    ) -> sqlx::Result<Vec<AuditLogEntry>> {
        query_as!(
            AuditLogEntry,
            "SELECT SolveLog.timestamp,
                    SolveLog.editor_id,
                    UserAccount.name AS editor_name,
                    SolveLog.json_data AS event
                FROM SolveLog
                LEFT JOIN UserAccount ON SolveLog.editor_id = UserAccount.id
                WHERE SolveLog.solve_id = $1
                ORDER BY timestamp DESC
            ",
            solve_id.0,
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Returns all user log entries in reverse-chronological order
    pub async fn get_all_user_log_entries(
        &self,
        user_id: UserId,
    ) -> sqlx::Result<Vec<AuditLogEntry>> {
        query_as!(
            AuditLogEntry,
            "SELECT UserLog.timestamp,
                    UserLog.editor_id,
                    UserAccount.name AS editor_name,
                    UserLog.json_data AS event
                FROM UserLog
                LEFT JOIN UserAccount ON UserLog.editor_id = UserAccount.id
                WHERE UserLog.user_id = $1
                ORDER BY timestamp DESC
            ",
            user_id.0,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_general_log_entry(
        transaction: &mut Transaction<'_, Postgres>,
        editor: &User,
        event: AuditLogEvent,
    ) -> sqlx::Result<()> {
        query!(
            "INSERT INTO GeneralLog (editor_id, json_data) VALUES ($1, $2)",
            editor.id.0,
            to_sql_json(event)?,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn add_solve_log_entry(
        transaction: &mut Transaction<'_, Postgres>,
        editor: &User,
        solve_id: SolveId,
        event: AuditLogEvent,
    ) -> sqlx::Result<()> {
        query!(
            "INSERT INTO SolveLog (
                solve_id, editor_id, json_data
            ) VALUES ($1, $2, $3)",
            solve_id.0,
            editor.id.0,
            to_sql_json(event)?,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn add_user_log_entry(
        transaction: &mut Transaction<'_, Postgres>,
        editor: &User,
        user_id: UserId,
        log_entry: AuditLogEvent,
    ) -> sqlx::Result<()> {
        query!(
            "INSERT INTO UserLog (user_id, editor_id, json_data) VALUES ($1, $2, $3)",
            user_id.0,
            editor.id.0,
            to_sql_json(log_entry)?,
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

fn to_sql_json(value: impl Serialize) -> sqlx::Result<serde_json::Value> {
    serde_json::to_value(&value).map_err(|e| sqlx::Error::Encode(e.into()))
}
