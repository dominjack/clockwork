use super::board::Board;
use crate::types::color::Color;
use crate::types::piece::Piece;
use crate::types::square::Square;
use crate::types::moves::Move;
use crate::types::moves::MoveType;
use crate::types::board::transposition::ZOBRIST_KEYS;


impl Board{
    fn move_normal(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                self.clear_piece(piece, from);
                self.set_piece(piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                self.check_castle_rights(from, to, piece); //TODO 
                if piece == Piece::WhitePawn || piece == Piece::BlackPawn {
                    self.state.halfmove_clock = 0;
                }
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
            },
            
        }
    }

    fn move_capture(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                let p_to = self.get_piece_on_square(&Square::new(to));
                self.clear_piece(piece, from);
                self.clear_piece(p_to, to);
                self.set_piece(piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                self.check_castle_rights(from, to, piece); //TODO
                self.state.halfmove_clock = 0;
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[p_to as usize][to as usize];
            },
        }
    }

    fn move_double_push(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                self.clear_piece(piece, from);
                self.set_piece(piece, to);
                match piece {
                    Piece::WhitePawn => {
                        if self.state.en_passant != Square::None{
                            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                        }
                        self.state.en_passant = Square::new(to-8);
                        self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[((to-8) as usize)%8];
                        self.state.halfmove_clock = 0;
                    },
                    Piece::BlackPawn => {
                        if self.state.en_passant != Square::None{
                            self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                        }
                        self.state.en_passant = Square::new(to+8);
                        self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[((to+8) as usize)%8];
                        self.state.halfmove_clock = 0;
                    },
                    _ => {}
                };
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
            },
        }
    }

    fn move_king_castle(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                self.clear_piece(piece, from);
                self.set_piece(piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                if piece == Piece::WhiteKing {
                    self.clear_piece(Piece::WhiteRook, 7);
                    self.set_piece(Piece::WhiteRook, 5);
                    self.state.castling.remove_white_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
                    self.state.castling.remove_white_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
                } else {
                    self.clear_piece(Piece::BlackRook, 63);
                    self.set_piece(Piece::BlackRook, 61);
                    self.state.castling.remove_black_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
                    self.state.castling.remove_black_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
                }
                
                
            },
        }
    }

    fn move_queen_castle(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                self.clear_piece(piece, from);
                self.set_piece(piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                if piece == Piece::WhiteKing {
                    self.clear_piece(Piece::WhiteRook, 0);
                    self.set_piece(Piece::WhiteRook, 3);
                    self.state.castling.remove_white_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
                    self.state.castling.remove_white_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
                } else {
                    self.clear_piece(Piece::BlackRook, 56);
                    self.set_piece(Piece::BlackRook, 59);
                    self.state.castling.remove_black_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
                    self.state.castling.remove_black_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
                }
                
            },
        }
    }

    fn move_en_passant(&mut self, mv: &Move){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                match piece {
                    Piece::WhitePawn => {
                        self.clear_piece(self.get_piece_on_square(&Square::new(to-8)), to-8);
                        self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::BlackPawn as usize][(to-8) as usize];
                    },
                    Piece::BlackPawn => {
                        self.clear_piece(self.get_piece_on_square(&Square::new(to+8)), to+8);
                        self.state.hash ^= ZOBRIST_KEYS.piece_keys[Piece::WhitePawn as usize][(to+8) as usize];
                    },
                    _ => {}
                };
                self.clear_piece(piece, from);
                self.set_piece(piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                self.state.halfmove_clock = 0;
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][to as usize];
            },
        }
    }

    fn move_promotion(&mut self, mv: &Move, promotion_piece: Piece){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                self.clear_piece(piece, from);
                self.set_piece(promotion_piece, to);
                if self.state.en_passant != Square::None{
                    self.state.hash ^= ZOBRIST_KEYS.en_passant_keys[(self.state.en_passant as usize)%8];
                }
                self.state.en_passant = Square::None;
                self.state.halfmove_clock = 0;
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[promotion_piece as usize][to as usize];
            },
        }
    }

    fn move_promotion_capture(&mut self, mv: &Move, promotion_piece: Piece){
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(from));
        match piece {
            Piece::None => {},
            _ => {
                let p_to = self.get_piece_on_square(&Square::new(to));
                self.clear_piece(piece, from);
                self.clear_piece(p_to, to);
                self.set_piece(promotion_piece, to);
                self.check_castle_rights(from, to, piece); //TODO
                self.state.en_passant = Square::None;
                self.state.halfmove_clock = 0;
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[piece as usize][from as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[p_to as usize][to as usize];
                self.state.hash ^= ZOBRIST_KEYS.piece_keys[promotion_piece as usize][to as usize];

            },
        }
    }
     
    fn move_pieces(&mut self, mv: &Move) {
        let kind = mv.kind();
        let color = self.state.color;

        
        match kind {
            MoveType::Normal=>{self.move_normal(mv)},
            MoveType::DoublePush=>{self.move_double_push(mv)},
            MoveType::KingCastle=>{self.move_king_castle(mv)},
            MoveType::QueenCastle=>{self.move_queen_castle(mv);},
            MoveType::Capture=>{self.move_capture(mv);},
            MoveType::EnPassant=>{self.move_en_passant(mv)},
            MoveType::KPromotion => {self.move_promotion(mv, if color == Color::White { Piece::WhiteKnight } else { Piece::BlackKnight });},
            MoveType::BPromotion=>{self.move_promotion(mv, if color == Color::White { Piece::WhiteBishop } else { Piece::BlackBishop });},
            MoveType::RPromotion=>{self.move_promotion(mv, if color == Color::White { Piece::WhiteRook } else { Piece::BlackRook });},
            MoveType::QPromotion=>{self.move_promotion(mv, if color == Color::White { Piece::WhiteQueen } else { Piece::BlackQueen });},
            MoveType::KPromotionCapture => {self.move_promotion_capture(mv, if color == Color::White { Piece::WhiteKnight } else { Piece::BlackKnight });},
            MoveType::BPromotionCapture=>{self.move_promotion_capture(mv, if color == Color::White { Piece::WhiteBishop } else { Piece::BlackBishop });},
            MoveType::RPromotionCapture=>{self.move_promotion_capture(mv, if color == Color::White { Piece::WhiteRook } else { Piece::BlackRook });},
            MoveType::QPromotionCapture=>{self.move_promotion_capture(mv, if color == Color::White { Piece::WhiteQueen } else { Piece::BlackQueen });},
        }
        
    }

    pub fn apply_move(&mut self, mv: &Move){
        self.state.hash_history[self.state.halfmove_clock as usize % 100] = self.hash();
        match mv.kind(){
            MoveType::Capture | MoveType::BPromotionCapture | MoveType::QPromotionCapture | MoveType::KPromotionCapture |MoveType::RPromotionCapture =>{
                self.state.captured = self.get_piece_on_square(&Square::new(mv.to()))
            },
            MoveType::EnPassant =>{
                if self.state.color == Color::White{
                    self.state.captured = Piece::BlackPawn;
                } else if self.state.color == Color::Black{
                    self.state.captured = Piece::WhitePawn;
                }
            }
            _ =>{
                self.state.captured = Piece::None
            }

        }
        self.history.push(self.state);
        self.move_pieces(mv);
        self.fill_colors();
        self.state.halfmove_clock += 1;
        if self.state.color == Color::Black {
            self.state.fullmove_number += 1;
        }
        self.state.color = self.state.color.invert();
    }

    pub fn undo_move(&mut self, mv: &Move){
        let state = self.history.pop().unwrap();
        self.state =  state;
         
        let from = mv.from();
        let to = mv.to();
        let piece = self.get_piece_on_square(&Square::new(to));

        self.set_piece(piece, from);
        self.clear_piece(piece, to);
        let captured = state.captured;

        if captured != Piece::None {
            self.set_piece(captured, to);
        }

        match mv.kind(){
            MoveType::EnPassant =>{
                if state.color == Color::White{
                    self.clear_piece(Piece::BlackPawn, to);
                    self.set_piece(Piece::BlackPawn, to-8);
                } else if state.color == Color::Black{
                    self.clear_piece(Piece::WhitePawn, to);
                    self.set_piece(Piece::WhitePawn, to+8);
                }
            }
            MoveType::KingCastle =>{
                if state.color == Color::White{
                    self.set_piece(Piece::WhiteRook, 7);
                    self.clear_piece(Piece::WhiteRook, 5);
                } else if state.color == Color::Black{
                    self.clear_piece(Piece::BlackRook, 61);
                    self.set_piece(Piece::BlackRook, 63);
                }
            }
            MoveType::QueenCastle =>{
                if state.color == Color::White{
                    self.set_piece(Piece::WhiteRook, 0);
                    self.clear_piece(Piece::WhiteRook, 3);
                } else if state.color == Color::Black{
                    self.set_piece(Piece::BlackRook, 56);
                    self.clear_piece(Piece::BlackRook, 59);
                }
            }
            MoveType::QPromotionCapture | MoveType::QPromotion =>{
                if state.color == Color::White{
                    self.clear_piece(Piece::WhiteQueen, from);
                    self.set_piece(Piece::WhitePawn, from);
                }else if state.color == Color::Black{
                    self.clear_piece(Piece::BlackQueen, from);
                    self.set_piece(Piece::BlackPawn, from);
                }
            }
            MoveType::KPromotionCapture | MoveType::KPromotion =>{
                if state.color == Color::White{
                    self.clear_piece(Piece::WhiteKnight, from);
                    self.set_piece(Piece::WhitePawn, from);
                }else if state.color == Color::Black{
                    self.clear_piece(Piece::BlackKnight, from);
                    self.set_piece(Piece::BlackPawn, from);
                }
            }
            MoveType::BPromotionCapture | MoveType::BPromotion =>{
                if state.color == Color::White{
                    self.clear_piece(Piece::WhiteBishop, from);
                    self.set_piece(Piece::WhitePawn, from);
                }else if state.color == Color::Black{
                    self.clear_piece(Piece::BlackBishop, from);
                    self.set_piece(Piece::BlackPawn, from);
                }
            }
            MoveType::RPromotionCapture | MoveType::RPromotion =>{
                if state.color == Color::White{
                    self.clear_piece(Piece::WhiteRook, from);
                    self.set_piece(Piece::WhitePawn, from);
                }else if state.color == Color::Black{
                    self.clear_piece(Piece::BlackRook, from);
                    self.set_piece(Piece::BlackPawn, from);
                }
            }
            _ =>{}
        }
    }
}