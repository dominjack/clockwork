use crate::types::{bitboard::Bitboard, lists::MoveList, moves::{Move, MoveType}, piece::{Piece, PieceType}, square::Square, color::Color};
use crate::types::board::lookup::{lookup_pawn_captures};
use super::board::Board;
use super::lookup::{lookup_bishop, lookup_knight, lookup_rook, lookup_queen, lookup_king};
use crate::types::board::internalstate::GameState;

const ALL: u8 = 2;
const NOISY: u8 = 1;
const QUIET: u8 = 0;



impl Board{
    pub fn generate_noisy_moves(&mut self) -> MoveList {
        let mut list = MoveList::new();
        self.append_noisy_moves(&mut list);
        list
    }

    pub fn generate_all_moves(&mut self) -> MoveList {
        let mut list = MoveList::new();
        self.append_all_moves(&mut list);
        list
    }

    pub fn generate_attacking_mask(&mut self, color:Color) -> Bitboard {
        let mut mask = Bitboard(0);
        for (sq, p) in self.mailbox.iter().enumerate(){
            let _sq = Square::new(sq as u8);
            if p.color() == color {
                match p.piece_type() {
                    PieceType::Pawn => {
                        mask |= lookup_pawn_captures(&_sq, &color).0;
                    },
                    PieceType::Knight => {
                        mask |= lookup_knight(&_sq).0;
                    },
                    PieceType::Bishop => {
                        mask |= lookup_bishop(&_sq, &self.occupied()).0;
                    },
                    PieceType::Rook => {
                        mask |= lookup_rook(&_sq, &self.occupied()).0;
                    },
                    PieceType::Queen => {
                        mask |= lookup_queen(&_sq, &self.occupied()).0;
                    },
                    PieceType::King => {},
                    _ => {}
                }
            }
        }
        mask
    }
        

