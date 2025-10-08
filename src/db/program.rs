use std::fmt;
use std::str::FromStr;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

use crate::{db::User, AppError, AppResult, AppState};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ProgramQuery {
    /// Default for the variant.
    #[default]
    Default,
    /// Any material program.
    Material,
    /// Any virtual program.
    Virtual,
    /// Any program.
    All,
    /// Specific programs, listed by abbreviation.
    Programs(Vec<String>),
}
impl fmt::Display for ProgramQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramQuery::Default => write!(f, "default"),
            ProgramQuery::Material => write!(f, "material"),
            ProgramQuery::Virtual => write!(f, "virtual"),
            ProgramQuery::All => write!(f, "all"),
            ProgramQuery::Programs(items) => write!(f, "{}", items.iter().join(",")),
        }
    }
}
impl FromStr for ProgramQuery {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(ProgramQuery::Default),
            "material" => Ok(ProgramQuery::Material),
            "virtual" => Ok(ProgramQuery::Virtual),
            "all" => Ok(ProgramQuery::All),
            other => Ok(ProgramQuery::Programs(
                other.split(',').map(str::to_owned).collect(),
            )),
        }
    }
}
impl Serialize for ProgramQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ProgramQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.parse() {
            Ok(ret) => Ok(ret),
        }
    }
}

id_struct!(ProgramId, Program);
/// Program.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub id: ProgramId,
    /// Full name. (e.g., "Hyperspeedcube 2")
    pub name: String,
    /// Abbreviated name. (e.g., "HSC2")
    pub abbr: String,

    /// Whether the "program" is actually material (not software).
    pub material: bool,
}

#[derive(Debug, Clone)]
pub struct ProgramData {
    pub name: String,
    pub abbr: String,
    pub material: bool,
}

impl AppState {
    /// Returns all programs, sorted by name.
    pub async fn get_all_programs(&self) -> sqlx::Result<Vec<Program>> {
        query_as!(Program, "SELECT * FROM Program ORDER BY name")
            .fetch_all(&self.pool)
            .await
    }

    /// Updates an existing program.
    pub async fn update_program(&self, editor: &User, program: Program) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let Program {
            id,
            name,
            abbr,
            material,
        } = program.clone();

        query!(
            "UPDATE Program
                SET name = $1, abbr = $2, material = $3
                WHERE id = $4
                RETURNING id",
            name,
            abbr,
            material,
            id.0,
        )
        .fetch_one(&self.pool)
        .await?;

        tracing::info!(editor_id = ?editor.id.0, ?program, "Updated program");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** updated program **{name}**. \
             See [all programs](<{domain_name}/categories#programs>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(())
    }

    /// Adds a new program to the database.
    pub async fn add_program(&self, editor: &User, data: ProgramData) -> AppResult<ProgramId> {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let ProgramData {
            name,
            abbr,
            material,
        } = data.clone();

        let program_id = query!(
            "INSERT INTO Program (name, abbr, material)
                VALUES ($1, $2, $3)
                RETURNING id",
            name,
            abbr,
            material,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        tracing::info!(editor_id = ?editor.id.0, ?program_id, ?data, "Added program");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** added a new program **{name}**. \
             See [all programs](<{domain_name}/categories#programs>)."
        );
        self.send_private_discord_update(msg).await;

        Ok(ProgramId(program_id))
    }
}
