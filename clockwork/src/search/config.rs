use crate::search::score::Score;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub threads: usize,
    pub max_depth: u8,
    pub q_depth: u8,
    pub tt_size_mb: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            threads: 1,
            max_depth: 7,
            q_depth: 32,
            tt_size_mb: 64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub alpha: Score,
    pub beta: Score,
    pub depth: u8,
}

impl SearchParams {
    pub fn new(alpha: Score, beta: Score, depth: u8) -> Self {
        Self { alpha, beta, depth }
    }
}
