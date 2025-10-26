use std::ops::BitOrAssign;
use std::ops::Sub;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use crate::types::square::Square;
use serde::{Serialize, Deserialize};

use std::ops::Shl;
use std::ops::Shr;



#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new(board: u64) -> Self {
        Bitboard(board)
    }

    pub fn rank(rank: u8) -> Self {
        Bitboard(0xFF << ((rank -1)* 8))
    }

    pub fn file(file: u8) -> Self {
        Bitboard(0b1000000010000000100000001000000010000000100000001 << (file -1))
    }


    pub fn single_bit_new(index: u8) -> Self {
        Bitboard(1 << index )
    }

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn toggle_bit(&mut self, index: u8) {
        self.0 ^= 1 << index;
    }

    pub fn is_set(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn lsb(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    pub fn is_edge(&self, index: u8) -> bool {
        index/8 == 0 || index/8 == 7 || index%8 == 0 || index%8 == 7
    }

    pub fn count_set(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn to_squares(&self) -> Vec<Square> {
        let mut squares = Vec::with_capacity(self.count_set() as usize); // Pre-allocate capacity
        let mut bits = self.0; // Get the u64 representation

        while bits != 0 {
            let index = bits.trailing_zeros() as u8;
            bits ^= 1 << index;
            squares.push(Square::new(index));
        }
        squares
    }

    pub fn pretty_print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let position = rank * 8 + file;
                if self.is_set(position) {
                    print!("X ");
                } else {
                    print!(". ");
                }
            }
            println!();
    }}

    pub fn shift(&self, bits: i8) -> Bitboard{
        if bits > 0{
            *self << bits
        }else{
            *self >> -bits
        }
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let lsb = self.lsb();
            self.0 &= self.0 - 1;
            Some(lsb)
        }
    }
}

impl Sub for Bitboard {
    type Output = Bitboard;

    fn sub(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0.wrapping_sub(rhs.0))
    }
}


impl std::ops::Mul for Bitboard {
    type Output = Bitboard;

    fn mul(self, rhs: Bitboard) -> Bitboard {
        Bitboard::new(self.0.wrapping_mul(rhs.0))
    }
}

impl std::ops::Mul<u64> for Bitboard {
    type Output = Bitboard;

    fn mul(self, rhs: u64) -> Bitboard {
        Bitboard::new(self.0.wrapping_mul(rhs))
    }
}


impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: u64) -> Bitboard {
        Bitboard(self.0 & rhs)
    }

}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard(!self.0)
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}


impl PartialEq<u64> for Bitboard {
    fn eq(&self, other_u64_value: &u64) -> bool {
        self.0 == *other_u64_value
    }
}

impl Shl<i8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: i8) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl Shr<i8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: i8) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}


