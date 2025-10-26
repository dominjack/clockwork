use colored::Colorize;
use std::fmt;
use std::str::FromStr;
use arrayvec::ArrayVec;
use crate::types::bitboard::Bitboard;
use crate::types::piece::Piece;
use crate::types::board::internalstate::InternalState;
use crate::types::square::Square;
use crate::types::color::Color;
use super::transposition::ZOBRIST_KEYS;


#[derive(Debug)]
pub enum FenError {
    InvalidFormat,
    InvalidPiece,
    InvalidColor,
    InvalidCastling,
    InvalidEnPassant,
    InvalidHalfmoveClock,
    InvalidFullmoveNumber
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [Bitboard; Piece::COUNT],
    pub state: InternalState,
    pub history: Box<ArrayVec<InternalState, 512>>,
    pub colors: [Bitboard; Color::COUNT],
    pub mailbox: [Piece; Square::COUNT]
}


//#[allow(dead_code)]
impl Board {
    pub fn start() -> Self {
        return Self::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    fn empty() -> Self {
        Board {
            pieces: [Bitboard::new(0); Piece::COUNT],
            state: InternalState::new(),
            history: Box::new(ArrayVec::new()),
            colors: [Bitboard::new(0); Color::COUNT],
            mailbox: [Piece::None; Square::COUNT]
        }
    }

    pub fn occupied(&self) -> Bitboard{
        self.colors[0] | self.colors[1]
    }

    pub fn us(&self) -> Bitboard{
        self.colors[self.state.color as usize]
    }
    
    pub fn them(&self) -> Bitboard{
        self.colors[self.state.color.invert() as usize]
    }

    pub fn fill_colors(&mut self) {
        let mut bitboard_white= Bitboard::new(0);
        for &piece_bb in self.pieces[0..6].iter() {
                bitboard_white |= piece_bb.0; 
            };
        let mut bitboard_black= Bitboard::new(0);
        for &piece_bb in self.pieces[6..12].iter() {
                bitboard_black |= piece_bb.0; 
            };
        self.colors[0] = bitboard_white;
        self.colors[1] = bitboard_black;
    }
        
    pub fn get_piece_on_square(&self, square: &Square) -> Piece {
        self.mailbox[square.to_index() as usize]
    }

    pub fn get_color_blockers(&self, color: Color) -> Bitboard {
        let mut combined_bitboard= Bitboard::new(0);

        match color {
            Color::White => { 
                for &piece_bb in self.pieces[0..6].iter() {
                combined_bitboard |= piece_bb.0; 
            } },
            Color::Black => { 
                for &piece_bb in self.pieces[6..12].iter() {
                combined_bitboard |= piece_bb.0; 
            } },
            _ => {}
        }
        combined_bitboard
    }

    pub fn set_piece(&mut self, piece: Piece, position: u8) {
        self.pieces[piece as usize].set_bit(position);
        self.mailbox[position as usize] = piece;
        self.fill_colors();
    }

    pub fn clear_piece(&mut self, piece: Piece, position: u8) {
        self.pieces[piece as usize].clear_bit(position);
        self.mailbox[position as usize] = Piece::None;
        self.fill_colors();
    }

    pub fn is_set(&self, piece: Piece, position: u8) -> bool {
        self.pieces[piece as usize].is_set(position)
    }

    pub fn is_square_set(&self, piece: Piece, sq: &Square) -> bool {
        self.pieces[piece as usize].is_set(sq.to_index())
    }

    
    pub fn update_mailbox_from_pieces(&mut self) {
        for piece in 0..Piece::COUNT {
            let bb = self.pieces[piece];
            for pos in bb {
                self.mailbox[pos as usize] = Piece::try_from(piece).unwrap_or(Piece::None);
            }
        }
    }
    

    pub fn check_castle_rights(&mut self, from: u8, to: u8, p: Piece){
        match p {
            Piece::WhiteKing => {
                if self.state.castling.white_king() {
                    self.state.castling.remove_white_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
                }
                if self.state.castling.white_queen() {
                    self.state.castling.remove_white_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
                }
            },
            Piece::BlackKing => {
                if self.state.castling.black_king() {
                    self.state.castling.remove_black_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
                }
                if self.state.castling.black_queen() {
                    self.state.castling.remove_black_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
                }
            },
            Piece::WhiteRook => {
                if from == 0 && self.state.castling.white_queen() {
                    self.state.castling.remove_white_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
                }
                else if from == 7 && self.state.castling.white_king(){
                    self.state.castling.remove_white_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
                }
            },
            Piece::BlackRook => {
                if from == 56 && self.state.castling.black_queen(){
                    self.state.castling.remove_black_queen();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
                }
                else if from == 63 && self.state.castling.black_king(){
                    self.state.castling.remove_black_king();
                    self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
                }
            },
             _ => {
                match to{
                    56 => {
                        if self.state.castling.black_queen() {
                            self.state.castling.remove_black_queen();
                            self.state.hash ^= ZOBRIST_KEYS.castling_keys[1];
                        }
                    },
                    63 => {
                        if self.state.castling.black_king() {
                            self.state.castling.remove_black_king();
                            self.state.hash ^= ZOBRIST_KEYS.castling_keys[0];
                        }
                    },
                    0 => {
                        if self.state.castling.white_queen() {
                            self.state.castling.remove_white_queen();
                            self.state.hash ^= ZOBRIST_KEYS.castling_keys[3];
                        }
                    },
                    7 => {
                        if self.state.castling.white_king() {
                            self.state.castling.remove_white_king();
                            self.state.hash ^= ZOBRIST_KEYS.castling_keys[2];
                        }
                    },
                    _ => {}
                }
                
             }
        }
    }

    pub fn is_legal(&self) -> bool {
        if self.state.color == Color::White {
            let sq_king = Square::new(self.pieces[Piece::BlackKing as usize].0.trailing_zeros() as u8);
            if self.is_attacked(&sq_king, Color::Black, None){
                return false;
            }
        }else if self.state.color == Color::Black {
            let sq_king = Square::new(self.pieces[Piece::WhiteKing as usize].0.trailing_zeros() as u8);
            if self.is_attacked(&sq_king, Color::White, None){
                return false;
            }
        }else {
            return false
        }
        true
    }


}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", (rank + 1).to_string().blue())?;
            for file in 0..8 {
                let position  = rank * 8 + file;
                let piece  = self.pieces.iter().position(|bb| bb.is_set(position));
                let s;
                match piece {
                    Some(p) => s = Piece::try_from(p).unwrap().to_string(),
                    None => s = String::from("."),
                }
                if s == "." {
                    if (file + rank + 1) % 2 == 0 {
                        write!(f, "{} ", s.white())?;
                    } else {
                        write!(f, "{} ", s.black())?;
                    }
                }  else if s.chars().next().map_or(false, |c| c.is_ascii_uppercase()) {
                    write!(f, "{} ", s.white().bold())?;
                } else {
                    write!(f, "{} ", s.black().bold())?;
                }
                
            }
            write!(f, "{}", self.state.color.to_string())?;
            writeln!(f)?;
        }
        writeln!(f, "{}", String::from("  a b c d e f g h").blue())?;

        Ok(())
    }

}

