#![allow(dead_code)]
use crate::db::EditAuthorization;
use crate::AppState;
use derive_more::From;
use derive_more::Into;
use serde::Deserialize;
use serde::Serialize;
use sqlx::query;
use sqlx::query_as;
use sqlx::Decode;
use sqlx::Encode;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Encode, Decode, From, Into,
)]
#[repr(transparent)]
pub struct UserId(pub i32);

#[derive(Serialize, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub discord_id: Option<i64>,
    pub display_name: Option<String>,
    pub moderator: bool,
    pub moderator_notes: String,
    pub dummy: bool,
}

impl User {
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id,
            display_name: self.display_name.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id: UserId,
    pub display_name: Option<String>,
}

impl PublicUser {
    pub fn name(&self) -> String {
        match &self.display_name {
            Some(name) => name.to_string(),
            None => format!("#{}", self.id.0),
        }
    }

    pub fn url_path(&self) -> String {
        format!("/solver?id={}", self.id.0)
    }

    pub fn can_edit_id(target_id: UserId, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if target_id == editor.id {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }

    pub fn can_edit_id_opt(target_id: UserId, editor: Option<&User>) -> Option<EditAuthorization> {
        editor
            .map(|editor| Self::can_edit_id(target_id, editor))
            .flatten()
    }

    pub fn can_edit(&self, editor: &User) -> Option<EditAuthorization> {
        Self::can_edit_id(self.id, editor)
    }

    pub fn can_edit_opt(&self, editor: Option<&User>) -> Option<EditAuthorization> {
        Self::can_edit_id_opt(self.id, editor)
    }
}

impl AppState {
    pub async fn get_user_from_email(&self, email: &str) -> sqlx::Result<Option<User>> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE email = $1",
            Some(email)
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_user_from_discord_id(&self, discord_id: i64) -> sqlx::Result<Option<User>> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE discord_id = $1",
            Some(discord_id)
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_user(&self, id: UserId) -> sqlx::Result<Option<User>> {
        query_as!(User, "SELECT * FROM UserAccount WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_user(
        &self,
        email: String,
        display_name: Option<String>,
    ) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO UserAccount (email, display_name) VALUES ($1, $2) RETURNING *",
            Some(email),
            display_name
        )
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(?user.id, "new user created");
        Ok(user)
    }

    pub async fn create_user_discord(
        &self,
        discord_id: i64,
        display_name: Option<String>,
    ) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO UserAccount (discord_id, display_name) VALUES ($1, $2) RETURNING *",
            Some(discord_id),
            display_name
        )
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(?user.id, "new user created");
        Ok(user)
    }

    pub async fn update_display_name(
        &self,
        id: UserId,
        display_name: Option<String>,
    ) -> sqlx::Result<()> {
        query!(
            "UPDATE UserAccount SET display_name = $1 WHERE id = $2 RETURNING display_name",
            display_name,
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        tracing::info!(user_id = ?id, ?display_name, "user display name updated");
        Ok(())
    }
}
