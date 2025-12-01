use std::cmp::Ordering;

use chess_core::types::{
    board::{self, board::Board, movegen},
    movelist::MoveList,
    moves::Move,
    piece::PieceType,
};

use crate::search::thread::SearchThread;

type OrderMap = Vec<(Move, u16)>;

const AB_METHODS: &[OrderMethod] = &[OrderMethod::Cache, OrderMethod::MvvLva, OrderMethod::Killers, OrderMethod::History];
const Q_METHODS: &[OrderMethod] = &[OrderMethod::MvvLva];

pub enum OrderMethod {
    Cache,
    MvvLva,
    History,
    Killers
}

pub struct MoveOrder {
    pub map: OrderMap,
    pub index: usize,
}

impl MoveOrder {
    pub fn absearch(board: &mut Board, ply: usize, thread: &SearchThread) -> Self {
        let binding = thread.tt.lock().unwrap();
        let cached = binding
            .probe(board.state.hash, ply)
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
            let score = score(mv, cached, methods, board, thread, ply);
            map.push((mv, score));
        }
        map.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        Self { map, index: 0 }
    }

    pub fn next(&mut self) -> Option<Move> {
        if self.index >= self.map.len() {
            return None;
        }

        let best = self.map[self.index].0;
        self.index += 1;
        Some(best)
    }
}

pub fn score(mv: Move, cached: Option<Move>, methods: &[OrderMethod], board: &Board, thread: &SearchThread, ply:usize) -> u16 {
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
            OrderMethod::Killers => {
                //println!("Probing {}, {}", mv, ply);
                if let Some(value) = thread.heuristics.killers.probe(mv, ply) {
                    return value;
                }
            }
            OrderMethod::History => {
                if let Some(value) = thread.heuristics.history.probe(mv) {
                    return value;
                }
            }
        }
    }
    0
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

const TT_MOVE: u16 = 5000;


/// Most Valuable Victim – Least Valuable Attacker heuristic table indexed by `[attacker][victim]`.
const MVV_LVA: [[u16; PieceType::COUNT]; PieceType::COUNT] = [
    [4015, 4025, 4035, 4045, 4055, 4065],
    [1014, 4024, 4034, 4044, 4054, 4064],
    [1013, 1023, 4033, 4043, 4053, 4063],
    [1012, 1022, 1032, 4042, 4052, 4062],
    [1011, 1021, 1031, 1041, 4051, 4061],
    [1010, 1020, 1030, 1040, 1050, 4060],
];