impl FromStr for Board {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        //https://de.wikipedia.org/wiki/Forsyth-Edwards-Notation
        let mut parts = fen.split_whitespace();
        let mut board = Board::empty();

        // First part fills board with pieces
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
                let piece = Piece::try_from(&piece).unwrap_or_default();
                if piece != Piece::None {
                    board.set_piece(piece, position);
                }
            }
        }

        //Second part creates game state
        //Color to move
        let color = parts.next().unwrap_or("n").try_into();
        match color {
            Ok(color) => {board.state.color = color;},
            Err(_) => {return Err(FenError::InvalidColor);}
        }
        //Castling rights
        let castling = parts.next().unwrap_or("-").try_into();
        match castling {
            Ok(castling) => {board.state.castling = castling;},
            Err(_) => {return Err(FenError::InvalidCastling);}
        }
        //En passant square
        let en_passant = parts.next().unwrap_or("-").try_into();
        match en_passant {
            Ok(en_passant) => {board.state.en_passant = en_passant;},
            Err(_) => {return Err(FenError::InvalidEnPassant);}
        }
        //Halfmove clock
        let halfmove_clock = parts.next().unwrap_or("0").parse();
        match halfmove_clock {
            Ok(halfmove_clock) => {board.state.halfmove_clock = halfmove_clock;},
            Err(_) => {return Err(FenError::InvalidHalfmoveClock);}
        }
        //Fullmove number
        let fullmove_number = parts.next().unwrap_or("0").parse();
        match fullmove_number {
            Ok(fullmove_number) => {board.state.fullmove_number = fullmove_number;},
            Err(_) => {return Err(FenError::InvalidFullmoveNumber);}
        }
        board.fill_colors();
        board.update_mailbox_from_pieces();

        Ok(board)
    }
}

impl Board {
    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(90); // Pre-allocate for typical FEN length

        // 1. Piece Placement
        // Iterate ranks from 8 down to 1 (FEN standard)
        // Internally, rank_idx_fen 7 corresponds to board rank 8, 0 to board rank 1.
        for rank_idx_fen in (0..8).rev() {
            let mut empty_squares_count = 0;
            // Iterate files from 'a' to 'h' (FEN standard)
            // Internally, file_idx 0 corresponds to 'a'-file, 7 to 'h'-file.
            for file_idx in 0..8 {
                // Calculate the square index (0-63) based on FEN rank and file
                let square_index = Square::new(rank_idx_fen * 8 + file_idx);
                let piece = self.get_piece_on_square(&square_index);

                if  piece!= Piece::None {
                    // If there were preceding empty squares, append their count
                    if empty_squares_count > 0 {
                        fen.push_str(&empty_squares_count.to_string());
                        empty_squares_count = 0;
                    }
                    fen.push(piece.to_char()); // Append the piece's FEN character
                } else {
                    empty_squares_count += 1; // Increment count of consecutive empty squares
                }
            }
            // If the rank ended with empty squares, append their count
            if empty_squares_count > 0 {
                fen.push_str(&empty_squares_count.to_string());
            }
            // Add '/' separator between ranks, except for the last one
            if rank_idx_fen > 0 {
                fen.push('/');
            }
        }

