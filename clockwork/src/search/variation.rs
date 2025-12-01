use core::fmt;

use arrayvec::ArrayVec;
use chess_core::types::board::board::Board;
use chess_core::types::moves::Move;

const MAX_PLY: usize = 64;
#[derive(Debug, Clone, Default)]
pub struct Variation(ArrayVec<Move, MAX_PLY>);

impl Variation {
    pub fn new() -> Self {
        Variation(ArrayVec::new())
    }

    pub fn get_first(&self) -> Option<Move> {
        match self.0.get(0) {
            Some(mv) => return Some(*mv),
            None => return None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push_first(&mut self, mv: Move) {
        if self.0.is_full() {
            return;
        }
        self.0.insert(0, mv);
    }

    pub fn push_last(&mut self, mv: Move) {
        if self.0.is_full() {
            return;
        }
        self.0.push(mv);
    }

    pub fn as_slice(&self) -> &[Move] {
        self.0.as_slice()
    }
}

impl fmt::Display for Variation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pv_string = if !self.0.is_empty() {
            let moves_str: String = self
                .0
                .as_slice()
                .iter()
                .map(|mv| mv.to_lan())
                .collect::<Vec<String>>()
                .join(" ");

            write!(f, "{}", moves_str)
        } else {
            write!(f, "")
        };
        Ok(())
    }
}
