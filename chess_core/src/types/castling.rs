use super::bitboard::Bitboard;
use super::moves::Move;
use super::color::Color;
use crate::types::moves::MoveType;

#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
pub struct Castling(pub u8);

impl Castling{
    /// BIN      DEC       DESCRIPTION
    /// 0001       1       black king side
    /// 0010       2       black queen side
    /// 0100       4       white king side
    /// 1000       8       white queen side
    pub fn new(right: u8) -> Self{
        Castling(right)
    }

    pub fn remove_white_king(&mut self){
        self.0 &= 0b1011;
    }

    pub fn remove_white_queen(&mut self){
        self.0 &= 0b0111;
    }

    pub fn remove_black_king(&mut self){
        self.0 &= 0b1110;
    }

    pub fn remove_black_queen(&mut self){
        self.0 &= 0b1101;
    }

    pub fn white_king(&self) -> bool{
        if (self.0 << 5) >>7 !=0{
               return true
            }
        false
    }

    pub fn white_queen(&self) -> bool{
        if (self.0 << 4) >>7 !=0{
               return true
            }
        false
    }

    pub fn black_king(&self) -> bool{
        if (self.0 << 7) >>7 !=0{
               return true
            }
        false
    }

    pub fn black_queen(&self) -> bool{
        if (self.0 << 6) >>7 !=0{
               return true
            }
        false
    }

    pub fn to_fen_string(&self) -> String{
        let mut fen = String::new();
        if self.0 == 0{
            fen.push('-');
        }else{
            if self.white_queen() {
                fen.push('Q');
            }
            if self.white_king() {
                fen.push('K');
            }
            if self.black_queen() {
                fen.push('q');
            }
            if self.black_king() {
                fen.push('k');
            }
        }
        fen
    }

    pub fn get_castling_possibilities(&self, color: Color) -> Vec<(Bitboard, Move)> {
        if color == Color::White{
            match self.0 >> 2 {
                0 => vec![],
                1 => vec![(Bitboard::new(0b01110000), Move::new(4, 6, MoveType::KingCastle))],
                2 => vec![(Bitboard::new(0b00011100), Move::new(4, 2, MoveType::QueenCastle))],
                3 => vec![(Bitboard::new(0b01110000), Move::new(4, 6, MoveType::KingCastle)), (Bitboard::new(0b00011100), Move::new(4, 2, MoveType::QueenCastle))],
                _ => vec![],
            }
        }else if color == Color::Black{
            match (self.0 << 6) >> 6 {
                0 => vec![],
                1 => vec![(Bitboard::new(0b01110000 <<56), Move::new(60, 62, MoveType::KingCastle))],
                2 => vec![(Bitboard::new(0b00011100 <<56), Move::new(60, 58, MoveType::QueenCastle))],
                3 => vec![(Bitboard::new(0b01110000 <<56), Move::new(60, 62, MoveType::KingCastle)), (Bitboard::new(0b00011100 << 56), Move::new(60, 58, MoveType::QueenCastle))],
                _ => vec![],
            }
        } else{
            vec![]
        }
    }

}

impl TryFrom<&str> for Castling{
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error>{
        let mut right: u8 = 0;
        for c in s.chars(){
            match c{
                'Q' => right +=8,
                'K' => right +=4,
                'q' => right +=2,
                'k' => right +=1,
                _ => {},
            }
        }
        Ok(Castling(right))
    }
}