use std::time::Instant;

use chess_core::types::board::board::Board;

/// A function for performance testing and debugging of the move generation.
/// Perft (performance test) is a standard test for chess engines that counts the
/// number of legal moves from a given position to a certain depth.
/// It is used to verify the correctness of the move generation.
pub fn perft(board: &mut Board, ply: u8) -> Result<u64, ()> {
    if ply == 0 {
        return Ok(1);
    }

    let moves = board.generate_all_moves();
    let mut num_nodes: u64 = 0;

    for mv in moves.iter() {
        if board.apply_move(mv).is_ok() {
            num_nodes += perft(board, ply - 1)?;
            board.undo_move(mv);
        }
    }

    Ok(num_nodes)
}

/// Initiates and manages a perft test from a given board state to a specified depth.
///
/// This function prints the progress of the perft test to the console, including a breakdown
/// of nodes per move at the first level. It also prints a summary at the end of the test.
///
/// # Arguments
/// * `board` - The starting board state.
/// * `ply` - The depth of the perft test.
///
/// # Returns
/// The total number of leaf nodes found during the perft test.
pub fn start_perft(board: &mut Board, ply: u8) -> u64 {
    if ply == 0 {
        return 0;
    }
    let mut num_nodes: u64 = 0;
    let moves = board.generate_all_moves();

    println!("Starting perft depth {} on fen {}", ply, board.to_fen());
    println!("|-------------------------------------------------------------|");
    println!(
        "{:>5} {:>7} {:>12} {:>13} {:>15}",
        "Nr.", "Move", "Nodes", "Elapsed", "NPS"
    );
    println!("|-------------------------------------------------------------|");

    let stopwatch = Instant::now();

    for (index, mv) in moves.iter().enumerate() {
        let _stopwatch = Instant::now();
        if board.apply_move(mv).is_ok() {
            let out = perft(board, ply - 1);
            board.undo_move(mv);
            match out {
                Ok(num) => {
                    println!(
                        "|{:>3} {:>8} {:>12} {:>12.3}s {:>15.2} kN/s |",
                        index,
                        mv.to_lan(),
                        num,
                        _stopwatch.elapsed().as_secs_f32(),
                        (num as f32) / 1000. / (_stopwatch.elapsed().as_secs_f32())
                    );
                    num_nodes += num;
                }
                _ => {}
            }
        }
    }
    println!("|-------------------------------------------------------------|");
    println!(
        "|    Total: {:>14} {:>12.3}s {:>15.3} kN/s |",
        num_nodes,
        stopwatch.elapsed().as_secs_f32(),
        (num_nodes as f32) / 1000. / (stopwatch.elapsed().as_secs_f32())
    );
    println!("|-------------------------------------------------------------|");

    num_nodes
}
