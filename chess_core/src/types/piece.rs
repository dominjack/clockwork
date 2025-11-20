use crate::types::color::Color;
use std::fmt;
use std::mem;

/// Represents a chess piece with its color.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

/// Represents a type of chess piece, without color.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const COUNT: usize = 6;
}

impl Piece {
    /// The number of distinct piece types for both colors.
    pub const COUNT: usize = 12;

    /// Gets the `PieceType` of the piece, ignoring its color.
    pub fn piece_type(&self) -> PieceType {
        if *self as usize <= 5 {
            unsafe { mem::transmute(*self as u8) }
        } else {
            unsafe { mem::transmute((*self as u8) - 6) }
        }
    }

    /// Gets the `Color` of the piece.
    pub fn color(&self) -> Color {
        if *self as usize <= 5 {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Converts the piece to its character representation (e.g., 'P' for white pawn, 'p' for black pawn).
    pub fn to_char(&self) -> char {
        match self {
            Piece::WhitePawn => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
        }
    }
}

/// Converts a `usize` to a `Piece`.
impl TryFrom<usize> for Piece {
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < Self::COUNT + 1 {
            Ok(unsafe { mem::transmute(value as u8) })
        } else {
            Err("Index out of bounds for Piece enum")
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Converts a `char` to a `Piece`.
impl TryFrom<&char> for Piece {
    type Error = &'static str;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            'P' => Ok(Piece::WhitePawn),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'R' => Ok(Piece::WhiteRook),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'p' => Ok(Piece::BlackPawn),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'r' => Ok(Piece::BlackRook),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            _ => Err("Invalid character for Piece"),
        }
    }
}
