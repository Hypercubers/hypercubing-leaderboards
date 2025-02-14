use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use sqlx::{query, Decode, Encode};

use crate::AppState;

id_struct!(ProgramId, Program);
#[derive(Serialize, Deserialize)]
pub struct Program {
    pub id: ProgramId,
    pub name: String,
    pub abbreviation: String,
}

id_struct!(ProgramVersionId, ProgramVersion);
#[derive(Serialize, Deserialize)]
pub struct ProgramVersion {
    pub id: ProgramVersionId,
    pub program: Program,
    pub version: Option<String>,
}

impl ProgramVersion {
    pub fn name(&self) -> String {
        format!(
            "{} {}",
            self.program.name,
            self.version
                .clone()
                .unwrap_or("(unknown version)".to_string())
        )
    }

    pub fn abbreviation(&self) -> String {
        match &self.version {
            Some(v) => format!("{} {}", self.program.abbreviation, v),
            None => self.program.abbreviation.clone(),
        }
    }
}

impl AppState {
    pub async fn get_all_program_versions(&self) -> sqlx::Result<Vec<ProgramVersion>> {
        Ok(query!(
            "SELECT
                ProgramVersion.*,
                Program.name,
                Program.abbreviation
            FROM ProgramVersion
            JOIN Program ON ProgramVersion.program_id = Program.id
            "
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|row| ProgramVersion {
            id: row.id.into(),
            version: row.version,
            program: Program {
                id: row.program_id.into(),
                name: row.name,
                abbreviation: row.abbreviation,
            },
        })
        .collect())
    }
}
