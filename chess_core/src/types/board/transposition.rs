use crate::types::piece::Piece;
use crate::types::square::Square;
use crate::types::board::board::Board;
use crate::types::color::Color;
use crate::types::moves::Move;

include!(concat!(env!("OUT_DIR"), "/zobrist_keys.rs"));

impl Board{
    pub fn hash(&mut self) -> u64{
        if self.state.hash == 0{
            let mut hash = 0;
            for i in 0..Piece::COUNT{
                for j in self.pieces[i].to_squares(){
                    hash ^= ZOBRIST_KEYS.piece_keys[i][j as usize];
                }
            }
            if self.state.color == Color::Black{
                hash ^= ZOBRIST_KEYS.black_to_move_key;
            }
            if self.state.castling.black_king(){
                hash ^= ZOBRIST_KEYS.castling_keys[0];
            }
            if self.state.castling.black_queen(){
                hash ^= ZOBRIST_KEYS.castling_keys[1];
            }
            if self.state.castling.white_king(){
                hash ^= ZOBRIST_KEYS.castling_keys[2];
            }
            if self.state.castling.white_queen(){
                hash ^= ZOBRIST_KEYS.castling_keys[3];
            }
            if self.state.en_passant != Square::None{
                hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
            }
            self.state.hash = hash;
            hash
        }else{
            self.state.hash
        }
    }
}


#[derive(Debug, Clone)]
pub struct ZobristKeys {
    pub piece_keys: [[u64; 64]; Piece::COUNT],
    pub black_to_move_key: u64,
    pub castling_keys: [u64; 4],
    pub en_passant_keys: [u64; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEntryFlag {
    Exact,
    LowerBound, // Score is >= value (alpha)
    UpperBound, // Score is <= value (beta)
    None,       // Invalid or empty entry
}

impl Default for TableEntryFlag {
    fn default() -> Self {
        TableEntryFlag::None
    }
}

#[derive(Debug, Clone, Copy, Default,PartialEq)]
pub struct TableEntry {
    pub zobrist_hash: u64, // Full hash to detect collisions
    pub best_move: Move,   // Or Option<Move> if space is a concern and not always present
    pub score: i16,        // Score relative to the side being evaluated
    pub depth: i8,         // Remaining depth of the search that stored this
    pub flags: TableEntryFlag,

}

pub struct TranspositionTable {
    table: Vec<TableEntry>,
    mask: usize,
}

impl TranspositionTable{
    pub fn new(mb: usize) -> Self{
        let entry_size = std::mem::size_of::<TableEntry>();
        if entry_size == 0 {
             panic!("Transposition Table can not be 0 sized");
        }

        let max_entries_for_size = (mb * 1024 * 1024) / entry_size;

        let num_entries = if max_entries_for_size == 0 {
            0
        } else {
            1 << (max_entries_for_size.ilog2()) // floor to power of 2
        };

        if num_entries == 0 {
            println!(
                "Requested TT size {} MB is too small for even one entry (entry size: {} bytes). TT will be disabled (0 entries).",
                mb, entry_size
            );
            return TranspositionTable {
                table: Vec::new(),
                mask: 0
            };
        }
        println!(
            "Initializing Transposition Table: {:.2} MB, {} entries (entry size: {} bytes)",
            (num_entries * entry_size) as f64 / (1024.0 * 1024.0),
            num_entries,
            entry_size,
        );

        TranspositionTable {
            // Initialize with default (empty/invalid) entries
            table: vec![TableEntry::default(); num_entries],
            mask: num_entries - 1,
        }
    }

    #[inline]
    fn get_index(&self, hash: u64) -> usize {

        if self.table.is_empty() {
            return 0;
        }

        (hash as usize) & self.mask
    }

    pub fn store(
        &mut self,
        zobrist_hash: u64,
        best_move: Move,
        score: i16,
        depth: i8,
        flags: TableEntryFlag,
    ) {
        if self.table.is_empty() {
            return; // Cannot store in an empty table
        }
        let index = self.get_index(zobrist_hash);
        let entry = &mut self.table[index];

        if depth >= entry.depth || entry.flags == TableEntryFlag::None { // Prioritize deeper or overwrite empty
             *entry = TableEntry {
                zobrist_hash,
                best_move,
                score,
                depth,
                flags,
            };
        }
    }

    /// Probes the transposition table for a given Zobrist hash.
    pub fn probe(&self, zobrist_hash: u64) -> Option<&TableEntry> {
        if self.table.is_empty() {
            return None;
        }
        let index = self.get_index(zobrist_hash);
        let entry = &self.table[index];

        // Check if the entry is valid and if the full hash matches (to avoid collisions)
        if entry.flags != TableEntryFlag::None && entry.zobrist_hash == zobrist_hash {
            Some(entry)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        if !self.table.is_empty() {
            self.table = vec![TableEntry::default(); self.mask + 1];
        }
         println!("Transposition Table Cleared.");
    }

    pub fn num_entries(&self) -> usize {
        self.table.len()
    }

}

