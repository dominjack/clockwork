use super::tables::{PieceSquareTables, PSTS};
use chess_core::types::board::board::Board;
use super::functions::{material_eval, pices_square_eval};

pub struct Evaluator{
    psts: PieceSquareTables,
}

impl Evaluator{
    pub fn new() ->  Self{
        Self {
            psts: PSTS
        }
    }

    pub fn evaluate(&self, board: &Board) -> i32{
        let mut score = 0;
        let us = board.us();
        
        score += material_eval(board); // Evaluate raw material difference
        score += pices_square_eval(board, &self.psts); // Evaluate board with piece square tables
        
        score
    }
}