    /// Generates pseudo legal moves for the current position.
    fn generate_moves<const TYPE: u8>(&mut self, list: &mut MoveList) {
        let (checker, num_checker) = self.get_checker();
        self.state.checker = checker;
        self.state.num_checker = num_checker;
        let (pinned, pin_rays) = self.get_pinner();
        self.state.pinned = pinned;
        self.state.pin_rays = pin_rays;
        let occupancies = self.occupied();

        self.collect_pawn_moves::<TYPE>(list);
        if self.state.color == Color::White {

            self.collect_moves::<TYPE, _>(list, Piece::WhiteKnight, |square| lookup_knight(&square));
            self.collect_moves::<TYPE, _>(list, Piece::WhiteBishop, |square| lookup_bishop(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::WhiteRook, |square| lookup_rook(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::WhiteQueen, |square| lookup_queen(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::WhiteKing, |square| lookup_king(&square));

        }else if self.state.color == Color::Black {
            self.collect_moves::<TYPE, _>(list, Piece::BlackKnight, |square| lookup_knight(&square));
            self.collect_moves::<TYPE, _>(list, Piece::BlackBishop, |square| lookup_bishop(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::BlackRook, |square| lookup_rook(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::BlackQueen, |square| lookup_queen(&square, &occupancies));
            self.collect_moves::<TYPE, _>(list, Piece::BlackKing, |square| lookup_king(&square));

        }
        if TYPE == QUIET || TYPE == ALL{
            self.collect_castling(list);
        }

    }

    pub fn append_all_moves(&mut self, list: &mut MoveList) {
        //self.generate_moves::<ALL>(list);
        self.generate_moves::<NOISY>(list);
        self.generate_moves::<QUIET>(list);
        self.update_game_state(list);
    }

    pub fn append_quiet_moves(&mut self, list: &mut MoveList) {
        self.generate_moves::<QUIET>(list);
    }

    /// Generates only pseudo legal capture moves for the current position.
    pub fn append_noisy_moves(&mut self, list: &mut MoveList) {
        self.generate_moves::<NOISY>(list);
    }

    pub fn update_game_state(&mut self, list: &MoveList) {
        if list.is_empty(){
            if self.state.checker != 0{
                self.state.game_state = match self.state.color {
                    Color::White => GameState::BlackWin,
                    Color::Black => GameState::WhiteWin,
                    _ => {self.state.game_state},
                };
            }else{
                self.state.game_state = GameState::Draw;
            }
        }
        else{
            if self.state.halfmove_clock == 100 {
                self.state.game_state = GameState::Draw;
            }
        else{
            self.state.game_state = GameState::InProgress;
            }
        }
    }



    pub fn collect_moves<const TYPE: u8, T> (&self, list: &mut MoveList, piece: Piece, generator: T) 
    where T: Fn(Square) -> Bitboard, 
    {
        let is_king = piece == Piece::WhiteKing || piece == Piece::BlackKing;
        for from in self.pieces[piece as usize]{
        let targets = generator(from) & !self.us();
            match TYPE {
                ALL => {
                    for to in targets & !self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Normal);
                        if self.is_move_legal(&mv, is_king){
                            list.push(mv);
                        }
                    }
                    for to in targets & self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Capture);
                        if self.is_move_legal(&mv, is_king){
                            list.push(mv);
                        }
                    }
                },
                NOISY => {
                    for to in targets & self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Capture);
                        if self.is_move_legal(&mv, is_king){
                            list.push(mv);
                        }
                    }
                },
                QUIET => {
                    for to in targets & !self.them() {
                        let mv = Move::new_from_squares(from, to, MoveType::Normal);
                        if self.is_move_legal(&mv, is_king){
                            list.push(mv);
                        }
                    }
                },
                _ => {}
            }
    }}

    pub fn collect_pawn_moves<const TYPE: u8>(&self, list: &mut MoveList){
        let (pawns, before_promotion) = match self.state.color {
            Color::White => (self.pieces[Piece::WhitePawn as usize], Bitboard::rank(7)),
            Color::Black => (self.pieces[Piece::BlackPawn as usize], Bitboard::rank(2)),
            _ => (Bitboard::new(0), Bitboard::new(0))
        };

        self.collect_pawn_pushes::<TYPE>(list, &pawns, &before_promotion);
        if TYPE == NOISY || TYPE == ALL {
            self.collect_pawn_captures::<NOISY>(list, pawns, before_promotion);
            self.collect_en_passant_moves(list, pawns);
        }
    }

    pub fn collect_pawn_pushes<const TYPE: u8>(&self, list: &mut MoveList, pawns: &Bitboard, before_promotion: &Bitboard){
        let (diff, double_push_rank) = match self.state.color {
            Color::White => (8, Bitboard::rank(3)),
            Color::Black => (-8i8, Bitboard::rank(6)),
            _ => (0, Bitboard::new(0))
        };

        let free = !self.occupied();

        if TYPE == QUIET || TYPE == ALL {
            let pushed = (*pawns & !before_promotion.0).shift(diff) & free;
            let double = (pushed & double_push_rank).shift(diff) & free;

            for to in pushed{
                let mv = Move::new_from_squares(to.shift(-diff), to, MoveType::Normal);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                }  
            }
            for to in double{
                let mv = Move::new_from_squares(to.shift(-2*diff), to, MoveType::DoublePush);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                } 
            }

        }

        let promotions = (*pawns & *before_promotion).shift(diff) & free;
        for to in promotions {
            let from = to.shift(-diff);

            if TYPE == NOISY || TYPE == ALL {
                let mv = Move::new_from_squares(from, to, MoveType::QPromotion);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                }
            }

            if TYPE == QUIET || TYPE == ALL{
                let mv = Move::new_from_squares(from, to, MoveType::BPromotion);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                    list.push(Move::new_from_squares(from, to, MoveType::RPromotion));
                    list.push(Move::new_from_squares(from, to, MoveType::KPromotion));
                }
                
            }
        }

    }


    fn collect_pawn_captures<const TYPE: u8>(&self, list: &mut MoveList, pawns: Bitboard, before_promotion: Bitboard) {
        let promotions = pawns & before_promotion;
        for from in promotions {
            let captures = self.them() & lookup_pawn_captures(&from, &self.state.color);
            for to in captures {
                let mv = Move::new_from_squares(from, to, MoveType::BPromotionCapture);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                    list.push(Move::new_from_squares(from, to, MoveType::KPromotionCapture));
                    list.push(Move::new_from_squares(from, to, MoveType::QPromotionCapture));
                    list.push(Move::new_from_squares(from, to, MoveType::RPromotionCapture));
                }
                
            }
        }

        let non_promotions = pawns & !before_promotion;
        for from in non_promotions {
            let targets = self.them() & lookup_pawn_captures(&from, &self.state.color);
            for to in targets {
                let mv = Move::new_from_squares(from, to, MoveType::Capture);
                if self.is_move_legal(&mv, false){
                    list.push(mv);
                }
            }
        }
    }

    fn collect_en_passant_moves(&self, list: &mut MoveList, pawns: Bitboard) {
        if self.state.en_passant != Square::None {
            let pawns = pawns & lookup_pawn_captures(&self.state.en_passant, &self.state.color.invert());
            for pawn in pawns {
                let mv = Move::new_from_squares(pawn, self.state.en_passant, MoveType::EnPassant);
                let mut _board = self.clone();
                _board.apply_move(&mv);
                if _board.is_legal(){
                    list.push(mv);
                }
            }
        }
    }

    pub fn is_attacked(&self,sq: &Square, color: Color, remove_piece: Option<Piece>) -> bool {
    let color_index = match color {
        Color::White => 6,
        Color::Black => 0,
        _ => return false
    };
    let mut blockers = self.occupied();
    if remove_piece.is_some() {
        blockers = blockers & !self.pieces[remove_piece.unwrap() as usize];
    }

    

    let moves = lookup_bishop(sq, &blockers);
    if (self.pieces[color_index + PieceType::Bishop as usize] | self.pieces[color_index + PieceType::Queen as usize]) & moves != 0 {
        return true;
    }

    let moves = lookup_rook(sq, &blockers);
    if (self.pieces[color_index + PieceType::Rook as usize] | self.pieces[color_index + PieceType::Queen as usize]) & moves != 0 {
        return true;
    }

    let moves = lookup_knight(sq);
    if self.pieces[color_index + PieceType::Knight as usize] & moves != 0 {
        return true;
    }
    
    let moves = lookup_king(sq);
    if self.pieces[color_index + PieceType::King as usize] & moves != 0 {
        return true;
    }

    if color == Color::White {
        let moves = lookup_pawn_captures(sq, &Color::White);
        if self.pieces[color_index + PieceType::Pawn as usize] & moves != 0 {
            return true;
        }
    } else {
        let moves = lookup_pawn_captures(sq, &Color::Black);
        if self.pieces[color_index + PieceType::Pawn as usize] & moves != 0 {
            return true;
        }
    }
    false

}

    pub fn collect_castling(&self, list: &mut MoveList) {
        let color = self.state.color;
        let possiblities = self.state.castling.get_castling_possibilities(color);
        let blockers = self.occupied();
        'outer: for (bb, mv) in possiblities {
            if mv.kind() == MoveType::KingCastle && (bb & !(1 << 4) & !(1 << 60)) & blockers == 0 || mv.kind() == MoveType::QueenCastle && (bb.0 >> 1) & blockers.0 == 0 {
                for sq in bb{
                    if bb.is_set(sq as u8) {
                        if self.is_attacked( &sq, color, None){
                            continue 'outer;
                        }
                    }
                }
                list.push(mv);
            }
        }
    }

    pub fn pseudo_to_legal(&mut self, moves: &MoveList) -> MoveList {
        let mut legal = MoveList::new();
        for mv in moves.iter() {
            self.apply_move(mv);
            if self.is_legal() {
                legal.push(*mv)
            }
            self.undo_move(mv);
        }
        legal
    }
    
    pub fn get_checker(&self) -> (Bitboard, u8) {
        let mut num_checkers = 0u8;
        let mut checkers = Bitboard(0);
        let (color_index, sq) = match self.state.color {
            Color::White => (6, Square::new(self.pieces[Piece::WhiteKing as usize].0.trailing_zeros() as u8)),
            Color::Black => (0, Square::new(self.pieces[Piece::BlackKing as usize].0.trailing_zeros() as u8)),
            _ => return (checkers, num_checkers)
        };
        let blockers = self.occupied();

        let moves = lookup_bishop(&sq, &blockers);
        let checking_bb = (self.pieces[color_index + PieceType::Bishop as usize] | self.pieces[color_index + PieceType::Queen as usize]) & moves;
        if checking_bb != 0 {
            for asq in checking_bb{
                num_checkers += 1;
                let attacking_moves = lookup_bishop(&asq, &blockers);
                checkers |= moves.0 & attacking_moves.0;
                checkers.set_bit(asq.to_index());
            }
        }

        let moves = lookup_rook(&sq, &blockers);
        let checking_bb = (self.pieces[color_index + PieceType::Rook as usize] | self.pieces[color_index + PieceType::Queen as usize]) & moves;
        if checking_bb != 0 {
            for asq in checking_bb{
                num_checkers += 1;
                let attacking_moves = lookup_rook(&asq, &blockers);
                checkers |= moves.0 & attacking_moves.0;
                checkers.set_bit(asq.to_index());
            }
        }

        let moves = lookup_knight(&sq);
        let checking_bb = (self.pieces[color_index + PieceType::Knight as usize]) & moves;
        if checking_bb != 0 {
            for asq in checking_bb{
                num_checkers += 1;
                checkers.set_bit(asq.to_index());
            }
        }
        
        let moves = lookup_king(&sq);
        let checking_bb = (self.pieces[color_index + PieceType::King as usize]) & moves;
        if checking_bb != 0 {
            for asq in checking_bb{
                num_checkers += 1;
                checkers.set_bit(asq.to_index());
            }
        }

        if self.state.color == Color::White && sq.to_index() < 56{
            let moves = lookup_pawn_captures(&sq, &Color::White);
            let checking_bb = (self.pieces[color_index + PieceType::Pawn as usize]) & moves;
            if checking_bb != 0 {
                for asq in checking_bb{
                    num_checkers += 1;
                    checkers.set_bit(asq.to_index());
                }
            }
        } else if sq.to_index() >= 8{
            let moves = lookup_pawn_captures(&sq, &Color::Black);
            let checking_bb = (self.pieces[color_index + PieceType::Pawn as usize]) & moves;
            if checking_bb != 0 {
                for asq in checking_bb{
                    num_checkers += 1;
                    checkers.set_bit(asq.to_index());
                }
            }
        }
        checkers.clear_bit(sq.to_index());
        return (checkers, num_checkers)
    }

    pub fn get_pinner(&self) -> (Bitboard, [Bitboard; 64]) {
        let mut pinned = Bitboard(0);
        let mut pin_rays = [Bitboard(0); 64];

        let (color_index, sq, our_blockers) = match self.state.color {
            Color::White => (6, Square::new(self.pieces[Piece::WhiteKing as usize].0.trailing_zeros() as u8), self.colors[Color::White as usize]),
            Color::Black => (0, Square::new(self.pieces[Piece::BlackKing as usize].0.trailing_zeros() as u8), self.colors[Color::Black as usize]),
            _ => return (pinned, pin_rays)
        };
        let blockers = self.occupied();

        let mut reduced_blockers = Bitboard(0);
        reduced_blockers.set_bit(sq.to_index());


        let potential_diag = self.pieces[color_index + PieceType::Bishop as usize] | self.pieces[color_index + PieceType::Queen as usize];
        for _sq in potential_diag{
            let _sq_bit = _sq.to_index();
            reduced_blockers.set_bit(_sq_bit);
            
            let moves_sq = lookup_bishop(&_sq, &reduced_blockers);
            if moves_sq.is_set(sq.to_index()) {
                let moves_king = lookup_bishop(&sq, &reduced_blockers);
                let ray_between = moves_sq & moves_king;
                if ray_between != 0 {
                    if (ray_between & blockers).count() == 1{
                        let _pinned = ray_between & our_blockers;
                        if _pinned.count() == 1 {
                            let idx = _pinned.lsb() as usize;
                            pinned |= _pinned.0;
                            pin_rays[idx] |= ray_between.0;
                            pin_rays[idx].set_bit(_sq_bit);
                        }
                    } 
                }
            }
            reduced_blockers.clear_bit(_sq_bit);
        }

        let potential_hv = self.pieces[color_index + PieceType::Rook as usize] | self.pieces[color_index + PieceType::Queen as usize];
        for _sq in potential_hv{
            let _sq_bit = _sq.to_index();
            reduced_blockers.set_bit(_sq_bit);

            let moves_sq = lookup_rook(&_sq, &reduced_blockers);
            if moves_sq.is_set(sq.to_index()) {
                let moves_king = lookup_rook(&sq, &reduced_blockers);
                let ray_between = moves_sq & moves_king;
                if ray_between != 0 {
                    if (ray_between & blockers).count() == 1{
                        let _pinned = ray_between & our_blockers;
                        if _pinned.count() == 1 {
                            let idx = _pinned.lsb() as usize;
                            pinned |= _pinned.0;
                            pin_rays[idx] |= ray_between.0;
                            pin_rays[idx].set_bit(_sq_bit);
                        }
                    } 
                }
            }
            reduced_blockers.clear_bit(_sq_bit);
        }

        return (pinned, pin_rays)
    }

    pub fn is_move_legal(&self, mv: &Move, is_king: bool) -> bool{
        if !self.is_move_pinned(mv) && self.does_move_evade_checkers(mv, is_king){
            return true;
        }
        false
    }

    pub fn is_move_pinned(&self, mv: &Move) -> bool{
        let pinned = self.state.pinned;
        if pinned & (1 << mv.from()) != 0{
            let pin_rays = self.state.pin_rays;
            let ray =  pin_rays[mv.from() as usize];
            if ray & (1 << mv.to()) != 0{
                return false;
            }
            return true;
        };
        return false;
    }

    pub fn does_move_evade_checkers(&self, mv: &Move, is_king: bool) -> bool{
        let checker = self.state.checker;
        let num_checker = self.state.num_checker;
        if !is_king{
            if num_checker == 0{
                return true;
            }
            if num_checker > 1{
                return false;
            }else{
                if checker & (1 << mv.to()) != 0{
                    return true;
                }
                return false;
            }
        }else{
            //println!("{}", self.is_attacked(&Square::new(mv.to()), self.state.color));
            let p = match self.state.color {
                Color::White => Piece::WhiteKing,
                Color::Black => Piece::BlackKing,
                _ => return false
            };
            if self.is_attacked(&Square::new(mv.to()), self.state.color, Some(p)){
                return false;
            }
            //println!("{}", mv.to_string());
            return true;
        }
    }
    

}


