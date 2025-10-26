use crate::data::magics::{MagicEntry, BISHOP_MAGICS, ROOK_MAGICS};
use crate::data::attacks::{ROOK_ATTACKS, BISHOP_ATTACKS};
use crate::types::bitboard::Bitboard;
use crate::types::square::Square;
use crate::types::color::Color;



fn get_magic_index(magic: &MagicEntry, blockers: &Bitboard) -> usize{
    let index = *blockers & magic.mask;
    let _index = (index * magic.magic).0 >> magic.shift;
    _index as usize + magic.offset

}

pub fn lookup_bishop(sq: &Square, blockers:&Bitboard) -> Bitboard{
    unsafe {
        let magic = BISHOP_MAGICS.get_unchecked(*sq as usize);
        let index = get_magic_index(magic, blockers);
        Bitboard(*BISHOP_ATTACKS.get_unchecked(index))
    }
}

pub fn lookup_rook(sq: &Square, blockers:&Bitboard) -> Bitboard{
    unsafe {
        let magic = ROOK_MAGICS.get_unchecked(*sq as usize);
        let index = get_magic_index(magic, blockers);
        Bitboard(*ROOK_ATTACKS.get_unchecked(index))
    }
}

pub fn lookup_queen(sq: &Square, blockers:&Bitboard) -> Bitboard{
    lookup_bishop(sq, blockers) | lookup_rook(sq, blockers)
}

pub fn lookup_pawn_captures(sq: &Square, color: &Color) -> Bitboard {
    let mut targets = Bitboard::new(0);

    let index = sq.to_index();
    let start_file = (index % 8) as i8;
    if *color == Color::White && index < 56{
        if start_file > 0 {
            targets.set_bit(index + 7);
        }
        if start_file < 7 {
            targets.set_bit(index + 9);
        }
    } else if *color == Color::Black && index > 8{
        if start_file > 0 {
            targets.set_bit(index - 9);
        }
        if start_file < 7 {
            targets.set_bit(index - 7);
        }
    }
    targets
}

pub fn lookup_knight(sq: &Square) -> Bitboard{
    let mut moves_bb = Bitboard::new(0);

    let sq_ind = sq.to_index();
    
    let start_rank = (sq_ind / 8) as i8; // Convert to i8 for easier signed arithmetic
    let start_file = (sq_ind % 8) as i8;

    // Possible knight moves (row, file offsets)
    let moves: [(i8, i8); 8] = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];

    for (dr, df) in moves {
        let nr = start_rank as i8 + dr;
        let nf = start_file as i8 + df;
        // Check if the new square is on the board
        if nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
            moves_bb |= 1u64 << (nr * 8 + nf);
        }
    }

    moves_bb

}

pub fn lookup_king(sq: &Square) -> Bitboard {
    let mut moves_bb = Bitboard::new(0);
    let sq_ind = sq.to_index();

    let start_rank = (sq_ind / 8) as i8; // Convert to i8 for easier signed arithmetic
    let start_file = (sq_ind % 8) as i8;

    // Relative king moves: (delta_rank, delta_file)
    let king_deltas: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1), // South-West, South, South-East
        ( 0, -1),          ( 0, 1), // West, East
        ( 1, -1), ( 1, 0), ( 1, 1)  // North-West, North, North-East
    ];

    for (dr, df) in king_deltas.iter() {
        let target_rank = start_rank + dr;
        let target_file = start_file + df;

        // Check if the target square is on the board
        if target_rank >= 0 && target_rank < 8 && target_file >= 0 && target_file < 8 {
            let target_sq_ind = (target_rank as u8 * 8) + target_file as u8;
            moves_bb.set_bit(target_sq_ind);
        }
    }

    moves_bb
}