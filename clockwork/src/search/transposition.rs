use chess_core::types::board::board::Board;
use chess_core::types::color::Color;
use chess_core::types::moves::Move;
use chess_core::types::piece::Piece;
use chess_core::types::square::Square;

use crate::search::config::SearchParams;
use crate::search::score::Score;

#[derive(Debug, Clone)]
pub struct ZobristKeys {
    pub piece_keys: [[u64; 64]; Piece::COUNT],
    pub black_to_move_key: u64,
    pub castling_keys: [u64; 4],
    pub en_passant_keys: [u64; 8],
}

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

#[derive(Debug, Clone, Copy)]
pub struct TTEntry {
    pub age: u8,
    pub zobrist_hash: u64,
    pub best_move: Move,
    pub score: Score,
    pub depth: u8,
    pub flag: TableEntryFlag,
}

impl TTEntry {
    pub fn empty() -> Self {
        TTEntry {
            age: 0,
            zobrist_hash: 0,
            best_move: Move::NONE,
            score: Score::INVALID,
            depth: 0,
            flag: TableEntryFlag::None,
        }
    }

    pub fn get_score(&self, params: &SearchParams) -> Option<Score> {
        if self.depth < params.depth as u8 {
            return None;
        }

        match self.flag {
            TableEntryFlag::Pv => Some(self.score),
            TableEntryFlag::Cut if self.score >= params.beta => Some(params.beta),
            TableEntryFlag::All if self.score <= params.alpha => Some(params.alpha),
            _ => None,
        }
    }

    pub fn adjust_mate_score(&mut self, ply: i32){
        if self.score.is_mating(){
            self.score += ply
        }else if self.score.is_getting_mated(){
            self.score -= ply
        }
    }
}

// NOTE: This struct replaces both TTShard and StripedTranspositionTable
pub struct TranspositionTable {
    pub age: u8,
    pub table: Vec<TTEntry>,
    pub mask: usize,
}

impl TranspositionTable {
    pub fn new(total_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<TTEntry>();
        let mut num_entries = (total_mb * 1024 * 1024) / entry_size;

        num_entries = if num_entries == 0 {
            0
        } else {
            1 << (num_entries.ilog2())
        };

        if num_entries == 0 {
            return TranspositionTable {
                age: 0,
                table: Vec::new(),
                mask: 0,
            };
        }

        TranspositionTable {
            age: 0,
            table: vec![TTEntry::empty(); num_entries],
            mask: num_entries - 1,
        }
    }

    pub fn age(&mut self){
        if self.age < u8::MAX{
            self.age += 1;
        }else{
            self.age = 0;
        }
    }

    #[inline]
    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }

    pub fn store(
        &mut self,
        zobrist_hash: u64,
        best_move: Move,
        score: Score,
        depth: u8,
        flag: TableEntryFlag,
        ply: usize,
    ) {
        if self.table.is_empty() || best_move == Move::NONE || score == Score::INVALID || score == -Score::INVALID {
            return;
        }

        let index = self.get_index(zobrist_hash);
        let table_entry = unsafe { self.table.get_unchecked_mut(index) };

        let entry = TTEntry {
            age: self.age,
            zobrist_hash,
            best_move,
            score,
            depth,
            flag,
        };

        // Standard replacement strategy:
        // Only replace if the new entry is from a deeper search,
        // or if the existing slot is empty.
        if table_entry.flag == TableEntryFlag::None
            || table_entry.age != entry.age
            || entry.depth >= table_entry.depth
        {
            //entry.adjust_mate_score(-(ply as i32));
            *table_entry = entry;
        }
    }

    pub fn probe(&self, zobrist_hash: u64, ply: usize) -> Option<TTEntry> {
        if self.table.is_empty() {
            return None;
        }

        let index = self.get_index(zobrist_hash);
        let mut entry = unsafe { *self.table.get_unchecked(index) };

        // Verify the hash matches (to avoid collisions)
        // and that the entry has enough depth to be useful.
        if entry.zobrist_hash == zobrist_hash && entry.flag != TableEntryFlag::None {
            //entry.adjust_mate_score(ply as i32);
            Some(entry)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = TTEntry::empty();
        }
    }
}
