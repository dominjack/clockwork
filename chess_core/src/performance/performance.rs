use crate::types::board::board::Board;
use std::{str::FromStr, time::{Duration, Instant}};

#[derive(Debug)]
struct BenchmarkResult {
    fen: String,
    move_gen_time: Duration,
    make_unmake_time_per_move: Duration, // Average time per make/unmake cycle
    num_moves_tested: usize,
}

/// Runs a benchmark for a given FEN position.
fn run_benchmark_for_fen(fen: &str) -> Option<BenchmarkResult> {
    let mut board = Board::from_str(fen).unwrap();
    //board.generate_all_moves();
    //board.generate_all_moves();
    //board.generate_all_moves();
    //board.generate_all_moves();

    // 1. Measure Move Generation Time
    let mut move_gen_time = Duration::ZERO;
    let mut num_tested = 0;

    for _i in 0..100{
        let start_time = Instant::now();
        let moves = board.generate_all_moves();
        move_gen_time += start_time.elapsed();
        num_tested += 1;
    }

    move_gen_time /= num_tested;
    

    // 3. Measure Apply/Undo Move Time
    let mut total_make_unmake_time = Duration::ZERO;
    let mut moves_tested = 0;

    let moves = board.generate_all_moves();


    for mv in moves.iter() {
        for _i in 0..10{
            let start_time = Instant::now();
            
            // Ensure the move is valid before attempting (your engine's generate_moves should ensure this)
            // For the dummy board, any generated move is "valid" for make/unmake.
            board.apply_move(mv);
            board.undo_move(mv); // Restore board state
            
            total_make_unmake_time += start_time.elapsed();
            moves_tested += 1;
        }
    }

    let make_unmake_time_per_move = if moves_tested > 0 {
        total_make_unmake_time / moves_tested as u32
    } else {
        Duration::ZERO
    };

    Some(BenchmarkResult {
        fen: fen.to_string(),
        move_gen_time,
        make_unmake_time_per_move,
        num_moves_tested: moves_tested,
    })
}

pub fn test() {
    println!("--- Chess Engine Performance Benchmark ---");

    let fens = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Starting position
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", // Complex middle game
        "8/2p5/3p4/KP5r/1P3p1k/8/4P1P1/8 w - - 0 1", // Endgame
        "8/7p/5kp1/5p2/p1P2P2/P5P1/8/6K1 w - - 0 1", // Another endgame
    ];

    let mut all_results = Vec::new();

    for fen in fens {
        println!("Benchmarking FEN: {}", fen);
        if let Some(result) = run_benchmark_for_fen(fen) {
            println!("  Move Generation: {:?}", result.move_gen_time);
            println!(
                "  Make/Unmake (avg over {} moves): {:?}",
                result.num_moves_tested, result.make_unmake_time_per_move
            );
            println!();
            all_results.push(result);
        } else {
            println!("  Failed to parse FEN or run benchmark.\n");
        }
    }

    println!("--- Overall Summary ---");
    let mut total_move_gen_time = Duration::ZERO;
    let mut total_make_unmake_time = Duration::ZERO;
    let mut total_moves_tested_for_make_unmake = 0;

    for result in &all_results {
        total_move_gen_time += result.move_gen_time;
        total_make_unmake_time += result.make_unmake_time_per_move * result.num_moves_tested as u32; // Re-calculate total from average
        total_moves_tested_for_make_unmake += result.num_moves_tested;
    }

    let num_fens = all_results.len();
    if num_fens > 0 {
        println!("Average Move Generation: {:?}", total_move_gen_time / num_fens as u32);
        if total_moves_tested_for_make_unmake > 0 {
            println!(
                "Overall Average Make/Unmake (per move): {:?}",
                total_make_unmake_time / total_moves_tested_for_make_unmake as u32
            );
        } else {
            println!("Overall Average Make/Unmake (per move): N/A (no moves tested)");
        }
    } else {
        println!("No FENs were successfully benchmarked.");
    }
}