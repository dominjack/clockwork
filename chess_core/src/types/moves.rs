use std::mem;
use crate::types::square::Square;
use crate::types::board::board::Board;
use crate::types::lists::MoveList;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType{
    Normal=0,

    DoublePush=1,
    KingCastle=2,
    QueenCastle=3,
    Capture=4,
    EnPassant=5,

    KPromotion=8,
    BPromotion=9,
    RPromotion=10,
    QPromotion=11,

    KPromotionCapture=12,
    BPromotionCapture=13, 
    RPromotionCapture=14,
    QPromotionCapture=15,
}

impl Move{
    pub const NULL: Self = Self(0);

    pub fn new(from: u8, to: u8, kind: MoveType) -> Self {
        let mut move_value = 0 as u16;
        move_value |= (from as u16) << 6;
        move_value |= to as u16;
        move_value |= (kind as u16) << 12;
        Self(move_value)
    }

    pub fn new_from_squares(from: Square, to: Square, kind: MoveType) -> Self {
        let mut move_value = 0 as u16;
        move_value |= (from as u16) << 6;
        move_value |= to as u16;
        move_value |= (kind as u16) << 12;
        Self(move_value)
    }

    pub fn from(&self) -> u8 {
        ((self.0 >> 6) & 0x3F) as u8
    }
    pub fn to(&self) -> u8 {
        (self.0 & 0x3F) as u8
    }
    pub fn kind(&self) -> MoveType {
        unsafe { mem::transmute((self.0 >> 12) as u8)}
    }
    pub fn to_string(&self) -> String {
        let st = format!("{}{}", Square::new(self.from()).to_algebraic().unwrap_or_default(), Square::new(self.to()).to_algebraic().unwrap_or_default());
        st
    }

    pub fn is_promotion(&self) -> bool {
        if self.kind() as u8 > 7{
            return true
        }
        false
    }

    pub fn is_capture(&self) -> bool {
        if self.kind() as u8 >= 12 || self.kind() as u8 == 5 || self.kind() as u8 == 4{
            return true
        }
        false
    }

    pub fn generate_algebraic_notation(&self, board: &Board, movelist: &MoveList) -> String {
        // Notation without gamestate and checks
        let mut notation = String::new();
        let mut disambiguation = Vec::<Move>::new();
        let mv_piece = board.mailbox[self.from() as usize];
        let mv_to = self.to();
        for _mv in movelist.moves.iter() {
            if board.mailbox[_mv.from() as usize] == mv_piece && _mv.to() == mv_to && _mv != self {
                disambiguation.push(*_mv);
            };
        }
        if self.kind() == MoveType::KingCastle {
            return "O-O".to_string();
        }else if self.kind() == MoveType::QueenCastle {
            return "O-O-O".to_string();
        }
        let mut disambiguate_rank = false;
        let mut disambiguate_file = false;
        for mv in disambiguation.iter() {
            if Square::new(mv.from()).rank() != Square::new(self.from()).rank() {
                disambiguate_rank = true;
            }else if Square::new(mv.from()).file() != Square::new(self.from()).file() {
                disambiguate_file = true;
            }
        }
        if disambiguate_rank && disambiguate_file {
            notation.push_str(Square::new(self.from()).to_algebraic().unwrap().to_ascii_lowercase().as_str());
        }else if disambiguate_rank {
            notation.push((b'1' + Square::new(self.from()).rank()) as char);
        }else if disambiguate_file {
            notation.push((b'a' + Square::new(self.from()).file()) as char);
        }
        notation.push(mv_piece.to_char().to_ascii_uppercase());
        notation = notation.replace("P", "");
        if self.is_capture() {
            if notation.is_empty() {
                notation.push((b'a' + Square::new(self.from()).file()) as char);
            }
            notation.push('x');
        }
        notation.push_str(Square::new(self.to()).to_algebraic().unwrap().as_str());
        notation
    }
}


impl Move{
    pub fn from_lan(board: &Board, uci: &str) -> Self{
        let mut _board = board.clone();
        let mut promotion_str = "";

        let from = Square::try_from(&uci[0..2]).unwrap();
        let to = Square::try_from(&uci[2..4]).unwrap();

        // Check for and parse the promotion piece if it exists.
        if uci.len() == 5 {
            promotion_str = &uci[4..5];
        }


        let moves = _board.generate_all_moves();
        for mv in moves.iter(){
            if mv.to() == to.to_index() && mv.from() == from.to_index(){
                if promotion_str == ""{
                    return mv.clone();
                }else{
                    match promotion_str{
                        "q" => {
                            if mv.kind() ==  MoveType::QPromotion || mv.kind() ==  MoveType::QPromotionCapture{
                                return mv.clone();
                            }
                        },
                        "r" => {
                            if mv.kind() ==  MoveType::RPromotion || mv.kind() ==  MoveType::RPromotionCapture{
                                return mv.clone();
                            }
                        },
                        "b" => {
                            if mv.kind() ==  MoveType::BPromotion || mv.kind() ==  MoveType::BPromotionCapture{
                                return mv.clone();
                            }
                        },
                        "n" => {
                            if mv.kind() ==  MoveType::KPromotion || mv.kind() ==  MoveType::KPromotionCapture{
                                return mv.clone();
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        panic!()
    }


    pub fn to_lan(self) -> String{
        let mut promotion_str = "";

        match self.kind(){
            MoveType::QPromotion | MoveType::QPromotionCapture => {
                promotion_str = "q";
            },
            MoveType::RPromotion | MoveType::RPromotionCapture => {
                promotion_str = "r";
            },
            MoveType::BPromotion | MoveType::BPromotionCapture => {
                promotion_str = "b";
            },
            MoveType::KPromotion | MoveType::KPromotionCapture => {
                promotion_str = "n";
            },
            _ => {}

        }
        format!("{}{}", self.to_string(), promotion_str)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_move(){
        let m = Move::new(1, 2, MoveType::Normal);
        assert_eq!(m.from(), 1);
        assert_eq!(m.to(), 2);
        assert_eq!(m.kind(), MoveType::Normal);
    }
}