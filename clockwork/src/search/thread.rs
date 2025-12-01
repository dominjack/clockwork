use std::{
    iter::Inspect,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use chess_core::types::{board::board::Board, moves::Move};

use crate::search::{
    heuristics::Heuristics, timecontrol::TimeControl, transposition::TranspositionTable, variation::Variation
};

pub struct SearchThread {
    pub tc: TimeControl,
    pub terminator: Arc<AtomicBool>,
    pub tt: Arc<Mutex<TranspositionTable>>,
    pub start_time: Instant,
    pub nodes: u32,
    pub current_depth: usize,
    pub seldepth: usize,
    pub heuristics: Heuristics,
}

impl SearchThread {
    pub fn new(
        tc: TimeControl,
        tt: Arc<Mutex<TranspositionTable>>,
        terminator: Arc<AtomicBool>,
    ) -> Self {
        return Self {
            tc,
            terminator,
            tt,
            start_time: Instant::now(),
            nodes: 0,
            current_depth: 0,
            seldepth: 0,
            heuristics: Heuristics::new(),
        };
    }

    pub fn get_terminator(&self) -> bool {
        self.terminator.load(Ordering::Relaxed)
    }

    pub fn set_terminator(&self, val: bool) {
        self.terminator.store(val, Ordering::Relaxed)
    }

    pub fn is_over(&self) -> bool {
        self.tc
            .is_over(self.current_depth, self.start_time.elapsed())
            || self.terminator.load(Ordering::Relaxed)
    }

    pub fn get_pv(&self, board: &mut Board, depth: usize, variation: &mut Variation) {
        if depth == 0 {
            return;
        }

        // Recursively fill the vector by going through the chain of moves in the TT
        if let Some(mv) = self.get_pv_move(board) {
            variation.push_last(mv);
            board.apply_move(&mv).unwrap();
            self.get_pv(board, depth - 1, variation);
            board.undo_move(&mv);
        }
    }

    pub fn get_pv_move(&self, board: &Board) -> Option<Move> {
        let binding = self.tt.lock().unwrap();
        let entry = binding.probe(board.state.hash, 0);
        entry.map(|e| e.best_move)
    }
}
