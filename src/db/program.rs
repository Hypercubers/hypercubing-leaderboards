use crate::AppState;
use serde::Deserialize;
use serde::Serialize;
use sqlx::query;

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProgramVersion {
    pub id: i32,
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
            id: row.id,
            version: row.version,
            program: Program {
                id: row.program_id,
                name: row.name,
                abbreviation: row.abbreviation,
            },
        })
        .collect())
    }
}
