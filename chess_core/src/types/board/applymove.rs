use super::board::Board;
use crate::types::bitboard::Bitboard;
use crate::types::board;
use crate::types::board::hash::ZOBRIST_KEYS;
use crate::types::color::Color;
use crate::types::moves::Move;
use crate::types::moves::MoveType;
use crate::types::piece::Piece;
use crate::types::piece::PieceType;
use crate::types::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct IllegalMoveError;

impl Board {
    fn move_normal(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        if self.get_piece_on_square(&Square::new(from)).is_none() {
            println!("{}", self)
        }
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        self.clear_piece(piece, from);
        self.set_piece(piece, to);
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        self.update_castle_rights(from, to, piece);
        if piece == Piece::WhitePawn || piece == Piece::BlackPawn {
            self.state.halfmove_clock = 0;
        }
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
    }

    fn move_capture(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        let p_to = self.get_piece_on_square(&Square::new(to)).unwrap();
        self.clear_piece(piece, from);
        self.clear_piece(p_to, to);
        self.set_piece(piece, to);
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        self.update_castle_rights(from, to, piece);
        self.state.halfmove_clock = 0;
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[p_to as usize][to as usize];
    }

    fn move_double_push(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        self.clear_piece(piece, from);
        self.set_piece(piece, to);
        match piece {
            Piece::WhitePawn => {
                if let Some(ep) = self.state.en_passant {
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
                }
                self.state.en_passant = Some(Square::new(to - 8));
                self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[((to - 8) as usize) % 8];
                self.state.halfmove_clock = 0;
            }
            Piece::BlackPawn => {
                if let Some(ep) = self.state.en_passant {
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
                }
                self.state.en_passant = Some(Square::new(to + 8));
                self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[((to + 8) as usize) % 8];
                self.state.halfmove_clock = 0;
            }
            _ => {}
        };
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
    }

