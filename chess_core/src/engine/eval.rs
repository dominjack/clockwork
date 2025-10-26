use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::piece::Piece;
use crate::types::board::board::Board;
use crate::engine::piece_square_tables::{PSTS, flip_square};

use std::sync::LazyLock;

static FILES: LazyLock<[Bitboard; 8]> = LazyLock::new(|| [
    Bitboard::file(0),
    Bitboard::file(1),
    Bitboard::file(2),
    Bitboard::file(3),
    Bitboard::file(4),
    Bitboard::file(5),
    Bitboard::file(6),
    Bitboard::file(7),
]);
static RANKS: LazyLock<[Bitboard; 8]> = LazyLock::new(|| [
    Bitboard::rank(0),
    Bitboard::rank(1),
    Bitboard::rank(2),
    Bitboard::rank(3),
    Bitboard::rank(4),
    Bitboard::rank(5),
    Bitboard::rank(6),
    Bitboard::rank(7),
]);


impl Board{
    pub fn eval(&mut self) -> i32{
        let mut score = 0i32;
        score += piece_square_eval(self);
        score += material_eval(self);
        score += color_eval(self);
        //score += pawn_eval(self);
        //score += mobility_eval(self);
        score
    }
}

fn mobility_eval(board: &mut Board) -> i32{
    let mut score = 0i32;
    score += (board.generate_attacking_mask(Color::White).count() as i32) * 10;
    score -= (board.generate_attacking_mask(Color::Black).count() as i32) * 10;
    score
}

pub fn piece_square_eval(board: &Board) -> i32{
    let mut score = 0i32;
    score -= piece_square_from_color(board, Color::Black);
    score += piece_square_from_color(board, Color::White);
    score
}

pub fn color_eval(board: &Board) -> i32{
    match board.state.color{
        Color::White => {return 20},
        Color::Black => {return -20},
        _ => {return 0}
    }
            
}

pub fn piece_square_from_color(board: &Board, color: Color) -> i32{
    let mut score = 0i32;
    if color == Color::White{
        let mut squares = board.pieces[Piece::WhitePawn as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.pawn[sq.to_index() as usize];
        }
        squares = board.pieces[Piece::WhiteRook as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.rook[sq.to_index() as usize];
        }
        squares = board.pieces[Piece::WhiteKnight as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.knight[sq.to_index() as usize];
        }
        squares = board.pieces[Piece::WhiteBishop as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.bishop[sq.to_index() as usize];
        }
        squares = board.pieces[Piece::WhiteQueen as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.queen[sq.to_index() as usize];
        }
        squares = board.pieces[Piece::WhiteKing as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.king_mg[sq.to_index() as usize];
        }
    }else if color == Color::Black{
        let mut squares = board.pieces[Piece::BlackPawn as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.pawn[flip_square(sq.to_index() as usize)];
        }
        squares = board.pieces[Piece::BlackRook as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.rook[flip_square(sq.to_index() as usize)];
        }
        squares = board.pieces[Piece::BlackKnight as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.knight[flip_square(sq.to_index() as usize)];
        }
        squares = board.pieces[Piece::BlackBishop as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.bishop[flip_square(sq.to_index() as usize)];
        }
        squares = board.pieces[Piece::BlackQueen as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.queen[flip_square(sq.to_index() as usize)];
        }
        squares = board.pieces[Piece::BlackKing as usize].to_squares();
        for sq in squares.iter(){
            score += PSTS.king_mg[flip_square(sq.to_index() as usize)];
        }
    }
    score
}

pub fn material_eval(board: &Board) -> i32{
    let mut score = 0i32;
    score -= material_from_color(board, Color::Black);
    score += material_from_color(board, Color::White);
    score

}

fn material_from_color(board: &Board, color: Color) -> i32{
    let mut score = 0usize;
    if color == Color::White{
        score +=board.pieces[Piece::WhitePawn as usize].count() * 100;
        score +=board.pieces[Piece::WhiteKnight as usize].count() * 300;
        score +=board.pieces[Piece::WhiteBishop as usize].count() * 325;
        score +=board.pieces[Piece::WhiteRook as usize].count() * 500;
        score +=board.pieces[Piece::WhiteQueen as usize].count() * 900;
    }else if color == Color::Black{
        score +=board.pieces[Piece::BlackPawn as usize].count() * 100;
        score +=board.pieces[Piece::BlackKnight as usize].count() * 300;
        score +=board.pieces[Piece::BlackBishop as usize].count() * 325;
        score +=board.pieces[Piece::BlackRook as usize].count() * 500;
        score +=board.pieces[Piece::BlackQueen as usize].count() * 900;
    }
    score as i32
}

fn pawn_eval(board: &Board) -> i32{
    let mut score = 0i32;
    score += pawn_structure_eval(board, Color::White);
    score -= pawn_structure_eval(board, Color::Black);
    score
}

fn pawn_structure_eval(board: &Board, color:Color) -> i32{
    let mut score = 0i32;
    score += get_isolated_pawns(board, color)*-30;
    score += get_double_pawns(board, color)*-30;
    score
}

fn get_isolated_pawns(board: &Board, color:Color) -> i32{
    let mut num_isolated = 0i32;
    let pawns = match color {
        Color::White => board.pieces[Piece::WhitePawn as usize],
        Color::Black => board.pieces[Piece::BlackPawn as usize],
        _ => Bitboard(0)
    };
    if (pawns & FILES[0]).0 > 0 && (pawns & FILES[1]).0 == 0 {
        num_isolated += 1;
    }
    if (pawns & FILES[7]).0 > 0 && (pawns & FILES[6]).0 == 0 {
        num_isolated += 1;
    }
    for i in 0..5{
        if (pawns & RANKS[i]).0 == 0 && (pawns & RANKS[i+2]).0 == 0 && (pawns & RANKS[i+1]).0 > 0 {
            num_isolated += 1;
        }
    }
     num_isolated   
}

fn get_double_pawns(board: &Board, color:Color) ->i32{
    let mut num_doubled = 0i32;
    let pawns = match color {
        Color::White => board.pieces[Piece::WhitePawn as usize],
        Color::Black => board.pieces[Piece::BlackPawn as usize],
        _ => Bitboard(0)
    };
    for i in 0..7{
        if (pawns & RANKS[i]).count() >= 2 {
            num_doubled += 1;
        }
    }
    num_doubled   
}

