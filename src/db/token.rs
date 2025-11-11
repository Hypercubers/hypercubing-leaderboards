use chrono::{DateTime, TimeDelta, Utc};
use sqlx::query_as;

use crate::db::{User, UserId};
use crate::{AppError, AppResult, AppState};

/// How long a token is valid for.
const TOKEN_DURATION: TimeDelta = TimeDelta::days(365);
/// Total numbers of characters in a token, including random characters and
/// expiry date.
const TOTAL_TOKEN_LEN: usize = 64;
/// Minimum number of random characters in a token.
///
/// These must be unique and cryptographically secure because we don't check for
/// overlaps.
const MIN_RANDOM_TOKEN_LEN: usize = 32;

id_struct!(TokenId, Token);
/// Token for staying logged in.
pub struct Token {
    #[allow(unused)]
    pub id: TokenId, // stored in DB; never actually read by Rust code
    pub user_id: UserId,
    pub string: String,
    pub expiry: DateTime<Utc>,
}

impl Token {
    /// Returns whether the token has expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiry
    }

    pub fn new_string(expiry: DateTime<Utc>) -> String {
        let ret = expiry.date_naive().to_epoch_days().to_string() + "_";
        let remaining_len = TOTAL_TOKEN_LEN - ret.len();
        assert!(remaining_len >= MIN_RANDOM_TOKEN_LEN);
        ret + &crate::util::random_b64_string(remaining_len)
    }
}

impl AppState {
    pub async fn verify_turnstile(&self, turnstile_response: Option<String>) -> AppResult {
        let Some(turnstile) = &self.turnstile else {
            return Ok(()); // don't bother checking
        };

        let turnstile_request = cf_turnstile::SiteVerifyRequest {
            secret: None,
            response: turnstile_response.ok_or(AppError::FailedCaptcha)?,
            remote_ip: None,
        };

        if turnstile
            .siteverify(turnstile_request)
            .await
            .map_err(|e| AppError::Other(e.to_string()))?
            .success
        {
            Ok(())
        } else {
            Err(AppError::FailedCaptcha)
        }
    }

    /// Returns the status of a token, includes user that the token belongs to
    /// if it is valid.
    pub async fn token_status(&self, string: Option<&str>) -> sqlx::Result<TokenStatus> {
        let Some(string) = string else {
            return Ok(TokenStatus::None);
        };

        let token = query_as!(Token, "SELECT * FROM Token WHERE Token.string = $1", string)
            .fetch_optional(&self.pool)
            .await?;

        let Some(token) = token else {
            return Ok(TokenStatus::Unknown);
        };

        if token.is_expired() {
            return Ok(TokenStatus::Expired);
        }

        let Some(user) = self.get_opt_user(token.user_id).await? else {
            return Ok(TokenStatus::Unknown);
        };

        Ok(TokenStatus::Valid(user))
    }

    /// Creates a token for a user and adds it to the database.
    pub async fn create_token(&self, user_id: UserId) -> sqlx::Result<Token> {
        let expiry = Utc::now() + TOKEN_DURATION;

        let string = Token::new_string(expiry);

        query_as!(
            Token,
            "INSERT INTO Token (user_id, string, expiry) VALUES ($1, $2, $3) RETURNING *",
            user_id.0,
            string,
            expiry,
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Removes a token from the database.
    pub async fn remove_token(&self, string: &str) -> sqlx::Result<()> {
        query_as!(Token, "DELETE FROM Token WHERE string = $1", string)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Removes all tokens for a user from the database.
    pub async fn remove_all_tokens_for_user(&self, user: UserId) -> sqlx::Result<()> {
        query_as!(Token, "DELETE FROM Token WHERE user_id = $1", user.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Removes **all** tokens for **all** users from the database.
    pub async fn remove_all_tokens_for_all_users(&self) -> sqlx::Result<()> {
        query_as!(Token, "DELETE FROM Token")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub enum TokenStatus {
    /// No token was given.
    #[default]
    None,
    /// The token is valid and the user is logged in.
    Valid(User),
    /// The token has expired.
    Expired,
    /// The token is not recognized.
    Unknown,
}
