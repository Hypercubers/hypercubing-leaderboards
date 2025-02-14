pub mod auth;
pub mod program;
pub mod puzzle;
pub mod solve;
pub mod user;

/// Authorization for editing an entry in the datbase.
#[derive(Debug)]
pub enum EditAuthorization {
    /// Moderator who can can edit anything.
    Moderator,
    /// Normal user who can only edit entries related to themself.
    IsSelf,
}
