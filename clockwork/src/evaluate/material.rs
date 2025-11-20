use super::data::*;
use chess_core::types::{board::board::Board, color::Color, piece::PieceType};

/// Calculates the material evaluation for the current board state.
pub fn material_eval(board: &Board) -> i32 {
    let mut score: i32 = 0;
    score += (board.pieces[PieceType::Pawn as usize] & board.colors[Color::White as usize]).count()
        as i32
        * PAWN_VALUE;
    score += (board.pieces[PieceType::Knight as usize] & board.colors[Color::White as usize])
        .count() as i32
        * KNIGHT_VALUE;
    score += (board.pieces[PieceType::Bishop as usize] & board.colors[Color::White as usize])
        .count() as i32
        * BISHOP_VALUE;
    score += (board.pieces[PieceType::Rook as usize] & board.colors[Color::White as usize]).count()
        as i32
        * ROOK_VALUE;
    score += (board.pieces[PieceType::Queen as usize] & board.colors[Color::White as usize]).count()
        as i32
        * QUEEN_VALUE;

    score -= (board.pieces[PieceType::Pawn as usize] & board.colors[Color::Black as usize]).count()
        as i32
        * PAWN_VALUE;
    score -= (board.pieces[PieceType::Knight as usize] & board.colors[Color::Black as usize])
        .count() as i32
        * KNIGHT_VALUE;
    score -= (board.pieces[PieceType::Bishop as usize] & board.colors[Color::Black as usize])
        .count() as i32
        * BISHOP_VALUE;
    score -= (board.pieces[PieceType::Rook as usize] & board.colors[Color::Black as usize]).count()
        as i32
        * ROOK_VALUE;
    score -= (board.pieces[PieceType::Queen as usize] & board.colors[Color::Black as usize]).count()
        as i32
        * QUEEN_VALUE;

    score
}
