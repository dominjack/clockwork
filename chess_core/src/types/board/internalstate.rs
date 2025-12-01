use crate::types::bitboard::Bitboard;
use crate::types::castling::Castling;
use crate::types::color::Color;
use crate::types::piece::Piece;
use crate::types::square::Square;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameState::WhiteWin => write!(f, "1-0"),
            GameState::BlackWin => write!(f, "0-1"),
            GameState::Draw => write!(f, "1/2-1/2"),
            GameState::InProgress => write!(f, "*"),
        }
    }
}

impl From<&str> for GameState {
    fn from(s: &str) -> Self {
        match s {
            "1-0" => GameState::WhiteWin,
            "0-1" => GameState::BlackWin,
            "1/2-1/2" => GameState::Draw,
            "*" => GameState::InProgress,
            _ => GameState::InProgress,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InternalState {
    pub hash: u64,
    pub color: Color,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: usize,
    pub checker: Bitboard,
    pub num_checker: Option<u8>,
    pub game_state: GameState,
    pub captured: Option<Piece>,
}

impl InternalState {
    pub fn new() -> Self {
        InternalState {
            hash: 0,
            color: Color::None,
            castling: Castling::new(0),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            checker: Bitboard(0),
            num_checker: None,
            game_state: GameState::InProgress,
            captured: None,
        }
    }
}
