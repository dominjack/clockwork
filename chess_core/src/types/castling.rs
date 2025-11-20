use super::bitboard::Bitboard;
use super::color::Color;
use super::moves::Move;
use crate::types::moves::MoveType;

/// Represents the castling rights for both players.
/// The rights are stored in a single byte, where each bit corresponds to a specific castling right.
#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
pub struct Castling(pub u8);

impl Castling {
    /// Creates a new `Castling` struct from a byte.
    /// The bits are interpreted as follows:
    /// - 0001 (1): black kingside
    /// - 0010 (2): black queenside
    /// - 0100 (4): white kingside
    /// - 1000 (8): white queenside
    pub fn new(right: u8) -> Self {
        Castling(right)
    }

    /// Removes the white kingside castling right.
    pub fn remove_white_king(&mut self) {
        self.0 &= 0b1011;
    }

    /// Removes the white queenside castling right.
    pub fn remove_white_queen(&mut self) {
        self.0 &= 0b0111;
    }

    /// Removes the black kingside castling right.
    pub fn remove_black_king(&mut self) {
        self.0 &= 0b1110;
    }

    /// Removes the black queenside castling right.
    pub fn remove_black_queen(&mut self) {
        self.0 &= 0b1101;
    }

    /// Checks if white has kingside castling rights.
    pub fn white_king(&self) -> bool {
        (self.0 & 0b0100) != 0
    }

    /// Checks if white has queenside castling rights.
    pub fn white_queen(&self) -> bool {
        (self.0 & 0b1000) != 0
    }

    /// Checks if black has kingside castling rights.
    pub fn black_king(&self) -> bool {
        (self.0 & 0b0001) != 0
    }

    /// Checks if black has queenside castling rights.
    pub fn black_queen(&self) -> bool {
        (self.0 & 0b0010) != 0
    }

    /// Converts the castling rights to a FEN string.
    pub fn to_fen_string(&self) -> String {
        let mut fen = String::new();
        if self.0 == 0 {
            fen.push('-');
        } else {
            if self.white_king() {
                fen.push('K');
            }
            if self.white_queen() {
                fen.push('Q');
            }
            if self.black_king() {
                fen.push('k');
            }
            if self.black_queen() {
                fen.push('q');
            }
        }
        fen
    }

    /// Gets the possible castling moves for a given color.
    /// Returns a vector of tuples, where each tuple contains a bitboard of the squares
    /// that must be empty for the castling to be legal, and the castling move itself.
    pub fn get_castling_possibilities(&self, color: Color) -> Vec<(Bitboard, Move)> {
        if color == Color::White {
            match self.0 >> 2 {
                0 => vec![],
                1 => vec![(
                    Bitboard::new(0b01100000),
                    Move::new(4, 6, MoveType::KingCastle),
                )],
                2 => vec![(
                    Bitboard::new(0b00001110),
                    Move::new(4, 2, MoveType::QueenCastle),
                )],
                3 => vec![
                    (
                        Bitboard::new(0b01100000),
                        Move::new(4, 6, MoveType::KingCastle),
                    ),
                    (
                        Bitboard::new(0b00001110),
                        Move::new(4, 2, MoveType::QueenCastle),
                    ),
                ],
                _ => vec![],
            }
        } else if color == Color::Black {
            match self.0 & 0b0011 {
                0 => vec![],
                1 => vec![(
                    Bitboard::new(0b01100000 << 56),
                    Move::new(60, 62, MoveType::KingCastle),
                )],
                2 => vec![(
                    Bitboard::new(0b00001110 << 56),
                    Move::new(60, 58, MoveType::QueenCastle),
                )],
                3 => vec![
                    (
                        Bitboard::new(0b01100000 << 56),
                        Move::new(60, 62, MoveType::KingCastle),
                    ),
                    (
                        Bitboard::new(0b00001110 << 56),
                        Move::new(60, 58, MoveType::QueenCastle),
                    ),
                ],
                _ => vec![],
            }
        } else {
            vec![]
        }
    }
}

/// Parses a FEN castling string and creates a `Castling` struct from it.
impl TryFrom<&str> for Castling {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut right: u8 = 0;
        for c in s.chars() {
            match c {
                'K' => right |= 0b0100,
                'Q' => right |= 0b1000,
                'k' => right |= 0b0001,
                'q' => right |= 0b0010,
                '-' => (),
                _ => return Err(()),
            }
        }
        Ok(Castling(right))
    }
}
