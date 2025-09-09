//! Methods for updating user profile.

use sqlx::query;

use super::{User, UserId};
use crate::db::EditAuthorization;
use crate::{AppResult, AppState};

impl AppState {
    /// Updates the target user's email address.
    ///
    /// Returns an error if `editor` is not authorized.
    pub async fn update_user_email(
        &self,
        editor: &User,
        target: UserId,
        new_email: Option<String>,
    ) -> AppResult {
        let auth = editor.try_edit_auth(target)?;

        query!(
            "UPDATE UserAccount SET email = $1 WHERE id = $2",
            new_email,
            target.0,
        )
        .execute(&self.pool)
        .await?;

        log_profile_update(editor.id, target, auth, new_email, "email");

        Ok(())
    }

    /// Updates the target user's display name.
    ///
    /// Returns an error if `editor` is not authorized.
    pub async fn update_user_name(
        &self,
        editor: &User,
        target: UserId,
        new_name: Option<String>,
    ) -> AppResult {
        let auth = editor.try_edit_auth(target)?;

        query!(
            "UPDATE UserAccount SET name = $1 WHERE id = $2",
            new_name,
            target.0
        )
        .execute(&self.pool)
        .await?;

        log_profile_update(editor.id, target, auth, new_name, "name");

        Ok(())
    }

    /// Updates the target user's Discord ID.
    ///
    /// Returns an error if `editor` is not authorized.
    pub async fn update_user_discord_id(
        &self,
        editor: &User,
        target: UserId,
        new_discord_id: Option<u64>,
    ) -> AppResult {
        let auth = editor.try_edit_auth(target)?;

        query!(
            "UPDATE UserAccount SET discord_id = $1 WHERE id = $2",
            new_discord_id.map(|i| i as i64),
            target.0
        )
        .execute(&self.pool)
        .await?;

        log_profile_update(editor.id, target, auth, new_discord_id, "Discord ID");

        Ok(())
    }
}

fn log_profile_update<T: tracing::Value>(
    editor: UserId,
    target: UserId,
    auth: EditAuthorization,
    new_value: T,
    field_name: &str,
) {
    tracing::info!(
        ?editor,
        ?target,
        ?auth,
        new_value,
        "Updated user {field_name}."
    );
}
