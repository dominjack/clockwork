use super::data::*;
use chess_core::types::{
    bitboard::Bitboard,
    board::board::Board,
    color::Color,
    piece::{Piece, PieceType},
};

pub fn evaluate_location(board: &Board, phase: i32) -> i32 {
    let mut score_mg = 0i32;
    let mut score_eg = 0i32;
    for num in 0..6 {
        for n in (board.pieces[num] & board.colors[Color::White as usize]) {
            score_mg += PIECE_SQUARE_TABLES_MG[num][invert(n as usize)];
            score_eg += PIECE_SQUARE_TABLES_EG[num][invert(n as usize)];
        }
        for n in (board.pieces[num] & board.colors[Color::Black as usize]) {
            score_mg -= PIECE_SQUARE_TABLES_MG[num][n as usize];
            score_eg -= PIECE_SQUARE_TABLES_EG[num][n as usize];
        }
    }

    (score_eg * (PHASE_SCALE - phase) + (score_mg * phase)) / PHASE_SCALE
}
