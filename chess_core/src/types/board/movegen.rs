use core::num;

use super::board::Board;
use super::lookup::{lookup_bishop, lookup_king, lookup_knight, lookup_queen, lookup_rook};
use crate::types::board::internalstate::GameState;
use crate::types::board::lookup::lookup_pawn_captures;
use crate::types::{
    bitboard::Bitboard,
    board,
    color::Color,
    movelist::MoveList,
    moves::{Move, MoveType},
    piece::{Piece, PieceType},
    square::Square,
};

const NOISY: u8 = 1;
const QUIET: u8 = 0;

impl Board {
    /// Generates all noisy (capture and promotion) moves.
    pub fn generate_noisy_moves(&mut self) -> MoveList {
        let mut list = MoveList::new();
        self.append_noisy_moves(&mut list);
        list
    }

    /// Generates all quiet (non-capture and non-promotion) moves.
    pub fn generate_quiet_moves(&mut self) -> MoveList {
        let mut list = MoveList::new();
        self.append_quiet_moves(&mut list);
        list
    }

    /// Generates all legal moves for the current position.
    pub fn generate_all_moves(&mut self) -> MoveList {
        let mut list = MoveList::new();
        self.append_all_moves(&mut list);
        list
    }

    /// Pre-calculates checker and pinner information for the current position.
    /// This is done once before generating moves to avoid redundant calculations.
    /// Pre-calculates checker and pinner information for the current position.
    /// This is done once before generating moves to avoid redundant calculations.
    fn prepare_move_generation(&mut self) {
        // Do not calculate the checkers again if they already exist. This needs to ba handled properly but leads to a 10% increase in perft nps.
        if self.state.num_checker.is_none() {
            let (checker, num_checker) = self.get_checker();
            self.state.checker = checker;
            self.state.num_checker = Some(num_checker);
        }
    }

    /// Orchestrates the generation of legal moves of a specific type (noisy or quiet).
    /// It uses a target_mask to handle check evasion, but does not check for pinners.
    /// Orchestrates the generation of legal moves of a specific type (noisy or quiet).
    /// It uses a target_mask to handle check evasion, but does not check for pinners.
    fn generate_moves<const TYPE: u8>(&mut self, list: &mut MoveList) {
        let occupancies = self.occupied();
        let mut target_mask = Bitboard::FULL;
        // If there is a single checker, the target_mask is the checker's square and the ray between the king and the checker.
        if self.state.num_checker.unwrap() == 1 {
            target_mask = self.state.checker.clone();
        // If there are multiple checkers, only king moves are legal.
        } else if self.state.num_checker.unwrap() > 1 {
            self.collect_king_moves::<TYPE, _>(list, |square| lookup_king(square));
            return;
        }

        self.collect_pawn_moves::<TYPE>(list, target_mask);
        self.collect_moves::<TYPE, _>(
            list,
            PieceType::Knight,
            |square| lookup_knight(square),
            target_mask,
        );
        self.collect_moves::<TYPE, _>(
            list,
            PieceType::Bishop,
            |square| lookup_bishop(square, occupancies),
            target_mask,
        );
        self.collect_moves::<TYPE, _>(
            list,
            PieceType::Rook,
            |square| lookup_rook(square, occupancies),
            target_mask,
        );
        self.collect_moves::<TYPE, _>(
            list,
            PieceType::Queen,
            |square| lookup_queen(square, occupancies),
            target_mask,
        );
        self.collect_king_moves::<TYPE, _>(list, |square| lookup_king(square));
        if TYPE == QUIET {
            self.collect_castling(list);
        }
    }

    /// Appends all legal moves to the given move list.
    pub fn append_all_moves(&mut self, list: &mut MoveList) {
        self.prepare_move_generation();
        self.generate_moves::<NOISY>(list);
        self.generate_moves::<QUIET>(list);
    }

    /// Appends all quiet moves to the given move list.
    pub fn append_quiet_moves(&mut self, list: &mut MoveList) {
        self.prepare_move_generation();
        self.generate_moves::<QUIET>(list);
    }

    /// Appends all noisy moves to the given move list.
    pub fn append_noisy_moves(&mut self, list: &mut MoveList) {
        self.prepare_move_generation();
        self.generate_moves::<NOISY>(list);
    }

