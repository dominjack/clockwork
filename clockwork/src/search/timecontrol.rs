use std::time::Duration;

pub static MAX_SEARCH_DEPTH: usize = 64;
static SAFETY_MARGIN: u64 = 10;

#[derive(PartialEq, Debug)]
pub enum TimeControl {
    Infinite,
    Depth(usize),
    FixedTime(u64),
    Incremental(u64, u64),
    Tournament(u64, u64, u64),
}

impl TimeControl {
    pub fn is_over(&self, depth: usize, duration: Duration) -> bool {
        let elapsed = duration.as_millis() as u64 + SAFETY_MARGIN;
        match self {
            &TimeControl::Infinite => return false,
            &TimeControl::Depth(_depth) => return depth > _depth,
            &TimeControl::FixedTime(time) => return elapsed >= time,
            &TimeControl::Incremental(time, increment) => {
                return elapsed >= time / 20 + increment / 2;
            }
            &TimeControl::Tournament(time, increment, moves) => {
                return elapsed >= time / (moves + 1) + increment / 2;
            }
        }
    }

    pub fn max_depth(&self) -> usize {
        match self {
            &TimeControl::Depth(depth) => depth,
            _ => MAX_SEARCH_DEPTH,
        }
    }
}