/// ######################################################
/// ################### TESTING ##########################
/// ######################################################


#[cfg(test)]
mod legal_move_count_tests {
    use crate::types::board::board::Board;
    use std::str::FromStr;

    fn count_legal_moves_depth_1(fen: &str) -> usize {
        let mut board = Board::from_str(fen).unwrap_or_else(|e| panic!("Failed to parse FEN for test '{}': {:?}", fen, e));
        let mut moves = board.generate_all_moves();
        
        moves = board.pseudo_to_legal(&moves);
        let legal_move_count = moves.len();

        legal_move_count
    }

    #[test]
    fn perft_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let expected_moves = 20;
        assert_eq!(count_legal_moves_depth_1(fen), expected_moves, "Initial position: FEN {}", fen);
    }

    #[test]
    fn perft_kiwipete() {
        let fen_full = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let expected_moves = 48;
        assert_eq!(count_legal_moves_depth_1(fen_full), expected_moves, "Kiwipete: FEN {}", fen_full);
    }

    #[test]
    fn perft_position_3() {
        let fen_full = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let expected_moves = 14;
        assert_eq!(count_legal_moves_depth_1(fen_full), expected_moves, "Position 3: FEN {}", fen_full);
    }

    #[test]
    fn perft_position_4() {
        // From CPW - tricky castling rights and attacks
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let expected_moves = 6;
        assert_eq!(count_legal_moves_depth_1(fen), expected_moves, "Position 4: FEN {}", fen);
    }

    #[test]
    fn perft_position_5() {
        // From CPW - involves promotions, checks
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let expected_moves = 44;
        assert_eq!(count_legal_moves_depth_1(fen), expected_moves, "Position 5: FEN {}", fen);
    }

    #[test]
    fn perft_position_6() {
        // From Steven Edwards' perft page
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let expected_moves = 46;
        assert_eq!(count_legal_moves_depth_1(fen), expected_moves, "Position 6: FEN {}", fen);
    }
}