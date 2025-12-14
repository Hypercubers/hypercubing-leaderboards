mod audit_log;
mod audit_log_event;
mod category;
mod event;
mod profile;
mod program;
mod puzzle;
mod score;
mod setup;
mod solve;
pub mod token;
mod user;
mod variant;

pub use audit_log::RenderedAuditLogEntry;
pub use audit_log_event::{AuditLogEvent, UpdatedObject};
pub use category::{Category, CategoryQuery, MainPageCategory};
pub use event::{Event, EventClass};
pub use program::{Program, ProgramData, ProgramId, ProgramQuery};
pub use puzzle::{Puzzle, PuzzleData, PuzzleId};
pub use score::ScoreQuery;
pub use solve::{FullSolve, RankedFullSolve, SolveDbFields, SolveFlags, SolveId};
pub use user::{OptionalDiscordId, PublicUser, User, UserData, UserId};
pub use variant::{CombinedVariant, Variant, VariantData, VariantId, VariantQuery};

/// Authorization for editing an entry in the datbase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditAuthorization {
    /// Moderator who can can edit anything.
    ///
    /// This takes priority over `IsSelf`.
    Moderator,
    /// Normal user who can only edit entries related to themself.
    IsSelf,
}
