pub mod global;
pub mod per_puzzle;

#[derive(serde::Deserialize, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LeaderboardEvent {
    /// Single solve (speed)
    #[default]
    Single,
    /// Average (speed)
    Avg,
    /// Blindfolded (speed)
    Bld,
    /// One-handed (speed)
    Oh,
    /// Fewest-moves (FMC)
    Fmc,
    /// Computer-assisted fewest-moves (FMC)
    FmcCa,
    /// Distinct puzzles (aggregate)
    Distinct,
}
