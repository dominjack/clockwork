use chess_core::types::board::board::Board;
use chess_core::types::color::Color;
use chess_core::types::moves::Move;
use super::evaluation::evaluator::Evaluator;
use once_cell::sync::Lazy;

// This will now compile
static EVALUATOR: Lazy<Evaluator> = Lazy::new(|| {
    // This closure will only be run once,
    // the first time EVALUATOR is referenced.
    Evaluator::new()
});

pub fn negamax(board: &mut Board, depth: u8) -> (i32, Option<Move>, u64){
    if depth == 0{
        return (EVALUATOR.evaluate(board), None, 1)
    }else{
        let multiplier = if board.state.color == Color::White {1} else {-1};
        let moves = board.generate_all_moves();
        let mut best_score = 0;
        let mut best_move = Move::NULL;
        let mut searched = 0u64;
        for mv in moves.iter(){
            board.apply_move(mv);
            let out = negamax(board, depth-1);
            let score = out.0;
            searched += out.2;
            if score * multiplier >= best_score{
                best_score = score * multiplier;
                best_move = *mv;
            }
            board.undo_move(mv);
        }
        return (best_score, Some(best_move), searched)
    }
}