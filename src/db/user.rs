#![allow(dead_code)]
use crate::AppState;
use sqlx::query_as;

pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub moderator: bool,
    pub moderator_notes: String,
    pub dummy: bool,
}

impl User {
    pub fn html_name(&self) -> String {
        match &self.display_name {
            Some(name) => ammonia::clean_text(name),
            None => format!("#{}", self.id),
        }
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
}
