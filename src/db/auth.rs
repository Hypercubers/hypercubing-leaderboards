#![allow(dead_code)]
use chrono::{DateTime, TimeDelta, Utc};
use derive_more::{From, Into};
use rand::distr::{Alphanumeric, Distribution};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Decode, Encode};

use crate::db::user::{User, UserId};
use crate::AppState;

/// How long an OTP is valid for.
const OTP_DURATION: TimeDelta = TimeDelta::minutes(5);
/// Number of characters in an OTP.
const OTP_LENGTH: i32 = 6;

/// How long a token is valid for.
const TOKEN_DURATION: TimeDelta = TimeDelta::days(365);
/// Number of characters in a token.
const TOKEN_LENGTH: i32 = 64;

#[derive(
    Serialize, Deserialize, Encode, Decode, From, Into, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[repr(transparent)]
pub struct TokenId(pub i32);

/// One-time passcode for logging in.
#[derive(Debug, Clone)]
pub struct Otp {
    pub code: String,
    pub expiry: DateTime<Utc>,
}

impl Otp {
    /// Generates a new random OTP.
    pub fn new() -> Self {
        let mut rng = StdRng::from_os_rng();
        let code = String::from_iter((0..OTP_LENGTH).map(|_| rng.random_range('0'..='9')));
        Otp {
            code,
            expiry: Utc::now() + OTP_DURATION,
        }
    }

    /// Returns whether the code is still valid based on the current time.
    pub fn is_valid(&self) -> bool {
        self.expiry > Utc::now()
    }
}

pub struct Token {
    pub id: TokenId,
    pub user_id: UserId,
    pub token: String,
    pub expiry: DateTime<Utc>,
}

impl Token {
    /// Returns whether the token is still valid based on the current time.
    pub fn is_valid(&self) -> bool {
        self.expiry > Utc::now()
    }
}

impl AppState {
    /// Creates an OTP for a user.
    pub fn create_otp(&self, user_id: UserId) -> Otp {
        let otp = Otp::new();
        self.otps.lock().insert(user_id, otp.clone());
        otp
    }

    /// Removes expired OTPs.
    pub fn clean_otps(&self) {
        self.otps.lock().retain(|_id, otp| otp.is_valid());
    }

    /// Creates a token for a user and adds it to the database.
    pub async fn create_token(&self, user_id: UserId) -> sqlx::Result<Token> {
        let mut rng = StdRng::from_os_rng();
        let token =
            String::from_iter((0..TOKEN_LENGTH).map(|_| Alphanumeric.sample(&mut rng) as char));
        let expiry = Utc::now() + TOKEN_DURATION;

        query_as!(
            Token,
            "INSERT INTO Token (user_id, token, expiry) VALUES ($1, $2, $3) RETURNING *",
            user_id.0,
            token,
            expiry,
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Returns the user that the token belongs to, or `None` if it is expired.
    pub async fn token_bearer(&self, token: &str) -> sqlx::Result<Option<User>> {
        let token = query_as!(
            Token,
            "SELECT Token.* FROM Token WHERE Token.token = $1",
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        let Some(token) = token else { return Ok(None) };

        if token.is_valid() {
            self.get_user(token.user_id).await
        }else {
            Ok(None)
        }
    }

    /// Removes a token from the database.
    pub async fn remove_token(&self, token: &str) -> sqlx::Result<()> {
        query_as!(Token, "DELETE FROM Token WHERE token = $1", token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
