use serde::{Deserialize, Serialize};
use sqlx::query;

use crate::AppState;

id_struct!(ProgramId, Program);
/// Hypercubing program.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub id: ProgramId,
    /// Full name. (e.g., "Hyperspeedcube")
    pub name: String,
    /// Abbreviated name. (e.g., "HSC")
    pub abbreviation: String,
}

id_struct!(ProgramVersionId, ProgramVersion);
/// Specific version of a hypercubing program.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramVersion {
    pub id: ProgramVersionId,
    pub program: Program,
    /// Version string. (e.g., "v2.0.0")
    pub version: Option<String>,
}

impl ProgramVersion {
    /// Returns a human-friendly name for the program version.
    pub fn name(&self) -> String {
        format!(
            "{} {}",
            self.program.name,
            self.version.as_deref().unwrap_or("(unknown version)"),
        )
    }

    /// Returns an abbreviated name for the program version.
    pub fn abbreviation(&self) -> String {
        match &self.version {
            Some(v) => format!("{} {}", self.program.abbreviation, v),
            None => self.program.abbreviation.clone(),
        }
    }
}

impl AppState {
    /// Returns all versions of all programs.
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
