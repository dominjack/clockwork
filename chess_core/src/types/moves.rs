use crate::types::board::board::Board;
use crate::types::movelist::MoveList;
use crate::types::square::Square;
use core::fmt;
use std::mem;

/// Represents a chess move.
/// The move is encoded into a 16-bit integer.
/// - Bits 0-5: destination square (0-63)
/// - Bits 6-11: origin square (0-63)
/// - Bits 12-15: move type (see `MoveType` enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(pub u16);

/// Represents the type of a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal = 0,
    DoublePush = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Capture = 4,
    EnPassant = 5,
    KPromotion = 8,
    BPromotion = 9,
    RPromotion = 10,
    QPromotion = 11,
    KPromotionCapture = 12,
    BPromotionCapture = 13,
    RPromotionCapture = 14,
    QPromotionCapture = 15,
}

impl Move {
    /// A null move, used to represent no move.
    pub const NONE: Self = Self(0);

    /// Creates a new `Move` from an origin square, destination square, and move type.
    pub fn new(from: u8, to: u8, kind: MoveType) -> Self {
        let mut move_value = 0 as u16;
        move_value |= (from as u16) << 6;
        move_value |= to as u16;
        move_value |= (kind as u16) << 12;
        Self(move_value)
    }

    /// Creates a new `Move` from `Square` objects.
    pub fn new_from_squares(from: Square, to: Square, kind: MoveType) -> Self {
        Self::new(from as u8, to as u8, kind)
    }

    /// Gets the origin square of the move.
    pub fn from(&self) -> u8 {
        ((self.0 >> 6) & 0x3F) as u8
    }

    /// Gets the destination square of the move.
    pub fn to(&self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    /// Gets the destination square as a `Square` object.
    pub fn to_sq(&self) -> Square {
        Square::new(self.to())
    }

    /// Gets the origin square as a `Square` object.
    pub fn from_sq(&self) -> Square {
        Square::new(self.from())
    }

    /// Gets the type of the move.
    pub fn kind(&self) -> MoveType {
        unsafe { mem::transmute((self.0 >> 12) as u8) }
    }

    /// Converts the move to a simple string representation (e.g., "e2e4").
    pub fn to_string(&self) -> String {
        format!(
            "{}{}",
            Square::new(self.from()).to_algebraic().unwrap_or_default(),
            Square::new(self.to()).to_algebraic().unwrap_or_default()
        )
    }

    /// Checks if the move is a promotion.
    pub fn is_promotion(&self) -> bool {
        (self.0 >> 12) > 7
    }

    /// Checks if the move is a capture.
    pub fn is_capture(&self) -> bool {
        let kind = self.kind() as u8;
        kind >= 12 || kind == 5 || kind == 4
    }

    pub fn is_en_passant(&self) -> bool {
        let kind = self.kind() as u8;
        kind == 5
    }

    pub fn is_quiet(&self) -> bool {
        !self.is_capture() && !self.is_promotion()
    }
}

impl Move {
    /// Creates a `Move` from a LAN (Long Algebraic Notation) string (e.g., "e2e4", "g1f3", "a7a8q").
    pub fn from_lan(board: &Board, uci: &str) -> Self {
        let mut _board = board.clone();
        let mut promotion_str = "";

        let from = Square::try_from(&uci[0..2]).unwrap();
        let to = Square::try_from(&uci[2..4]).unwrap();

        if uci.len() == 5 {
            promotion_str = &uci[4..5];
        }

        let moves = _board.generate_all_moves();
        for mv in moves.iter() {
            if mv.to() == to.to_index() && mv.from() == from.to_index() {
                if promotion_str == "" {
                    return mv.clone();
                } else {
                    match promotion_str {
                        "q" => {
                            if mv.kind() == MoveType::QPromotion
                                || mv.kind() == MoveType::QPromotionCapture
                            {
                                return mv.clone();
                            }
                        }
                        "r" => {
                            if mv.kind() == MoveType::RPromotion
                                || mv.kind() == MoveType::RPromotionCapture
                            {
                                return mv.clone();
                            }
                        }
                        "b" => {
                            if mv.kind() == MoveType::BPromotion
                                || mv.kind() == MoveType::BPromotionCapture
                            {
                                return mv.clone();
                            }
                        }
                        "n" => {
                            if mv.kind() == MoveType::KPromotion
                                || mv.kind() == MoveType::KPromotionCapture
                            {
                                return mv.clone();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        panic!("Invalid move: {}", uci)
    }

    /// Converts the move to a LAN (Long Algebraic Notation) string.
    pub fn to_lan(self) -> String {
        let mut promotion_str = "";

        match self.kind() {
            MoveType::QPromotion | MoveType::QPromotionCapture => {
                promotion_str = "q";
            }
            MoveType::RPromotion | MoveType::RPromotionCapture => {
                promotion_str = "r";
            }
            MoveType::BPromotion | MoveType::BPromotionCapture => {
                promotion_str = "b";
            }
            MoveType::KPromotion | MoveType::KPromotionCapture => {
                promotion_str = "n";
            }
            _ => {}
        }
        format!("{}{}", self.to_string(), promotion_str)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.to_lan())?;
        Ok(())
    }
}
