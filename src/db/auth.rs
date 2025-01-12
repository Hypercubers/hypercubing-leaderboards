#![allow(dead_code)]
use crate::db::user::User;
use crate::db::user::UserId;
use crate::AppState;
use chrono::{DateTime, TimeDelta, Utc};
use derive_more::From;
use derive_more::Into;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::Deserialize;
use serde::Serialize;
use sqlx::query_as;
use sqlx::Decode;
use sqlx::Encode;

const OTP_DURATION: TimeDelta = TimeDelta::minutes(5);
const OTP_LENGTH: i32 = 6;

const TOKEN_LENGTH: i32 = 64;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Encode, Decode, From, Into,
)]
#[repr(transparent)]
pub struct TokenId(pub i32);

#[derive(Clone, Debug)]
pub struct Otp {
    pub code: String,
    pub expiry: DateTime<Utc>,
}

impl Otp {
    pub fn is_valid(&self) -> bool {
        self.expiry > Utc::now()
    }
}

fn generate_otp() -> Otp {
    let mut rng = StdRng::from_entropy();
    let between = Uniform::from('0'..='9');
    let code = String::from_iter((0..OTP_LENGTH).map(|_| between.sample(&mut rng)));
    Otp {
        code,
        expiry: Utc::now() + OTP_DURATION,
    }
}

pub struct Token {
    pub id: TokenId,
    pub user_id: UserId,
    pub token: String,
}

impl AppState {
    pub fn create_otp(&self, user_id: UserId) -> Otp {
        let otp = generate_otp();
        self.otps.lock().insert(user_id, otp.clone());
        otp
    }

    pub fn clean_otps(&self) {
        self.otps.lock().retain(|_id, otp| otp.is_valid());
    }

    pub async fn create_token(&self, user_id: UserId) -> sqlx::Result<Token> {
        let mut rng = StdRng::from_entropy();
        let token =
            String::from_iter((0..TOKEN_LENGTH).map(|_| Alphanumeric.sample(&mut rng) as char));

        query_as!(
            Token,
            "INSERT INTO Token (user_id, token) VALUES ($1, $2) RETURNING *",
            user_id.0,
            token
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn token_bearer(&self, token: &str) -> sqlx::Result<Option<User>> {
        query_as!(User,"SELECT UserAccount.* FROM Token JOIN UserAccount ON Token.user_id = UserAccount.id WHERE Token.token = $1", token)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn remove_token(&self, token: &str) -> sqlx::Result<()> {
        query_as!(Token, "DELETE FROM Token WHERE token = $1", token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
