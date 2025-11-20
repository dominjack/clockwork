use crate::search::score::Score;
use chess_core::types::board::board::Board;
use chess_core::types::color::Color;

use super::material::material_eval;
use super::pst::evaluate_location;

/// Calculates the total evaluation of the board state from the perspective of the current player.
pub fn evaluate_relative(board: &Board) -> Score {
    let mut score: i32 = 0;
    score += evaluate_location(board);
    score += material_eval(board);
    Score(score)
}

pub fn evaluate_for(board: &Board, color: Color) -> Score {
    match color {
        Color::White => evaluate_relative(board),
        Color::Black => -evaluate_relative(board),
        _ => panic!(),
    }
}
