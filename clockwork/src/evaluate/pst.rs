use super::data::*;
use chess_core::types::{
    bitboard::Bitboard,
    board::board::Board,
    color::Color,
    piece::{Piece, PieceType},
};

fn game_state(board: &Board) {
    let mut phase = 0;
    phase += board.pieces[PieceType::Knight as usize].count() as u8 * KNIGHT_PHASE_VALUE;
    phase += board.pieces[PieceType::Bishop as usize].count() as u8 * BISHOP_PHASE_VALUE;
    phase += board.pieces[PieceType::Rook as usize].count() as u8 * ROOK_PHASE_VALUE;
    phase += board.pieces[PieceType::Queen as usize].count() as u8 * QUEEN_PHASE_VALUE;
    phase.clamp(0, PHASE_SCALE);
}

/// Calculates the piece-square table evaluation for the current board state.
pub fn evaluate_location(board: &Board) -> i32 {
    let mut score = 0;
    for num in 0..5 {
        for n in (board.pieces[num] & board.colors[Color::White as usize]) {
            score += PIECE_SQUARE_TABLES[num][invert(n as usize)]
        }
        for n in (board.pieces[num] & board.colors[Color::Black as usize]) {
            score -= PIECE_SQUARE_TABLES[num][n as usize]
        }
    }

    score
}
