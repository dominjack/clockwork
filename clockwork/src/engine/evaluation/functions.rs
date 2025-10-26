use chess_core::types::board::board::Board;
use chess_core::types::piece::Piece;
use chess_core::types::color::Color;

use super::tables::{PieceSquareTables};

pub fn material_eval(board: &Board) -> i32{
    let mut score = 0i32;
    score +=board.pieces[Piece::WhitePawn as usize].count() as i32 * 100;
    score +=board.pieces[Piece::WhiteKnight as usize].count() as i32 * 300;
    score +=board.pieces[Piece::WhiteBishop as usize].count() as i32 * 325;
    score +=board.pieces[Piece::WhiteRook as usize].count() as i32 * 500;
    score +=board.pieces[Piece::WhiteQueen as usize].count() as i32 * 900;
    score -=board.pieces[Piece::BlackPawn as usize].count() as i32 * 100;
    score -=board.pieces[Piece::BlackKnight as usize].count() as i32 * 300;
    score -=board.pieces[Piece::BlackBishop as usize].count() as i32 * 325;
    score -=board.pieces[Piece::BlackRook as usize].count() as i32 * 500;
    score -=board.pieces[Piece::BlackQueen as usize].count() as i32 * 900;
    score as i32
}

pub fn pices_square_eval(board: &Board, psts: &PieceSquareTables) -> i32{
    let mut score = 0i32;
        
    let mut squares = board.pieces[Piece::WhitePawn as usize].to_squares();
    for sq in squares.iter(){
        score += psts.pawn[sq.to_index() as usize];
    }
    squares = board.pieces[Piece::WhiteRook as usize].to_squares();
    for sq in squares.iter(){
        score += psts.rook[sq.to_index() as usize];
    }
    squares = board.pieces[Piece::WhiteKnight as usize].to_squares();
    for sq in squares.iter(){
        score += psts.knight[sq.to_index() as usize];
    }
    squares = board.pieces[Piece::WhiteBishop as usize].to_squares();
    for sq in squares.iter(){
        score += psts.bishop[sq.to_index() as usize];
    }
    squares = board.pieces[Piece::WhiteQueen as usize].to_squares();
    for sq in squares.iter(){
        score += psts.queen[sq.to_index() as usize];
    }
    squares = board.pieces[Piece::WhiteKing as usize].to_squares();
    for sq in squares.iter(){
        score += psts.king_mg[sq.to_index() as usize];
    }
    let mut squares = board.pieces[Piece::BlackPawn as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.pawn[flip_square(sq.to_index() as usize)];
    }
    squares = board.pieces[Piece::BlackRook as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.rook[flip_square(sq.to_index() as usize)];
    }
    squares = board.pieces[Piece::BlackKnight as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.knight[flip_square(sq.to_index() as usize)];
    }
    squares = board.pieces[Piece::BlackBishop as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.bishop[flip_square(sq.to_index() as usize)];
    }
    squares = board.pieces[Piece::BlackQueen as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.queen[flip_square(sq.to_index() as usize)];
    }
    squares = board.pieces[Piece::BlackKing as usize].to_squares();
    for sq in squares.iter(){
        score -= psts.king_mg[flip_square(sq.to_index() as usize)];
    }
    
    score
}

#[inline(always)]
pub fn flip_square(sq: usize) -> usize {
    sq ^ 56 // XOR with 0b111000, effectively (7-rank)*8 + file
}