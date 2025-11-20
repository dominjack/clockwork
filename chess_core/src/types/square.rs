/// Represents a square on the chessboard.
/// The enum variants are ordered from A1 to H8.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// The number of squares on the board.
    pub const COUNT: usize = 64;

    /// Creates a new `Square` from a `u8` value (0-63).
    pub fn new(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    /// Creates a new `Square` from a rank and file (0-7).
    pub fn from_rank_file(rank: u8, file: u8) -> Self {
        Self::new((rank << 3) | file)
    }

    /// Converts the square to its `u8` index (0-63).
    pub fn to_index(self) -> u8 {
        self as u8
    }

    /// Converts the square to its algebraic notation (e.g., "e4").
    pub fn to_algebraic(self) -> Option<String> {
        let idx = self.to_index();
        match idx {
            0..=63 => {
                let file_char = (b'a' + (idx % 8)) as char;
                let rank_char = (b'1' + (idx / 8)) as char;
                Some(format!("{}{}", file_char, rank_char))
            }
            _ => None,
        }
    }

    /// Shifts the square by a given amount.
    pub fn shift(&self, shift: i8) -> Self {
        let index = self.to_index();
        Square::new((index as i8 + shift) as u8)
    }

    /// Gets the rank of the square (1-8).
    pub fn rank(&self) -> u8 {
        (self.to_index() / 8 + 1) as u8
    }

    /// Gets the file of the square (1-8).
    pub fn file(&self) -> u8 {
        (self.to_index() % 8 + 1) as u8
    }

    /// Converts the square to a bitboard with a single bit set at the square's index.
    pub fn to_bitboard(self) -> crate::types::bitboard::Bitboard {
        crate::types::bitboard::Bitboard(1 << self.to_index())
    }

    pub fn from_str_optional(value: &str) -> Result<Option<Square>, ()> {
        match value {
            "-" => Ok(None),
            _ => Square::try_from(value).map(Some),
        }
    }
}

/// Parses a square from its algebraic notation (e.g., "e4").
impl TryFrom<&str> for Square {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.as_bytes() {
            [file @ b'a'..=b'h', rank @ b'1'..=b'8'] => {
                let rank = rank - b'1';
                let file = file - b'a';
                Ok(Self::from_rank_file(rank, file))
            }
            _ => Err(()),
        }
    }
}
