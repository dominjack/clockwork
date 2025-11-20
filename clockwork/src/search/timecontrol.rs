use std::time::Duration;

const MAX_SEARCH_DEPTH: usize = 64;
const SAFETX_MARGIN: u64 = 25;

#[derive(PartialEq, Debug)]
pub enum TimeControl {
    Infinite,
    Depth(usize),
    FixedTime(u64),
    Incremental(u64, u64),
    Tournament(u64, u64, u64),
}

impl TimeControl {
    /// Checks if the search should be terminated based on the current time control settings,
    /// depth, and duration. Returns `true` if the search is over, `false` otherwise.
    pub fn is_over(&self, depth: usize, duration: Duration) -> bool {
        let elapsed = duration.as_millis() as u64 + SAFETX_MARGIN;
        match self {
            // If the time control is infinite, the search is never over.
            &TimeControl::Infinite => return false,
            // If the search has reached the specified depth, it's over.
            &TimeControl::Depth(_depth) => return depth > _depth,
            // If the elapsed time has exceeded the fixed time, the search is over.
            &TimeControl::FixedTime(time) => return elapsed >= time,
            // If the elapsed time has exceeded the calculated time for the next move, the search is over.
            &TimeControl::Incremental(time, increment) => {
                return elapsed >= time / 20 + increment / 2;
            }
            // If the elapsed time has exceeded the calculated time for the next move, the search is over.
            &TimeControl::Tournament(time, increment, moves) => {
                return elapsed >= time / (moves + 1) + increment / 2;
            }
        }
    }

    /// Returns the maximum search depth based on the current time control settings.
    pub fn max_depth(&self) -> usize {
        match self {
            &TimeControl::Depth(depth) => depth,
            _ => MAX_SEARCH_DEPTH,
        }
    }
}
