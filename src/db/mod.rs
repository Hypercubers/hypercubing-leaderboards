#![allow(dead_code)]
use crate::AppState;
use rand::distributions::{Alphanumeric, Distribution};
use rand::rngs::StdRng;
use rand::SeedableRng;
use sqlx::{query, query_as};

const TOKEN_LENGTH: i32 = 64;

pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: Option<String>,
    pub moderator: bool,
}

pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

impl AppState {
    pub async fn get_user(&self, email: &str) -> sqlx::Result<Option<User>> {
        query_as!(User, "SELECT * FROM UserAccount WHERE email = $1", email)
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
            email,
            display_name
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_token(&self, user_id: i32) -> Token {
        let mut rng = StdRng::from_entropy();
        let token =
            String::from_iter((0..TOKEN_LENGTH).map(|_| Alphanumeric.sample(&mut rng) as char));

        query_as!(
            Token,
            "INSERT INTO Token (user_id, token) VALUES ($1, $2) RETURNING *",
            user_id,
            token
        )
        .fetch_one(&self.pool)
        .await
        .expect("inserting token should succeed")
    }

    pub async fn token_bearer(&self, token: &str) -> sqlx::Result<Option<User>> {
        query_as!(User,"SELECT UserAccount.* FROM Token JOIN UserAccount ON Token.user_id = UserAccount.id WHERE Token.token = $1", token)
            .fetch_optional(&self.pool)
            .await
    }
}
