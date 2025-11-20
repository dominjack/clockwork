use super::board::Board;
use crate::types::color;
use crate::types::piece::Piece;
use crate::types::square::Square;
use colored::{Color, Colorize};
use std::fmt;
use std::str::FromStr;

/// An error that can occur when parsing a FEN string.
#[derive(Debug)]
pub enum FenError {
    /// The FEN string has an invalid format.
    InvalidFormat,
    /// The FEN string contains an invalid piece character.
    InvalidPiece,
    /// The FEN string contains an invalid color character.
    InvalidColor,
    /// The FEN string contains an invalid castling string.
    InvalidCastling,
    /// The FEN string contains an invalid en passant square.
    InvalidEnPassant,
    /// The FEN string contains an invalid halfmove clock value.
    InvalidHalfmoveClock,
    /// The FEN string contains an invalid fullmove number value.
    InvalidFullmoveNumber,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", (rank + 1).to_string().blue())?;
            for file in 0..8 {
                let position = rank * 8 + file;
                let piece = self.mailbox[position as usize];
                let s = match piece {
                    Some(_piece) => _piece.to_string(),
                    None => String::from("."),
                };
                if s == "." {
                    if (file + rank + 1) % 2 == 0 {
                        write!(f, "{} ", s.black())?;
                    } else {
                        write!(f, "{} ", s.white())?;
                    }
                } else if s.chars().next().map_or(false, |c| c.is_ascii_uppercase()) {
                    write!(f, "{} ", s.white().bold())?;
                } else {
                    write!(f, "{} ", s.black().bold())?;
                }
            }
            match self.state.color {
                color::Color::White => write!(f, "{}", "w".white().italic())?,
                color::Color::Black => write!(f, "{}", "b".black().italic())?,
                _ => (),
            }

            writeln!(f)?;
        }
        writeln!(f, "{}", String::from("  A B C D E F G H").blue())?;

        Ok(())
    }
}

/// Parses a FEN string and creates a `Board` from it.
impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let mut parts = fen.split_whitespace();
        let mut board = Board::empty();

        // Pieces
        let rows = parts.next().unwrap().split('/');
        for (rank, row) in rows.rev().enumerate() {
            let mut additional = 0 as usize;
            for (file, piece) in row.chars().enumerate() {
                let number = piece.to_digit(10);
                match number {
                    Some(n) => additional += n as usize - 1,
                    None => {}
                }
                let position = (rank * 8 + file + additional) as u8;
                let piece = Piece::try_from(&piece);
                if let Ok(_piece) = piece {
                    board.set_piece(_piece, position);
                }
            }
        }

        // Color
        let color = parts.next().unwrap_or("n").try_into();
        match color {
            Ok(color) => {
                board.state.color = color;
            }
            Err(_) => {
                return Err(FenError::InvalidColor);
            }
        }

        // Castling
        let castling = parts.next().unwrap_or("-").try_into();
        match castling {
            Ok(castling) => {
                board.state.castling = castling;
            }
            Err(_) => {
                return Err(FenError::InvalidCastling);
            }
        }

        // En passant
        let en_passant = Square::from_str_optional(parts.next().unwrap_or("-"));
        match en_passant {
            Ok(en_passant) => {
                board.state.en_passant = en_passant;
            }
            Err(_) => {
                return Err(FenError::InvalidEnPassant);
            }
        }

        // Halfmove clock
        let halfmove_clock = parts.next().unwrap_or("0").parse();
        match halfmove_clock {
            Ok(halfmove_clock) => {
                board.state.halfmove_clock = halfmove_clock;
            }
            Err(_) => {
                return Err(FenError::InvalidHalfmoveClock);
            }
        }

        // Fullmove clock
        let fullmove_number = parts.next().unwrap_or("0").parse();
        match fullmove_number {
            Ok(fullmove_number) => {
                board.state.fullmove_number = fullmove_number;
            }
            Err(_) => {
                return Err(FenError::InvalidFullmoveNumber);
            }
        }
        board.update_mailbox_from_pieces();
        board.state.hash = board.hash();

        Ok(board)
    }

    /// Converts the board to a FEN string.
    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(90);

        // Pieces
        for rank_idx_fen in (0..8).rev() {
            let mut empty_squares_count = 0;
            for file_idx in 0..8 {
                let square_index = Square::new(rank_idx_fen * 8 + file_idx);
                let piece = self.get_piece_on_square(&square_index);

                if let Some(_piece) = piece {
                    if empty_squares_count > 0 {
                        fen.push_str(&empty_squares_count.to_string());
                        empty_squares_count = 0;
                    }
                    fen.push(_piece.to_char());
                } else {
                    empty_squares_count += 1;
                }
            }
            if empty_squares_count > 0 {
                fen.push_str(&empty_squares_count.to_string());
            }
            if rank_idx_fen > 0 {
                fen.push('/');
            }
        }

        // Color
        fen.push(' ');
        fen.push(self.state.color.to_string());

        // Castling
        fen.push(' ');
        fen.push_str(&self.state.castling.to_fen_string());

        // En Passant
        fen.push(' ');
        if let Some(ep) = self.state.en_passant {
            let ep_square = ep.to_algebraic().unwrap();
            fen.push_str(ep_square.as_str());
        } else {
            fen.push('-');
        }

        // Halfmove Clock
        fen.push(' ');
        fen.push_str(&self.state.halfmove_clock.to_string());

        // Fullmove Number
        fen.push(' ');
        fen.push_str(&self.state.fullmove_number.to_string());

        fen
    }
}
