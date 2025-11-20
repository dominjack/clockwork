use crate::search::score::Score;

/// Holds the configuration for the chess engine.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// The number of threads to use for searching.
    pub threads: usize,
    /// The maximum search depth.
    pub max_depth: u8,
    /// The maximum quiescence search depth.
    pub q_depth: u8,
    /// The size of the transposition table in megabytes.
    pub tt_size_mb: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            threads: 1,
            max_depth: 7,
            q_depth: 32,
            tt_size_mb: 1024,
        }
    }
}

/// Holds the parameters for an alpha-beta search.
#[derive(Debug, Clone)]
pub struct SearchParams {
    /// The lower bound of the search window.
    pub alpha: Score,
    /// The upper bound of the search window.
    pub beta: Score,
    /// The remaining search depth.
    pub depth: u8,
}

impl SearchParams {
    pub fn new(alpha: Score, beta: Score, depth: u8) -> Self {
        Self { alpha, beta, depth }
    }
}
