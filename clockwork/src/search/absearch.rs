use crate::search::{
    config::SearchParams,
    order::MoveOrder,
    quiescence::QSearch,
    score::Score,
    thread::SearchThread,
    transposition::{TTEntry, TableEntryFlag},
};
use chess_core::types::{
    board::board::Board,
    moves::{Move, MoveType},
    square::Square,
};
use std::{collections::btree_map::Entry, time::Instant};

pub struct ABSearch<'a> {
    start_time: Instant,
    board: &'a mut Board,
    thread: &'a mut SearchThread,
    ply: usize,
}

impl<'a> ABSearch<'a> {
    pub fn new(board: &'a mut Board, thread: &'a mut SearchThread) -> Self {
        return Self {
            start_time: Instant::now(),
            board,
            thread,
            ply: 0,
        };
    }

    pub fn search(&mut self, mut params: SearchParams) -> Score {
        if let Some(score) = self.check_time_limit() {
            return score;
        }

        if !self.ply == 0
            && let Some(score) = self.check_repetition()
        {
            return score;
        }

        let in_check = self.board.is_in_check();

        if in_check{
            params.depth += 1
        }        

        if let Some(score) = self.null_move_pruning(&params, in_check) {
            return score;
        }
        

        if let Some(score) = self.quiescence_search(&params) {
            return score;
        }

        self.thread.nodes += 1;

        if let Some(score) = self.probe_transposition_table(&params) {
            return score;
        }

        let mut best_move: Move = Move::NONE;
        let mut best_score: Score = -Score::INFINITY;
        let mut kind: TableEntryFlag = TableEntryFlag::All;

        let mut searched_nodes: u32 = 0;

        //let moves = self.board.generate_all_moves();
        let mut order = MoveOrder::absearch(self.board, self.ply, self.thread);

        while let Some(mv) = order.next() {
            if self.board.apply_move(&mv).is_err() {
                continue;
            }
            self.ply += 1;
            let score = self.negamax(searched_nodes, &params, mv, in_check);
            self.ply -= 1;
            self.board.undo_move(&mv);
            searched_nodes += 1;
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
            if score >= params.beta {
                self.store_transposition_table(mv, score, params.depth, TableEntryFlag::Cut);

                if mv.is_quiet() {
                    self.thread.heuristics.killers.store(mv, self.ply);
                }

                return params.beta;
            }
            if score > params.alpha {
                params.alpha = score;
                kind = TableEntryFlag::Pv;

                if mv.is_quiet() {
                    self.thread.heuristics.history.store(mv, params.depth as usize);
                }

            }
        }

        if let Some(score) = self.is_game_over(searched_nodes, in_check) {
            return score;
        }

        self.store_transposition_table(best_move, best_score, params.depth, kind);
        params.alpha
    }

    pub fn is_game_over(&self, searched_moves: u32, in_check: bool) -> Option<Score> {
        if searched_moves > 0 {
            return None;
        } else {
            if in_check {
                return Some(-Score::CHECKMATE + self.ply as i32);
            } else {
                return Some(Score::DRAW);
            }
        }
    }

    fn null_move_pruning(&mut self, params: &SearchParams, in_check: bool) -> Option<Score>{
        if params.depth >= 3 && !in_check{
            let ep = self.board.state.en_passant;
            self.board.apply_null_move();
            let score = -self.search(SearchParams {
                alpha: -params.beta,
                beta: -params.beta + 1,
                depth: params.depth - 3,
            });
            self.board.undo_null_move(ep);
            if score >= params.beta{
                return Some(params.beta)
            }
        }
        return None;
    }

    fn negamax(
        &mut self,
        searched_nodes: u32,
        params: &SearchParams,
        mv: Move,
        in_check: bool,
    ) -> Score {
        if searched_nodes < 4 {
            return -self.search(SearchParams {
                alpha: -params.beta,
                beta: -params.alpha,
                depth: params.depth - 1,
            });
        } else {
            let reduction = 2; 
            let is_deep = params.depth >= 3;
            let is_simple = mv.is_quiet() && !mv.is_promotion();
            let do_lmr = is_deep && is_simple && !in_check;
            if do_lmr {
                return -self.search(SearchParams {
                    alpha: -params.beta,
                    beta: -params.alpha,
                    depth: params.depth - reduction,
                });
            } else {
                let reduction = 1;
                return -self.search(SearchParams {
                    alpha: -params.beta,
                    beta: -params.alpha,
                    depth: params.depth - reduction,
                });
            }
        }
    }

    fn quiescence_search(&mut self, params: &SearchParams) -> Option<Score> {
        if params.depth == 0 {
            let mut qsearch = QSearch::new(self.board, self.thread);
            let result = Some(qsearch.search(
                SearchParams {
                    alpha: params.alpha,
                    beta: params.beta,
                    depth: 0,
                },
                self.ply,
            ));

            let q_seldepth = qsearch.seldepth;
            drop(qsearch);
            if q_seldepth > self.thread.seldepth {
                self.thread.seldepth = q_seldepth;
            }

            return result;
        }
        None
    }

    fn check_time_limit(&self) -> Option<Score> {
        if self.thread.is_over() {
            self.thread.set_terminator(true);
        }

        self.thread.get_terminator().then_some(Score::INVALID)
    }

    fn check_repetition(&self) -> Option<Score> {
        if self.board.check_draw_wo_stalemate() {
            return Some(Score::DRAW);
        }
        None
    }

    fn probe_transposition_table(&self, params: &SearchParams) -> Option<Score> {
        let hash = self.board.state.hash;
        let binding = self.thread.tt.lock().unwrap();
        let result = binding.probe(hash, self.ply);
        let score = result.and_then(|entry| entry.get_score(params));
        score
    }

    fn store_transposition_table(
        &mut self,
        mv: Move,
        score: Score,
        depth: u8,
        flag: TableEntryFlag,
    ) {
        let hash = self.board.state.hash;
        let mut binding = self.thread.tt.lock().unwrap();
        binding.store(hash, mv, score, depth, flag, self.ply);
    }
}
