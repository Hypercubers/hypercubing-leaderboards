pub mod auth;
mod category;
mod event;
mod program;
mod puzzle;
mod score;
mod setup;
mod solve;
mod user;
mod variant;

pub use category::{Category, CategoryQuery, MainPageCategory};
pub use event::Event;
pub use program::{Program, ProgramId, ProgramQuery};
pub use puzzle::{Puzzle, PuzzleId};
pub use score::ScoreQuery;
pub use solve::{FullSolve, RankedFullSolve, SolveFlags, SolveId};
pub use user::{PublicUser, User, UserId};
pub use variant::{Variant, VariantId, VariantQuery};

/// Authorization for editing an entry in the datbase.
#[derive(Debug)]
pub enum EditAuthorization {
    /// Moderator who can can edit anything.
    Moderator,
    /// Normal user who can only edit entries related to themself.
    IsSelf,
}
