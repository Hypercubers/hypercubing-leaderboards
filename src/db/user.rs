#![allow(dead_code)]
use crate::AppState;
use sqlx::query;
use sqlx::query_as;

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
    pub fn make_name(display_name: &Option<String>, id: i32) -> String {
        match display_name {
            Some(name) => name.to_string(),
            None => format!("#{}", id),
        }
    }

    pub fn make_html_name(display_name: &Option<String>, id: i32) -> String {
        match display_name {
            Some(name) => ammonia::clean_text(name),
            None => format!("#{}", id),
        }
    }

    pub fn html_name(&self) -> String {
        Self::make_html_name(&self.display_name, self.id)
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
        query_as!(
            User,
            "INSERT INTO UserAccount (email, display_name) VALUES ($1, $2) RETURNING *",
            Some(email),
            display_name
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_display_name(
        &self,
        id: i32,
        display_name: Option<String>,
    ) -> sqlx::Result<()> {
        query!(
            "UPDATE UserAccount SET display_name = $1 WHERE id = $2",
            display_name,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(())
    }
}
