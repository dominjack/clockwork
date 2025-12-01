use crate::types::board::board::Board;
use crate::types::moves::{Move, MoveType};
use crate::types::piece::PieceType;
use arrayvec::ArrayVec;
use std::ops::{Deref, DerefMut};

/// The maximum number of moves possible in any given position.
const MAX_MOVES_IN_LIST: usize = 218;

#[derive(Debug, Clone)]
pub struct MoveList {
    pub moves: ArrayVec<Move, MAX_MOVES_IN_LIST>,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: ArrayVec::new(),
        }
    }

    pub fn capacity(&self) -> usize {
        MAX_MOVES_IN_LIST
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.moves.is_full()
    }

    pub fn try_push(&mut self, mv: Move) -> Result<(), Move> {
        self.moves.try_push(mv).map_err(|e| e.element())
    }

    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv)
    }

    pub fn extend_from_other(&mut self, other: &MoveList) {
        for &mv in other.moves.as_slice() {
            if self.is_full() {
                break;
            }
            let _ = self.moves.try_push(mv);
        }
    }

    pub fn extend_from_slice(&mut self, moves_slice: &[Move]) {
        for &mv in moves_slice {
            if self.is_full() {
                break;
            }
            let _ = self.moves.try_push(mv);
        }
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }

    pub fn as_slice(&self) -> &[Move] {
        self.moves.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        self.moves.as_mut_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }

    pub fn get_index(&self, index: usize) -> Option<&Move> {
        self.moves.get(index)
    }
}
