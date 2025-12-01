use super::data::*;
use chess_core::types::{board::board::Board, color::Color, piece::PieceType};


// Make sure this constant matches your engine's max phase (usually 24 or 128)
const PHASE_SCALE: i32 = 24; 

pub fn material_eval(board: &Board, phase: i32) -> i32 {
    let mut score_mg: i32 = 0;
    let mut score_eg: i32 = 0;

    // 1. Cache the color bitboards (avoid fetching from memory 12 times)
    let w = board.colors[Color::White as usize];
    let b = board.colors[Color::Black as usize];

    // --- Index 0: PAWNS ---
    // Calculate counts once to use for both MG and EG
    let w_pawns = (board.pieces[0] & w).count() as i32;
    let b_pawns = (board.pieces[0] & b).count() as i32;
    
    score_mg += w_pawns * PIECE_VALUES_MG[0];
    score_mg -= b_pawns * PIECE_VALUES_MG[0];
    score_eg += w_pawns * PIECE_VALUES_EG[0];
    score_eg -= b_pawns * PIECE_VALUES_EG[0];

    // --- Index 1: KNIGHTS ---
    let w_knights = (board.pieces[1] & w).count() as i32;
    let b_knights = (board.pieces[1] & b).count() as i32;
    
    score_mg += w_knights * PIECE_VALUES_MG[1];
    score_mg -= b_knights * PIECE_VALUES_MG[1];
    score_eg += w_knights * PIECE_VALUES_EG[1];
    score_eg -= b_knights * PIECE_VALUES_EG[1];

    // --- Index 2: BISHOPS ---
    let w_bishops = (board.pieces[2] & w).count() as i32;
    let b_bishops = (board.pieces[2] & b).count() as i32;

    score_mg += w_bishops * PIECE_VALUES_MG[2];
    score_mg -= b_bishops * PIECE_VALUES_MG[2];
    score_eg += w_bishops * PIECE_VALUES_EG[2];
    score_eg -= b_bishops * PIECE_VALUES_EG[2];

    // --- Index 3: ROOKS ---
    let w_rooks = (board.pieces[3] & w).count() as i32;
    let b_rooks = (board.pieces[3] & b).count() as i32;

    score_mg += w_rooks * PIECE_VALUES_MG[3];
    score_mg -= b_rooks * PIECE_VALUES_MG[3];
    score_eg += w_rooks * PIECE_VALUES_EG[3];
    score_eg -= b_rooks * PIECE_VALUES_EG[3];

    // --- Index 4: QUEENS ---
    let w_queens = (board.pieces[4] & w).count() as i32;
    let b_queens = (board.pieces[4] & b).count() as i32;

    score_mg += w_queens * PIECE_VALUES_MG[4];
    score_mg -= b_queens * PIECE_VALUES_MG[4];
    score_eg += w_queens * PIECE_VALUES_EG[4];
    score_eg -= b_queens * PIECE_VALUES_EG[4];
    
    (score_mg * phase + score_eg * (PHASE_SCALE - phase)) / PHASE_SCALE
}
