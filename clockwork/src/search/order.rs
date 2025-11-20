use std::cmp::Ordering;

use chess_core::types::{
    board::{self, board::Board, movegen},
    movelist::MoveList,
    moves::Move,
    piece::PieceType,
};

use crate::search::thread::SearchThread;

type OrderMap = Vec<(Move, u16)>;

const AB_METHODS: &[OrderMethod] = &[OrderMethod::Cache, OrderMethod::MvvLva];
const Q_METHODS: &[OrderMethod] = &[OrderMethod::MvvLva];

pub enum OrderMethod {
    Cache,
    MvvLva,
}

pub struct MoveOrder {
    pub map: OrderMap,
    pub index: usize,
}

impl MoveOrder {
    pub fn absearch(board: &mut Board, ply: usize, thread: &SearchThread) -> Self {
        let binding = thread.tt.lock().unwrap();
        let cached = binding
            .probe(board.state.hash, ply as u8)
            .map(|entry| entry.best_move);
        let moves = board.generate_all_moves();
        Self::build(cached, AB_METHODS, board, ply, thread, &moves)
    }

    pub fn qsearch(board: &mut Board, ply: usize, thread: &SearchThread) -> Self {
        let mut moves = board.generate_noisy_moves();
        if board.state.num_checker.unwrap() > 0 {
            moves = board.generate_all_moves();
        };
        Self::build(None, Q_METHODS, board, ply, thread, &moves)
    }

    pub fn build(
        cached: Option<Move>,
        methods: &[OrderMethod],
        board: &Board,
        ply: usize,
        thread: &SearchThread,
        moves: &MoveList,
    ) -> Self {
        let mut map: OrderMap = Vec::with_capacity(moves.len());
        for &mv in moves.iter() {
            let score = score(mv, cached, methods, board);
            map.push((mv, score));
        }
        Self { map, index: 0 }
    }

    /// Returns the next most rated `Move` or `None` if there are no moves left.
    pub fn next(&mut self) -> Option<Move> {
        if self.index == self.map.len() {
            return None;
        }

        // Compare the current move rating with all others and swap if it's lower
        for next in (self.index + 1)..self.map.len() {
            if self.map[self.index].1 < self.map[next].1 {
                self.map.swap(self.index, next);
            }
        }

        let best = self.map[self.index].0;
        self.index += 1;
        Some(best)
    }
}

/// Calculates a score for a move based on a set of ordering methods.
///
/// A higher score indicates a better move.
///
/// # Arguments
/// * `mv` - The move to score.
/// * `cached` - The best move from the transposition table, if available.
/// * `methods` - A slice of `OrderMethod` enums to use for scoring.
/// * `board` - The current board state.
///
/// # Returns
/// The score of the move.
pub fn score(mv: Move, cached: Option<Move>, methods: &[OrderMethod], board: &Board) -> u16 {
    let score = 0u16;
    for method in methods {
        match method {
            OrderMethod::Cache => {
                if Some(mv) == cached {
                    return TT_MOVE;
                }
            }
            OrderMethod::MvvLva => {
                if let Some(value) = mvvlva_score(mv, board) {
                    return value;
                }
            }
        }
    }
    score
}

fn mvvlva_score(mv: Move, board: &Board) -> Option<u16> {
    if !mv.is_capture() {
        return None;
    }

    let attacker = board
        .get_piece_on_square(&mv.from_sq())
        .unwrap()
        .piece_type();
    let victim = if mv.is_en_passant() {
        PieceType::Pawn
    } else {
        board.get_piece_on_square(&mv.to_sq()).unwrap().piece_type()
    };

    return Some(MVV_LVA[attacker as usize][victim as usize]);
}

const TT_MOVE: u16 = 2000;

/// Quiet killer move is rated below any capture move from MVV-LVA
const KILLER_MOVE: u16 = 1000;

/// Most Valuable Victim – Least Valuable Attacker heuristic table indexed by `[attacker][victim]`.
const MVV_LVA: [[u16; PieceType::COUNT]; PieceType::COUNT] = [
    [1015, 1025, 1035, 1045, 1055, 1065],
    [1014, 1024, 1034, 1044, 1054, 1064],
    [1013, 1023, 1033, 1043, 1053, 1063],
    [1012, 1022, 1032, 1042, 1052, 1062],
    [1011, 1021, 1031, 1041, 1051, 1061],
    [1010, 1020, 1030, 1040, 1050, 1060],
];
