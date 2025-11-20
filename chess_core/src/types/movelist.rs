use crate::types::board::board::Board;
use crate::types::moves::{Move, MoveType};
use crate::types::piece::PieceType;
use arrayvec::ArrayVec;
use std::ops::{Deref, DerefMut};

/// The maximum number of moves possible in any given position.
const MAX_MOVES_IN_LIST: usize = 218;

/// A list of moves, implemented as a wrapper around `ArrayVec` for stack allocation.
#[derive(Debug, Clone)]
pub struct MoveList {
    pub moves: ArrayVec<Move, MAX_MOVES_IN_LIST>,
}

impl MoveList {
    /// Creates a new, empty `MoveList`.
    pub fn new() -> Self {
        MoveList {
            moves: ArrayVec::new(),
        }
    }

    /// Returns the maximum number of moves that can be stored in the list.
    pub fn capacity(&self) -> usize {
        MAX_MOVES_IN_LIST
    }

    /// Returns the number of moves in the list.
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Checks if the list is full.
    pub fn is_full(&self) -> bool {
        self.moves.is_full()
    }

    /// Tries to add a move to the list.
    /// Returns an error if the list is full.
    pub fn try_push(&mut self, mv: Move) -> Result<(), Move> {
        self.moves.try_push(mv).map_err(|e| e.element())
    }

    /// Adds a move to the list.
    /// Panics if the list is full.
    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv)
    }

    /// Extends the list with moves from another `MoveList`.
    pub fn extend_from_other(&mut self, other: &MoveList) {
        for &mv in other.moves.as_slice() {
            if self.is_full() {
                break;
            }
            let _ = self.moves.try_push(mv);
        }
    }

    /// Extends the list with moves from a slice.
    pub fn extend_from_slice(&mut self, moves_slice: &[Move]) {
        for &mv in moves_slice {
            if self.is_full() {
                break;
            }
            let _ = self.moves.try_push(mv);
        }
    }

    /// Clears the list.
    pub fn clear(&mut self) {
        self.moves.clear();
    }

    /// Returns a slice containing all the moves in the list.
    pub fn as_slice(&self) -> &[Move] {
        self.moves.as_slice()
    }

    /// Returns a mutable slice containing all the moves in the list.
    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        self.moves.as_mut_slice()
    }

    /// Returns an iterator over the moves in the list.
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }

    /// Gets a reference to the move at a given index.
    pub fn get_index(&self, index: usize) -> Option<&Move> {
        self.moves.get(index)
    }
}
