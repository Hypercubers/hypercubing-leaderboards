#![allow(dead_code)]
use crate::AppState;
use chrono::DateTime;
use chrono::Utc;
use rand::distributions::{Alphanumeric, Distribution};
use rand::rngs::StdRng;
use rand::SeedableRng;
use sqlx::{query, query_as};

const TOKEN_LENGTH: i32 = 64;

pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub moderator: bool,
    pub moderator_notes: String,
    pub dummy: bool,
}

pub struct UserPub {
    pub id: i32,
    pub display_name: Option<String>,
    pub dummy: bool,
}

impl From<User> for UserPub {
    fn from(user: User) -> Self {
        UserPub {
            id: user.id,
            display_name: user.display_name,
            dummy: user.dummy,
        }
    }
}

impl UserPub {
    pub fn html_name(&self) -> String {
        match &self.display_name {
            Some(name) => ammonia::clean_text(name),
            None => format!("#{}", self.id),
        }
    }
}

pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

//struct SolveFlat {}

/*pub struct Solve{

    id: i32 ,
    log_file: Option<String>,
    user: User,
    upload_time: DateTime<Utc>,
    //puzzle: Puzzle,
    move_count: i32,
    uses_macros: bool,
    uses_filters: bool,
    speed_cs: i32,
    memo_cs: i32,
    blind: bool,
    scramble_seed: CHAR(64),
    program_version_id: i32 REFERENCES ProgramVersion, -- NULL should mean "unknown"
    speed_evidence_id: i32 DEFAULT NULL, -- points to the canonical evidence
    valid_solve: bool, -- NULL should mean "unverifiable" or "not yet verified", FALSE is "invalid log"
    solver_notes: String NOT NULL DEFAULT '',
    moderator_notes: String NOT NULL DEFAULT ''
}*/

impl AppState {
    pub async fn get_user(&self, email: &str) -> sqlx::Result<Option<User>> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE email = $1",
            Some(email)
        )
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

    /*async fn get_all_solves(&self) -> sqlx::Result<Vec<SolveFlat>> {
        query!(
            "SELECT Solve.speed_cs, UserAccount.display_name
            FROM Solve
            JOIN UserAccount
            ON Solve.user_id = UserAccount.id
            WHERE speed_cs IS NOT NULL"
        )
    }*/
}
