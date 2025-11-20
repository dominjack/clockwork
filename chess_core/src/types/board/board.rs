use super::hash::ZOBRIST_KEYS;
use crate::types::bitboard::Bitboard;
use crate::types::board::internalstate::GameState;
use crate::types::board::internalstate::InternalState;
use crate::types::color::Color;
use crate::types::piece::{Piece, PieceType};
use crate::types::square::Square;
use arrayvec::ArrayVec;
use std::str::FromStr;

/// The main board representation for the chess engine.
/// It uses a bitboard representation for the pieces and a mailbox representation
/// for quick piece lookups.
#[derive(Debug, Clone)]
pub struct Board {
    /// An array of bitboards, one for each piece type.
    pub pieces: [Bitboard; PieceType::COUNT],
    /// The current state of the board (color to move, castling rights, etc.).
    pub state: InternalState,
    /// A history of previous board states, used for undoing moves.
    pub history: Box<ArrayVec<InternalState, 512>>,
    /// An array of bitboards, one for each color, representing all pieces of that color.
    pub colors: [Bitboard; Color::COUNT],
    /// A "mailbox" representation of the board, an array of 64 squares,
    /// each containing the piece on that square. This is used for quick piece lookups.
    pub mailbox: [Option<Piece>; Square::COUNT],
}

impl Board {
    /// Creates a new board from the standard starting position.
    pub fn start() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    /// Creates a new, empty board.
    pub fn empty() -> Self {
        Board {
            pieces: [Bitboard::new(0); PieceType::COUNT],
            state: InternalState::new(),
            history: Box::new(ArrayVec::new()),
            colors: [Bitboard::new(0); Color::COUNT],
            mailbox: [None; Square::COUNT],
        }
    }

    /// Returns a bitboard of all occupied squares.
    pub fn occupied(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }

    /// Returns a bitboard of all pieces for the current player.
    pub fn us(&self) -> Bitboard {
        self.colors[self.state.color as usize]
    }

    pub fn our(&self, piece: PieceType) -> Bitboard {
        self.pieces[piece as usize] & self.colors[self.state.color as usize]
    }

    /// Returns a bitboard of all pieces for the opponent.
    pub fn them(&self) -> Bitboard {
        self.colors[self.state.color.invert() as usize]
    }

    pub fn their(&self, piece: PieceType) -> Bitboard {
        self.pieces[piece as usize] & self.colors[self.state.color.invert() as usize]
    }

    pub fn from_color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    /// Adds a piece of a given color to a square on the color bitboard.
    pub fn add_color_square(&mut self, position: u8, color: &Color) {
        match color {
            Color::White => self.colors[0] |= 1u64 << position,
            Color::Black => self.colors[1] |= 1u64 << position,
            _ => {}
        }
    }

    /// Removes a piece of a given color from a square on the color bitboard.
    pub fn remove_color_square(&mut self, position: u8, color: &Color) {
        match color {
            Color::White => self.colors[0] &= !(1u64 << position),
            Color::Black => self.colors[1] &= !(1u64 << position),
            _ => {}
        }
    }

    /// Gets the piece on a given square from the mailbox.
    pub fn get_piece_on_square(&self, square: &Square) -> Option<Piece> {
        self.mailbox[square.to_index() as usize]
    }

    /// Places a piece on a given square on the board.
    pub fn set_piece(&mut self, piece: Piece, position: u8) {
        self.pieces[piece.piece_type() as usize].set_bit(position);
        self.mailbox[position as usize] = Some(piece);
        self.add_color_square(position, &piece.color());
    }

    /// Removes a piece from a given square on the board.
    pub fn clear_piece(&mut self, piece: Piece, position: u8) {
        self.pieces[piece.piece_type() as usize].clear_bit(position);
        self.mailbox[position as usize] = None;
        self.remove_color_square(position, &piece.color());
    }

    /// Updates the mailbox representation from the piece bitboards.
    pub fn update_mailbox_from_pieces(&mut self) {
        self.mailbox = [None; Square::COUNT];
        for piece in 0..PieceType::COUNT {
            let bb = self.pieces[piece];
            let _bb = bb & self.colors[Color::White as usize];
            for pos in _bb {
                match Piece::try_from(piece) {
                    Ok(_piece) => self.mailbox[pos as usize] = Some(_piece),
                    Err(_) => {}
                }
            }
            let _bb = bb & self.colors[Color::Black as usize];
            for pos in _bb {
                match Piece::try_from(piece + 6) {
                    Ok(_piece) => self.mailbox[pos as usize] = Some(_piece),
                    Err(_) => {}
                }
            }
        }
    }

    pub fn is_in_check(&self) -> bool {
        let king = self.our(PieceType::King).lsb();
        self.is_square_attacked(king, self.state.color.invert())
    }

    pub fn check_draw_wo_stalemate(&self) -> bool {
        if self.state.halfmove_clock >= 100 {
            return true;
        }

        let hash = self.state.hash;
        let repetition = self.history.iter().filter(|h| h.hash == hash).count();

        if repetition > 2 {
            return true;
        }

        return false;
    }

    /// Updates the castling rights based on a move.
    pub fn update_castle_rights(&mut self, from: u8, to: u8, p: Piece) {
        // 1. Handle King moves
        // If a king moves, it loses *all* its castling rights.
        match p {
            Piece::WhiteKing => {
                self.state.castling.remove_white_king();
                self.state.castling.remove_white_queen();
            }
            Piece::BlackKing => {
                self.state.castling.remove_black_king();
                self.state.castling.remove_black_queen();
            }
            _ => {} // Not a king move
        }

        // 2. Handle moves FROM a rook's starting square
        // If a rook moves from its home, that side's rights are lost.
        match Square::new(from) {
            Square::A1 => self.state.castling.remove_white_queen(),
            Square::H1 => self.state.castling.remove_white_king(),
            Square::A8 => self.state.castling.remove_black_queen(),
            Square::H8 => self.state.castling.remove_black_king(),
            _ => {} // Move didn't start from a rook corner
        }

        // 3. Handle moves TO a rook's starting square
        // If a piece captures a rook on its home square, that side's rights are lost.
        match Square::new(to) {
            Square::A1 => self.state.castling.remove_white_queen(),
            Square::H1 => self.state.castling.remove_white_king(),
            Square::A8 => self.state.castling.remove_black_queen(),
            Square::H8 => self.state.castling.remove_black_king(),
            _ => {} // Move didn't start from a rook corner
        }
    }
}
