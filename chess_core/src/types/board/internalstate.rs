use crate::types::color::Color;
use crate::types::square::Square;
use crate::types::castling::Castling;
use crate::types::piece::Piece;
use crate::types::bitboard::Bitboard;
use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState{
    InProgress,
    WhiteWin,
    BlackWin,
    Draw
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
            _ => GameState::InProgress, // Default for unrecognized results
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct InternalState {
    pub hash: u64,
    pub color: Color,
    pub castling: Castling,
    pub en_passant: Square,
    pub halfmove_clock: u8,
    pub fullmove_number: usize,
    pub captured: Piece,
    pub checker: Bitboard,
    pub pinned: Bitboard,
    pub pin_rays: [Bitboard; 64],
    pub num_checker: u8,
    pub game_state: GameState,
    pub hash_history: [u64; 100],
}

impl InternalState {
    pub fn new() -> Self {
        InternalState {
            hash: 0,
            color: Color::None,
            castling: Castling::new(0),
            en_passant: Square::None,
            halfmove_clock: 0,
            fullmove_number: 1,
            captured: Piece::None,
            checker: Bitboard(0),
            pinned: Bitboard(0),
            pin_rays: [Bitboard(0); 64],
            num_checker: 0,
            game_state: GameState::InProgress,
            hash_history: [0u64; 100],
        }
    }
}
