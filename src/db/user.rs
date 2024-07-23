#![allow(dead_code)]
use crate::db::EditAuthorization;
use crate::AppState;
use serde::Serialize;
use sqlx::query;
use sqlx::query_as;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: i32,
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
    pub id: i32,
    pub display_name: Option<String>,
}

impl PublicUser {
    pub fn name(&self) -> String {
        match &self.display_name {
            Some(name) => name.to_string(),
            None => format!("#{}", self.id),
        }
    }

    pub fn html_name(&self) -> String {
        ammonia::clean_text(&self.name())
    }

    pub fn url_path(&self) -> String {
        format!("/solver?id={}", self.id)
    }

    pub fn can_edit_id(target_id: i32, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if target_id == editor.id {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }

    pub fn can_edit_id_opt(target_id: i32, editor: Option<&User>) -> Option<EditAuthorization> {
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

    pub async fn get_user(&self, id: i32) -> sqlx::Result<Option<User>> {
        query_as!(User, "SELECT * FROM UserAccount WHERE id = $1", id)
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

        tracing::info!(user.id, "new user created");
        Ok(user)
    }

    pub async fn update_display_name(
        &self,
        id: i32,
        display_name: Option<String>,
    ) -> sqlx::Result<()> {
        query!(
            "UPDATE UserAccount SET display_name = $1 WHERE id = $2 RETURNING display_name",
            display_name,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        tracing::info!(user_id = id, ?display_name, "user display name updated");
        Ok(())
    }
}
