const PAWN_PST: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0, // Rank 1 (unreachable for pawns at start)
     5,  10,  10, -20, -20,  10,  10,   5, // Rank 2 (starting, some tension/penalty for blocked center pawns)
     5,  -5, -10,   0,   0, -10,  -5,   5, // Rank 3
     0,   0,   0,  20,  20,   0,   0,   0, // Rank 4 (strong center control)
     5,   5,  10,  25,  25,  10,   5,   5, // Rank 5 (advanced, good outposts)
    10,  10,  20,  30,  30,  20,  10,  10, // Rank 6 (very advanced)
    50,  50,  50,  50,  50,  50,  50,  50, // Rank 7 (powerful, near promotion)
     0,   0,   0,   0,   0,   0,   0,   0  // Rank 8 (promotion handled separately)
];

// --- Knight Piece-Square Table ---
const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30, // Strong central squares
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50
];

// --- Bishop Piece-Square Table ---
const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   5,   0,   0,   0,   0,   5, -10, // Slight preference for developing towards center
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -40, -10, -10, -40, -10, -20 // -40 for f1/c1, f8/c8 if bishop is trapped
];

// --- Rook Piece-Square Table ---
const ROOK_PST: [i32; 64] = [
     0,  0,  0,  5,  5,  0,  0,  0, // Central files slightly better
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     5, 10, 10, 10, 10, 10, 10,  5, // Rooks on 7th rank (or 2nd for opponent)
     0,  0,  0,  0,  0,  0,  0,  0
];

// --- Queen Piece-Square Table ---
const QUEEN_PST: [i32; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -10,   5,   5,   5,   5,   5,   0, -10,
      0,   0,   5,   5,   5,   5,   0,  -5,
     -5,   0,   5,   5,   5,   5,   0,  -5, // Slight preference for centralization
    -10,   0,   5,   5,   5,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20
];

// --- King Piece-Square Tables (Opening/Middlegame vs Endgame) ---
const KING_MG_PST: [i32; 64] = [ // Middlegame - prioritize safety
     20,  30,  10,   0,   0,  10,  30,  20, // Safety in castled position (g1/c1)
     20,  20,   0,   0,   0,   0,  20,  20,
    -10, -20, -20, -20, -20, -20, -20, -10,
    -20, -30, -30, -40, -40, -30, -30, -20, // Penalize king in center
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30
];

const KING_EG_PST: [i32; 64] = [ // Endgame - prioritize activity
    -50, -30, -30, -30, -30, -30, -30, -50,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30, // Centralize the king
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -50, -40, -30, -20, -20, -30, -40, -50
];

#[inline(always)]
pub fn flip_square(sq: usize) -> usize {
    sq ^ 56 // XOR with 0b111000, effectively (7-rank)*8 + file
}

pub struct PieceSquareTables {
    pub pawn: &'static [i32; 64],
    pub knight: &'static [i32; 64],
    pub bishop: &'static [i32; 64],
    pub rook: &'static [i32; 64],
    pub queen: &'static [i32; 64],
    pub king_mg: &'static [i32; 64],
    pub king_eg: &'static [i32; 64],
}

pub const PSTS: PieceSquareTables = PieceSquareTables {
    pawn: &PAWN_PST,
    knight: &KNIGHT_PST,
    bishop: &BISHOP_PST,
    rook: &ROOK_PST,
    queen: &QUEEN_PST,
    king_mg: &KING_MG_PST,
    king_eg: &KING_EG_PST,
};