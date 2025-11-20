use std::time::Instant;

use chess_core::types::board::board::Board;

/// Tests the move generation for the starting position.
/// It generates all moves, applies and undoes them, and prints the total time taken.
pub fn main() {
    let mut board = Board::start();
    let moves = board.generate_all_moves();

    let starttime = Instant::now();
    for mv in moves.iter() {
        if board.apply_move(mv).is_ok() {
            board.undo_move(mv);
        } else {
            println!("error for move: {}", mv.to_lan());
        }
    }
    println!("Move generation took {}ns", starttime.elapsed().as_nanos())
}
