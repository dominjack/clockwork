use crate::evaluate::evaluate::{evaluate_for, evaluate_relative};
use crate::search::{
    config::SearchParams,
    order::MoveOrder,
    score::Score,
    thread::SearchThread,
    transposition::{TTEntry, TableEntryFlag},
};
use chess_core::types::{
    board::{self, board::Board},
    color::Color,
    moves::Move,
};
use std::{collections::btree_map::Entry, time::Instant};

pub struct QSearch<'a> {
    board: &'a mut Board,
    thread: &'a mut SearchThread,
    pub seldepth: usize,
}

impl<'a> QSearch<'a> {
    pub fn new(board: &'a mut Board, thread: &'a mut SearchThread) -> Self {
        return Self {
            board,
            thread,
            seldepth: 0,
        };
    }

    /// Performs a quiescence search from the current board state.
    ///
    /// This search only considers captures and promotions to stabilize the evaluation
    /// and avoid the horizon effect. It's a simplified version of alpha-beta search
    /// that explores volatile positions until they become quiet.
    ///
    /// # Arguments
    /// * `params` - The alpha-beta search parameters (alpha, beta, depth).
    /// * `ply` - The current search depth.
    ///
    /// # Returns
    /// The best score found for the current player.
    pub fn search(&mut self, mut params: SearchParams, ply: usize) -> Score {
        if let Some(score) = self.stop_search() {
            return score;
        }

        if ply > self.seldepth {
            self.seldepth = ply
        }

        self.thread.nodes += 1;

        let evaluation = self.evaluate();

        if evaluation >= params.beta {
            return params.beta;
        }

        if evaluation > params.alpha {
            params.alpha = evaluation;
        }

        let mut order = MoveOrder::qsearch(self.board, ply, &self.thread);
        while let Some(mv) = order.next() {
            if self.board.apply_move(&mv).is_ok() {
                let score = -self.search(
                    SearchParams {
                        alpha: -params.beta,
                        beta: -params.alpha,
                        depth: 0,
                    },
                    ply + 1,
                );
                self.board.undo_move(&mv);

                if score >= params.beta {
                    return params.beta;
                }

                if score > params.alpha {
                    params.alpha = score;
                }
            }
        }

        params.alpha
    }

    /// Evaluates the current board state if there are no moves.
    fn evaluate(&self) -> Score {
        return evaluate_for(self.board, self.board.state.color);
    }

    /// Checks if the search should be terminated due to time constraints.
    fn stop_search(&self) -> Option<Score> {
        if self.thread.is_over() {
            self.thread.set_terminator(true);
        }

        self.thread.get_terminator().then_some(Score::INVALID)
    }
}