        // 2. Active Color
        fen.push(' ');
        fen.push(self.state.color.to_string());

        // 3. Castling Availability
        fen.push(' ');
        fen.push_str(&self.state.castling.to_fen_string());

        // 4. En Passant Target Square
        fen.push(' ');
        if let Some(ep_square) = self.state.en_passant.to_algebraic() {
            fen.push_str(ep_square.as_str());
        } else {
            fen.push('-'); // No en passant target
        }

        // 5. Halfmove Clock
        fen.push(' ');
        fen.push_str(&self.state.halfmove_clock.to_string());

        // 6. Fullmove Number
        fen.push(' ');
        fen.push_str(&self.state.fullmove_number.to_string());

        fen // Return the complete FEN string
    }
}










/// ######################################################
/// ################### TESTING ##########################
/// ######################################################


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::color::Color;
    use crate::types::square::Square;
    use crate::types::piece::Piece;


    struct FenTestCase {
        fen: &'static str,
        // Expected bitboards for all 12 piece types, in the order defined above
        expected_piece_bbs: [u64; 12], // Piece::COUNT should be 12
        expected_active_color: Color,
        expected_castling_raw: u8,
        expected_en_passant: Square,
        expected_halfmove: u8,
        expected_fullmove: usize,
    }

    const FEN_TEST_SUITE: &[FenTestCase] = &[
        FenTestCase {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            expected_piece_bbs: [
                0x000000000000FF00, // WP (Rank 2)
                0x0000000000000042, // WN (B1, G1)
                0x0000000000000024, // WB (C1, F1)
                0x0000000000000081, // WR (A1, H1)
                0x0000000000000008, // WQ (D1)
                0x0000000000000010, // WK (E1)
                0x00FF000000000000, // BP (Rank 7)
                0x4200000000000000, // BN (B8, G8)
                0x2400000000000000, // BB (C8, F8)
                0x8100000000000000, // BR (A8, H8)
                0x0800000000000000, // BQ (D8)
                0x1000000000000000, // BK (E8)
            ],
            expected_active_color: Color::White,
            expected_castling_raw: 15, // KQkq (binary 1111)
            expected_en_passant: Square::None, // Assuming Square::None exists
            expected_halfmove: 0,
            expected_fullmove: 1,
        },
        FenTestCase {
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            expected_piece_bbs: [
                0x000000001000EF00, // WP (A2-D2,F2-H2 + E4)
                0x0000000000000042, // WN (B1, G1)
                0x0000000000000024, // WB (C1, F1)
                0x0000000000000081, // WR (A1, H1)
                0x0000000000000008, // WQ (D1)
                0x0000000000000010, // WK (E1)
                0x00FF000000000000, // BP (Rank 7)
                0x4200000000000000, // BN (B8, G8)
                0x2400000000000000, // BB (C8, F8)
                0x8100000000000000, // BR (A8, H8)
                0x0800000000000000, // BQ (D8)
                0x1000000000000000, // BK (E8)
            ],
            expected_active_color: Color::Black,
            expected_castling_raw: 15, // KQkq
            expected_en_passant: Square::E3, // Assuming Square::E3 exists or its representation
            expected_halfmove: 0, // Pawn move resets halfmove clock
            expected_fullmove: 1, // Fullmove number increments after Black's move
        },
    ];

    #[test]
    fn test_fen_parsing() {
        for (i, case) in FEN_TEST_SUITE.iter().enumerate() {
            let board = case.fen.parse::<Board>();

            let context_msg = format!("Test case #{} (FEN: '{}')", i, case.fen);

            match board {
                Ok(board) => {
                    for piece_idx in 0..Piece::COUNT {
                        assert_eq!(
                            board.pieces[piece_idx],
                            case.expected_piece_bbs[piece_idx],
                            "Bitboard mismatch for piece index {}. {}", piece_idx, context_msg
                        );
                    }
                    assert_eq!(board.state.color, case.expected_active_color, "Active color mismatch. {}", context_msg);
                    assert_eq!(board.state.castling.0, case.expected_castling_raw, "Castling rights mismatch. {}", context_msg);
                    assert_eq!(board.state.en_passant, case.expected_en_passant, "En passant square mismatch. {}", context_msg);
                    assert_eq!(board.state.halfmove_clock, case.expected_halfmove, "Halfmove clock mismatch. {}", context_msg);
                    assert_eq!(board.state.fullmove_number, case.expected_fullmove, "Fullmove number mismatch. {}", context_msg);
                
                },
                Err(e) => panic!("Failed to parse FEN: {}. Error: {:?}", case.fen, e),
            }
            }
    }
}

