use chess_core::types::{moves::Move, square::Square};

use crate::search::timecontrol::{self, MAX_SEARCH_DEPTH};

const MAX_HISTORY: i32 = 2000;

pub struct Heuristics{
    pub history: HistoryHeuristic,
    pub killers: KillerHeuristic
}

impl Heuristics {
    pub fn new() -> Self{
        Self { 
            history: HistoryHeuristic::new(),
            killers: KillerHeuristic::new() 
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryHeuristic{
    pub table: [i16; Square::COUNT * Square::COUNT],
}

impl HistoryHeuristic {
    pub fn new() -> Self{
        HistoryHeuristic { table: [0; Square::COUNT * Square::COUNT] }
    }

    pub fn store(&mut self, mv: Move, depth: usize){
        let bonus = (depth * depth) as i32;
        let current_score = self.table[mv.squares_index()] as i32;
        let new_score = current_score + bonus - (current_score * bonus / MAX_HISTORY);
        self.table[mv.squares_index()] = new_score as i16;
    }

    pub fn probe(& self, mv: Move) -> Option<u16>{
        let probe = self.table[mv.squares_index()] as u16;
        if probe != 0{
            return Some(probe)
        }
        return None
    }
}

pub struct KillerHeuristic{
    pub table: [[Move; 2]; MAX_SEARCH_DEPTH],
}

impl KillerHeuristic{
    pub fn new() -> Self {
        Self{
            table: [[Move::NONE; 2]; MAX_SEARCH_DEPTH],
        }
    }

    pub fn store(&mut self, mv: Move, ply: usize) {
        if self.table[ply][0] == mv {
            return;
        }

        self.table[ply][1] = self.table[ply][0];
        self.table[ply][0] = mv;
    }

    pub fn probe(&self, mv: Move, ply: usize) -> Option<u16> {
        if self.table[ply][0] == mv {
            return Some(3500);
        }
        if self.table[ply][1] == mv {
            return Some(3250);
        }
        return None;
    }
    
    pub fn clear(&mut self) {
        self.table = [[Move::NONE; 2]; MAX_SEARCH_DEPTH];
    }
}