    /// Collects legal moves for a given piece type, handling pins and checks.
    /// This is a generic function for all pieces except pawns.
    pub fn collect_moves<const TYPE: u8, T>(
        &self,
        list: &mut MoveList,
        piece: PieceType,
        generator: T,
        target_mask: Bitboard,
    ) where
        T: Fn(Square) -> Bitboard,
    {
        for from in self.our(piece) {
            match TYPE {
                NOISY => {
                    for to in generator(from) & !self.us() & target_mask & self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Capture);
                        list.push(mv);
                    }
                }
                QUIET => {
                    for to in generator(from) & !self.us() & target_mask & !self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Normal);
                        list.push(mv);
                    }
                }
                _ => {}
            }
        }
    }

    /// Collects legal moves for kings, handling pins and checks.
    pub fn collect_king_moves<const TYPE: u8, T>(&self, list: &mut MoveList, generator: T)
    where
        T: Fn(Square) -> Bitboard,
    {
        for from in self.our(PieceType::King) {
            match TYPE {
                NOISY => {
                    for to in generator(from) & !self.us() & self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Capture);
                        list.push(mv);
                    }
                }
                QUIET => {
                    for to in generator(from) & !self.us() & !self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Normal);
                        list.push(mv);
                    }
                }
                _ => {}
            }
        }
    }

    /// Collects all legal pawn moves (pushes, captures, promotions, en-passant).
    pub fn collect_pawn_moves<const TYPE: u8>(&self, list: &mut MoveList, target_mask: Bitboard) {
        let pawns = self.pieces[PieceType::Pawn as usize] & self.us();
        let before_promotion = match self.state.color {
            Color::White => Bitboard::rank(7),
            Color::Black => Bitboard::rank(2),
            _ => panic!(),
        };

        self.collect_pawn_pushes::<TYPE>(list, pawns, before_promotion, target_mask);
        if TYPE == NOISY {
            self.collect_pawn_captures::<NOISY>(list, pawns, before_promotion, target_mask);
            self.collect_en_passant_moves(list, pawns);
        }
    }

    /// Collects legal pawn pushes (single, double, and promotions).
    pub fn collect_pawn_pushes<const TYPE: u8>(
        &self,
        list: &mut MoveList,
        pawns: Bitboard,
        before_promotion: Bitboard,
        target_mask: Bitboard,
    ) {
        let (diff, double_push_rank) = match self.state.color {
            Color::White => (8, Bitboard::rank(3)),
            Color::Black => (-8i8, Bitboard::rank(6)),
            _ => (0, Bitboard::new(0)),
        };

        let free = !self.occupied();

        if TYPE == QUIET {
            let pushed = (pawns & !before_promotion.0).shift(diff) & free;
            let double = (pushed & double_push_rank).shift(diff) & free;

            for to in pushed & target_mask {
                let from = to.shift(-diff);
                let mv = Move::new_from_squares(from, to, MoveType::Normal);
                list.push(mv);
            }
            for to in double & target_mask {
                let from = to.shift(-2 * diff);
                let mv = Move::new_from_squares(from, to, MoveType::DoublePush);
                list.push(mv);
            }
        }

        let promotions = (pawns & before_promotion).shift(diff) & free;
        for to in promotions & target_mask {
            let from = to.shift(-diff);
            if TYPE == NOISY {
                let mv = Move::new_from_squares(from, to, MoveType::QPromotion);
                list.push(mv);
            }

            if TYPE == QUIET {
                list.push(Move::new_from_squares(from, to, MoveType::BPromotion));
                list.push(Move::new_from_squares(from, to, MoveType::RPromotion));
                list.push(Move::new_from_squares(from, to, MoveType::KPromotion));
            }
        }
    }

    /// Collects legal pawn captures (including promotion captures).
    fn collect_pawn_captures<const TYPE: u8>(
        &self,
        list: &mut MoveList,
        pawns: Bitboard,
        before_promotion: Bitboard,
        target_mask: Bitboard,
    ) {
        let promotions = pawns & before_promotion;
        for from in promotions {
            let captures = self.them() & lookup_pawn_captures(from, self.state.color) & target_mask;
            for to in captures {
                list.push(Move::new_from_squares(
                    from,
                    to,
                    MoveType::BPromotionCapture,
                ));
                list.push(Move::new_from_squares(
                    from,
                    to,
                    MoveType::KPromotionCapture,
                ));
                list.push(Move::new_from_squares(
                    from,
                    to,
                    MoveType::QPromotionCapture,
                ));
                list.push(Move::new_from_squares(
                    from,
                    to,
                    MoveType::RPromotionCapture,
                ));
            }
        }

        let non_promotions = pawns & !before_promotion;
        for from in non_promotions {
            let targets = self.them() & lookup_pawn_captures(from, self.state.color);
            let targets = targets & target_mask;
            for to in targets {
                list.push(Move::new_from_squares(from, to, MoveType::Capture));
            }
        }
    }

    /// Collects legal en-passant moves, including a check for discovered attacks.
    fn collect_en_passant_moves(&self, list: &mut MoveList, pawns: Bitboard) {
        if let Some(ep) = self.state.en_passant {
            let attacks = pawns & lookup_pawn_captures(ep, self.state.color.invert());
            let shift = if self.state.color == Color::White {
                8
            } else {
                -8
            };
            for sq in attacks {
                list.push(Move::new_from_squares(sq, ep, MoveType::EnPassant));
            }
        }
    }

    /// Checks if a square is attacked by the given color.
    /// The color is the color of the piece that would be on the square.
    pub fn is_square_attacked(&self, square: Square, color: Color) -> bool {
        let occupancies = self.them() | self.us();

        let bishop_queen =
            self.pieces[PieceType::Bishop as usize] | self.pieces[PieceType::Queen as usize];
        let rook_queen =
            self.pieces[PieceType::Rook as usize] | self.pieces[PieceType::Queen as usize];

        let possible_attackers: Bitboard = (lookup_king(square)
            & self.pieces[PieceType::King as usize])
            | (lookup_knight(square) & self.pieces[PieceType::Knight as usize])
            | (lookup_bishop(square, occupancies) & bishop_queen)
            | (lookup_rook(square, occupancies) & rook_queen)
            | (lookup_pawn_captures(square, color.invert())
                & self.pieces[PieceType::Pawn as usize]);

        !(possible_attackers & self.colors[color as usize]).is_empty()
    }

    /// Collects legal castling moves.
    pub fn collect_castling(&self, list: &mut MoveList) {
        let color = self.state.color;
        let possiblities = self.state.castling.get_castling_possibilities(color);
        let blockers = self.occupied();
        'outer: for (bb, mv) in possiblities {
            if (mv.kind() == MoveType::KingCastle && (bb & !(1 << 4) & !(1 << 60)) & blockers == 0
                || mv.kind() == MoveType::QueenCastle
                    && (bb & !(1 << 4) & !(1 << 60)) & blockers.0 == 0)
            {
                for sq in (bb | 1 << (mv.from())) & !(1 << 1) & !(1 << 57) {
                    if self.is_square_attacked(sq, color.invert()) {
                        continue 'outer;
                    }
                }
                list.push(mv);
            }
        }
    }

    /// Gets the bitboard of checking pieces and the number of checkers.
    /// Gets the bitboard of checking pieces and the number of checkers.
    pub fn get_checker(&self) -> (Bitboard, u8) {
        let mut num_checkers = 0u8;
        let mut checkers = Bitboard(0);
        let sq = (self.pieces[PieceType::King as usize] & self.us()).lsb();
        let blockers = self.occupied();

        let moves = lookup_bishop(sq, blockers);
        let checking_bb = (self.pieces[PieceType::Bishop as usize]
            | self.pieces[PieceType::Queen as usize])
            & self.them()
            & moves;
        if checking_bb != 0 {
            for asq in checking_bb {
                num_checkers += 1;
                let attacking_moves = lookup_bishop(asq, blockers);
                checkers |= moves.0 & attacking_moves.0;
                checkers.set_bit(asq.to_index());
            }
        }

        let moves = lookup_rook(sq, blockers);
        let checking_bb = (self.pieces[PieceType::Rook as usize]
            | self.pieces[PieceType::Rook as usize])
            & self.them()
            & moves;
        if checking_bb != 0 {
            for asq in checking_bb {
                num_checkers += 1;
                let attacking_moves = lookup_rook(asq, blockers);
                checkers |= moves.0 & attacking_moves.0;
                checkers.set_bit(asq.to_index());
            }
        }

        let moves = lookup_knight(sq);
        let checking_bb = (self.pieces[PieceType::Knight as usize]) & self.them() & moves;
        if checking_bb != 0 {
            for asq in checking_bb {
                num_checkers += 1;
                checkers.set_bit(asq.to_index());
            }
        }

        if self.state.color == Color::White && sq.to_index() < 56 {
            let moves = lookup_pawn_captures(sq, Color::White);
            let checking_bb = (self.pieces[PieceType::Pawn as usize]) & self.them() & moves;
            if checking_bb != 0 {
                for asq in checking_bb {
                    num_checkers += 1;
                    checkers.set_bit(asq.to_index());
                }
            }
        } else if self.state.color == Color::Black && sq.to_index() >= 8 {
            let moves = lookup_pawn_captures(sq, Color::Black);
            let checking_bb = (self.pieces[PieceType::Pawn as usize]) & self.them() & moves;
            if checking_bb != 0 {
                for asq in checking_bb {
                    num_checkers += 1;
                    checkers.set_bit(asq.to_index());
                }
            }
        }
        checkers.clear_bit(sq.to_index());
        return (checkers, num_checkers);
    }
}
