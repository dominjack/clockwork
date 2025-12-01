use crate::search::score::Score;
use chess_core::types::board::board::Board;
use chess_core::types::color::Color;
use chess_core::types::piece::PieceType;
use super::data::*;

use super::material::material_eval;
use super::pst::evaluate_location;

pub fn game_phase(board: &Board) -> i32{
    let mut phase = 0i32;
    phase += board.pieces[PieceType::Knight as usize].count() as i32 * KNIGHT_PHASE_VALUE;
    phase += board.pieces[PieceType::Bishop as usize].count() as i32 * BISHOP_PHASE_VALUE;
    phase += board.pieces[PieceType::Rook as usize].count() as i32 * ROOK_PHASE_VALUE;
    phase += board.pieces[PieceType::Queen as usize].count() as i32 * QUEEN_PHASE_VALUE;
    phase.clamp(0, PHASE_SCALE);
    phase
}

pub fn color_eval(color: Color) -> i32{
    match color{
        Color::White => 12,
        Color::Black => -12,
        _ => panic!()
    }
}

pub fn evaluate_relative(board: &Board) -> Score {
    let phase = game_phase(board);
    let mut score: i32 = 0;
    score += color_eval(board.state.color);
    score += evaluate_location(board, phase);
    score += material_eval(board, phase);
    Score(score)
}

pub fn evaluate_for(board: &Board, color: Color) -> Score {
    match color {
        Color::White => evaluate_relative(board),
        Color::Black => -evaluate_relative(board),
        _ => panic!(),
    }
}
