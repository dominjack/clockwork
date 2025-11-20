use crate::types::bitboard::Bitboard;
use crate::types::castling::Castling;
use crate::types::color::Color;
use crate::types::piece::Piece;
use crate::types::square::Square;
use std::fmt;

/// Represents the state of the game (in progress, win, draw).
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

/// Represents the internal state of the chess board.
/// This struct is designed to be copied and stored in the history for undoing moves.
#[derive(Debug, Clone, Copy)]
pub struct InternalState {
    /// The Zobrist hash of the current board position.
    pub hash: u64,
    /// The color of the current player to move.
    pub color: Color,
    /// The castling rights for both players.
    pub castling: Castling,
    /// The en-passant square, if any.
    pub en_passant: Option<Square>,
    /// The halfmove clock, used for the fifty-move rule.
    pub halfmove_clock: u8,
    /// The fullmove number, which is incremented after each black move.
    pub fullmove_number: usize,
    /// The last captured piece, used for undoing moves.
    pub checker: Bitboard,
    /// The number of pieces that are currently checking the king.
    pub num_checker: Option<u8>,
    /// The current state of the game (in progress, win, draw).
    pub game_state: GameState,
    // The piece that was captured last
    pub captured: Option<Piece>,
}

impl InternalState {
    /// Creates a new `InternalState` with default values.
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