    fn move_king_castle(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        self.clear_piece(piece, from);
        self.set_piece(piece, to);
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        if piece == Piece::WhiteKing {
            self.clear_piece(Piece::WhiteRook, 7);
            self.set_piece(Piece::WhiteRook, 5);
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::WhiteRook as usize][7];
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::WhiteRook as usize][5];
            self.state.castling.remove_white_king();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
            self.state.castling.remove_white_queen();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
        } else {
            self.clear_piece(Piece::BlackRook, 63);
            self.set_piece(Piece::BlackRook, 61);
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::BlackRook as usize][63];
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::BlackRook as usize][61];
            self.state.castling.remove_black_king();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
            self.state.castling.remove_black_queen();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
        }
    }

    fn move_queen_castle(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        self.clear_piece(piece, from);
        self.set_piece(piece, to);
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        if piece == Piece::WhiteKing {
            self.clear_piece(Piece::WhiteRook, 0);
            self.set_piece(Piece::WhiteRook, 3);
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::WhiteRook as usize][0];
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::WhiteRook as usize][3];
            self.state.castling.remove_white_king();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
            self.state.castling.remove_white_queen();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
        } else {
            self.clear_piece(Piece::BlackRook, 56);
            self.set_piece(Piece::BlackRook, 59);
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::BlackRook as usize][56];
            self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::BlackRook as usize][59];
            self.state.castling.remove_black_king();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
            self.state.castling.remove_black_queen();
            self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
        }
    }

    fn move_en_passant(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        match piece {
            Piece::WhitePawn => {
                self.clear_piece(
                    self.get_piece_on_square(&Square::new(to - 8)).unwrap(),
                    to - 8,
                );
                self.state.hash ^=
                    ZOBRIST_KEYS.piece_keys[Piece::BlackPawn as usize][(to - 8) as usize];
            }
            Piece::BlackPawn => {
                self.clear_piece(
                    self.get_piece_on_square(&Square::new(to + 8)).unwrap(),
                    to + 8,
                );
                self.state.hash ^=
                    ZOBRIST_KEYS.piece_keys[Piece::WhitePawn as usize][(to + 8) as usize];
            }
            _ => {}
        };
        self.clear_piece(piece, from);
        self.set_piece(piece, to);
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        self.state.halfmove_clock = 0;
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
    }

    fn move_promotion(&mut self, mv: &Move, promotion_piece: Piece) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        self.clear_piece(piece, from);
        self.set_piece(promotion_piece, to);
        self.update_castle_rights(from, to, piece);
        if let Some(ep) = self.state.en_passant {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(ep as usize) % 8];
        }
        self.state.en_passant = None;
        self.state.halfmove_clock = 0;
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[promotion_piece as usize][to as usize];
    }

    fn move_promotion_capture(&mut self, mv: &Move, promotion_piece: Piece) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from)).unwrap();
        let p_to = self.get_piece_on_square(&Square::new(to)).unwrap();
        self.clear_piece(piece, from);
        self.clear_piece(p_to, to);
        self.set_piece(promotion_piece, to);
        self.update_castle_rights(from, to, piece);
        self.state.en_passant = None;
        self.state.halfmove_clock = 0;
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[p_to as usize][to as usize];
        self.state.hash ^= ZOBRIST_KEYS.piece_keys[promotion_piece as usize][to as usize];
    }

    fn move_pieces(&mut self, mv: &Move) {
        let kind = mv.kind();
        let color = self.state.color;

        match kind {
            MoveType::Normal => self.move_normal(mv),
            MoveType::DoublePush => self.move_double_push(mv),
            MoveType::KingCastle => self.move_king_castle(mv),
            MoveType::QueenCastle => self.move_queen_castle(mv),
            MoveType::Capture => self.move_capture(mv),
            MoveType::EnPassant => self.move_en_passant(mv),
            MoveType::KPromotion => self.move_promotion(
                mv,
                if color == Color::White {
                    Piece::WhiteKnight
                } else {
                    Piece::BlackKnight
                },
            ),
            MoveType::BPromotion => self.move_promotion(
                mv,
                if color == Color::White {
                    Piece::WhiteBishop
                } else {
                    Piece::BlackBishop
                },
            ),
            MoveType::RPromotion => self.move_promotion(
                mv,
                if color == Color::White {
                    Piece::WhiteRook
                } else {
                    Piece::BlackRook
                },
            ),
            MoveType::QPromotion => self.move_promotion(
                mv,
                if color == Color::White {
                    Piece::WhiteQueen
                } else {
                    Piece::BlackQueen
                },
            ),
            MoveType::KPromotionCapture => self.move_promotion_capture(
                mv,
                if color == Color::White {
                    Piece::WhiteKnight
                } else {
                    Piece::BlackKnight
                },
            ),
            MoveType::BPromotionCapture => self.move_promotion_capture(
                mv,
                if color == Color::White {
                    Piece::WhiteBishop
                } else {
                    Piece::BlackBishop
                },
            ),
            MoveType::RPromotionCapture => self.move_promotion_capture(
                mv,
                if color == Color::White {
                    Piece::WhiteRook
                } else {
                    Piece::BlackRook
                },
            ),
            MoveType::QPromotionCapture => self.move_promotion_capture(
                mv,
                if color == Color::White {
                    Piece::WhiteQueen
                } else {
                    Piece::BlackQueen
                },
            ),
        }
    }

    pub fn apply_null_move(&mut self) -> Option<Square> {
        let en_passant = self.state.en_passant;
        if self.state.en_passant.is_some() {
            self.state.hash ^=
                ZOBRIST_KEYS.en_passant_keys[self.state.en_passant.unwrap() as usize % 8];
            self.state.en_passant = None;
        }
        self.state.color = self.state.color.invert();
        self.state.hash ^= ZOBRIST_KEYS.black_to_move_key;
        en_passant
    }

    pub fn undo_null_move(&mut self, en_passant: Option<Square>) {
        self.state.color = self.state.color.invert();
        self.state.hash ^= ZOBRIST_KEYS.black_to_move_key;
        if en_passant.is_some() {
            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[en_passant.unwrap() as usize % 8];
            self.state.en_passant = en_passant;
        }
    }

    pub fn apply_move(&mut self, mv: &Move) -> Result<(), IllegalMoveError> {
        // Save the current state for undo_move.
        self.history.push(self.state);
        match mv.kind() {
            MoveType::Capture
            | MoveType::BPromotionCapture
            | MoveType::QPromotionCapture
            | MoveType::KPromotionCapture
            | MoveType::RPromotionCapture => {
                self.state.captured = self.get_piece_on_square(&Square::new(mv.to()))
            }
            MoveType::EnPassant => {
                if self.state.color == Color::White {
                    self.state.captured = Some(Piece::BlackPawn);
                } else if self.state.color == Color::Black {
                    self.state.captured = Some(Piece::WhitePawn);
                }
            }
            _ => (self.state.captured = None),
        }
        // Apply the move.
        self.move_pieces(mv);
        // Update the clocks and color.
        self.state.halfmove_clock += 1;
        if self.state.color == Color::Black {
            self.state.fullmove_number += 1;
        }
        self.state.hash ^= ZOBRIST_KEYS.black_to_move_key;
        self.state.color = self.state.color.invert();
        self.state.checker = Bitboard(0);
        self.state.num_checker = None;

        let king = self.their(PieceType::King).lsb();
        if self.is_square_attacked(king, self.state.color) {
            self.undo_move(mv);
            return Err(IllegalMoveError);
        }
        Ok(())
    }

    pub fn undo_move(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let kind = mv.kind();

        let moved_piece = self.get_piece_on_square(&Square::new(to)).unwrap();

        let original_piece = if mv.is_promotion() {
            if self.state.color == Color::Black {
                Piece::WhitePawn
            } else {
                Piece::BlackPawn
            }
        } else {
            moved_piece
        };

        self.set_piece(original_piece, from);
        self.clear_piece(moved_piece, to);

        if mv.is_capture() {
            let captured = self.state.captured;
            if kind == MoveType::EnPassant {
                let captured_sq = if self.state.color == Color::Black {
                    to - 8
                } else {
                    to + 8
                };
                self.set_piece(captured.unwrap(), captured_sq);
            } else {
                self.set_piece(captured.unwrap(), to);
            }
        }

        self.state = self.history.pop().unwrap();

        // Handle castling
        if kind == MoveType::KingCastle {
            if self.state.color == Color::White {
                self.set_piece(Piece::WhiteRook, 7);
                self.clear_piece(Piece::WhiteRook, 5);
            } else {
                self.set_piece(Piece::BlackRook, 63);
                self.clear_piece(Piece::BlackRook, 61);
            }
        } else if kind == MoveType::QueenCastle {
            if self.state.color == Color::White {
                self.set_piece(Piece::WhiteRook, 0);
                self.clear_piece(Piece::WhiteRook, 3);
            } else {
                self.set_piece(Piece::BlackRook, 56);
                self.clear_piece(Piece::BlackRook, 59);
            }
        }
    }
}
