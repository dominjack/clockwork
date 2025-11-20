// Removed: use std::sync::Mutex;

use chess_core::types::board::board::Board;
use chess_core::types::color::Color;
use chess_core::types::moves::Move;
use chess_core::types::piece::Piece;
use chess_core::types::square::Square;

use crate::search::config::SearchParams;
use crate::search::score::Score;

/// A struct containing the Zobrist keys for each possible feature of a chess position.
#[derive(Debug, Clone)]
pub struct ZobristKeys {
    pub piece_keys: [[u64; 64]; Piece::COUNT],
    pub black_to_move_key: u64,
    pub castling_keys: [u64; 4],
    pub en_passant_keys: [u64; 8],
}

/// The flag of a transposition table entry, indicating the type of score it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEntryFlag {
    All,
    Cut,
    Pv,
    None,
}

impl Default for TableEntryFlag {
    fn default() -> Self {
        TableEntryFlag::All
    }
}

/// An entry in the transposition table.
#[derive(Debug, Clone, Copy)]
pub struct TTEntry {
    pub zobrist_hash: u64,
    pub best_move: Move,
    pub score: Score,
    pub depth: u8,
    pub flag: TableEntryFlag,
}

impl TTEntry {
    pub fn empty() -> Self {
        TTEntry {
            zobrist_hash: 0,
            best_move: Move::NONE,
            score: Score::INVALID,
            depth: 0,
            flag: TableEntryFlag::None,
        }
    }

    /// Gets the score from the entry if it's valid for the current search parameters.
    /// Returns `Some(Score)` if a valid score can be returned, `None` otherwise.
    ///
    /// This function implements the logic for when to use a stored score from the
    /// transposition table, considering the depth and flags of the stored entry.
    pub fn get_score(&self, params: &SearchParams) -> Option<Score> {
        if self.depth < params.depth as u8 {
            return None;
        }

        match self.flag {
            // If the stored score is an exact value (PV node), it can always be used.
            TableEntryFlag::Pv => Some(self.score),
            // If the stored score is a lower bound (Cut node) and is greater than or equal to beta,
            // it causes a beta cutoff, so we can return beta.
            TableEntryFlag::Cut if self.score >= params.beta => Some(params.beta),
            // If the stored score is an upper bound (All node) and is less than or equal to alpha,
            // it's a fail-low, so we can return alpha.
            TableEntryFlag::All if self.score <= params.alpha => Some(params.alpha),
            // In all other cases, the stored score is not useful for the current search.
            _ => None,
        }
    }
}

// NOTE: This struct replaces both TTShard and StripedTranspositionTable
/// The transposition table, used to store previously searched positions.
/// This is a single-threaded implementation.
pub struct TranspositionTable {
    pub table: Vec<TTEntry>,
    pub mask: usize,
}

impl TranspositionTable {
    /// Creates a new transposition table with a given size in Megabytes.
    pub fn new(total_mb: usize) -> Self {
        // Calculate the total number of entries that fit in the allocated memory
        let entry_size = std::mem::size_of::<TTEntry>();
        let mut num_entries = (total_mb * 1024 * 1024) / entry_size;

        // Ensure num_entries is a power of 2 for bitwise masking
        num_entries = if num_entries == 0 {
            0
        } else {
            1 << (num_entries.ilog2())
        };

        if num_entries == 0 {
            return TranspositionTable {
                table: Vec::new(),
                mask: 0,
            };
        }

        TranspositionTable {
            table: vec![TTEntry::empty(); num_entries],
            mask: num_entries - 1,
        }
    }

    /// Gets the index into the table.
    #[inline]
    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }

    /// Stores an entry in the table. (Takes &mut self)
    pub fn store(&mut self, entry: TTEntry) {
        if self.table.is_empty() || entry.best_move == Move::NONE {
            return;
        }

        let index = self.get_index(entry.zobrist_hash);
        let table_entry = unsafe { self.table.get_unchecked_mut(index) };

        // Standard replacement strategy:
        // Only replace if the new entry is from a deeper search,
        // or if the existing slot is empty.
        if table_entry.flag == TableEntryFlag::None || entry.depth >= table_entry.depth {
            *table_entry = entry;
        }
    }

    /// Probes an entry in the table. (Takes &self)
    /// Returns a reference to the entry if found.
    pub fn probe(&self, zobrist_hash: u64, depth: u8) -> Option<&TTEntry> {
        if self.table.is_empty() {
            return None;
        }

        let index = self.get_index(zobrist_hash);
        let entry = unsafe { self.table.get_unchecked(index) };

        // Verify the hash matches (to avoid collisions)
        // and that the entry has enough depth to be useful.
        if entry.zobrist_hash == zobrist_hash && entry.flag != TableEntryFlag::None {
            Some(entry)
        } else {
            None
        }
    }

    /// Clears the table by resetting all entries to empty.
    /// This re-uses the existing table allocation.
    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = TTEntry::empty();
        }
    }
}
