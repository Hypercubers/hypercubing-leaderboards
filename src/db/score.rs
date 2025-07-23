#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScoreQuery {
    /// Distinct puzzles
    Distinct,
    // /// Sum of ranks
    // Sor,
    // /// Parallel sum of ranks
    // Parallel,
    // /// Kinch rank
    // Kinch,
}
