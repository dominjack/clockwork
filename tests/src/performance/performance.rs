use chess_core::types::board::board::Board;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

/// Holds the results of a benchmark for a single position.
#[derive(Debug)]
struct BenchmarkResult {
    /// The FEN string of the benchmarked position.
    fen: String,
    /// The average time taken to generate all moves for the position.
    move_gen_time: Duration,
    /// The average time taken for a make/unmake move cycle.
    make_unmake_time_per_move: Duration,
    /// The number of moves generated for this position.
    num_moves_generated: usize,
    /// The number of moves tested in the make/unmake benchmark (loops).
    num_moves_tested: usize,
}

/// Runs a benchmark for a given FEN position.
/// It measures the performance of move generation and make/unmake move functions.
fn run_benchmark_for_fen(fen: &str) -> Option<BenchmarkResult> {
    let mut board = Board::from_fen(fen).unwrap();

    // 1. Measure Move Generation Time
    let mut move_gen_time = Duration::ZERO;
    let mut num_tested = 0;

    // --- Warm-up ---
    // Run a few times before starting the timer to let the CPU ramp up
    // and to get the position's data into the cache.
    let _ = board.generate_all_moves();
    let _ = board.generate_all_moves();
    let _ = board.generate_all_moves();
    // --- End Warm-up ---

    // We run the test many times to get a stable average.
    // 100 iterations might be too few or too many depending on the machine;
    // a time-based loop (e.g., "run for 1 second") is another good approach.
    for _i in 0..100 {
        let start_time = Instant::now();
        let _moves = board.generate_all_moves();
        move_gen_time += start_time.elapsed();
        num_tested += 1;
    }

    move_gen_time /= num_tested;

    // 2. Measure Apply/Undo Move Time
    let mut total_make_unmake_time = Duration::ZERO;
    let mut moves_tested = 0;

    let moves = board.generate_all_moves();
    let num_moves_generated = moves.len(); // Get the number of moves

    // We test every single generated move.
    for mv in moves.iter() {
        // And we test each move multiple times to get a good average.
        for _i in 0..10 {
            let start_time = Instant::now();

            if board.apply_move(mv).is_ok() {
                board.undo_move(mv);
            }

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
        num_moves_generated, // Store the number of moves
        num_moves_tested: moves_tested,
    })
}

/// Runs the performance benchmark on a set of predefined FEN positions
/// and prints a summary of the results.
pub fn test() {
    println!("--- Chess Engine Performance Benchmark ---");

    // We've expanded this list to 20 to include more diverse positions.
    let fens = vec![
        // --- Standard Positions ---
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // 1. Starting position (Perft 1)
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", // 2. "Kiwipete" - Complex mid-game (Perft 2)
        "8/2p5/3p4/KP5r/1P3p1k/8/4P1P1/8 w - - 0 1", // 3. Endgame (Perft 3)
        "r3k2r/Pppp1ppp/1b3nbN/n7/B7/S7/P1P1P1PP/R3K2R w KQkq - 0 1", // 4. "Position 4" - Castling, piece-heavy
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 1 9", // 5. "Position 5" - Promotion, complex
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP2PPP/R2Q1RK1 w - - 0 10", // 6. "Position 6" - Symmetrical, many pieces
        // --- Specific Test Cases ---
        "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3", // 7. En Passant test
        "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1", // 8. Promotion test (multiple promotions)
        "r1k4r/p7/2N1b3/2N1b3/8/8/8/R1K4R w - - 0 1", // 9. Check-heavy position
        "8/7p/5kp1/5p2/p1P2P2/P5P1/8/6K1 w - - 0 1", // 10. Simple pawn endgame (from original list)
        // --- Added 10 More Positions ---
        "r3k2r/pppb1ppp/1b3n2/nP6/B7/S1P1P3/P3N1PP/R1B1K2R w KQkq - 1 12", // 11. Castling rights test (long/short available)
        "r1bqk2r/ppp2ppp/2n5/3p4/3Pn3/2B2N2/PPP2PPP/R2QKB1R w KQkq - 0 8", // 12. Common opening (Scotch Game)
        "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1", // 13. Rook endgame (simple)
        "8/k7/8/p1p1p1p1/P1P1P1P1/8/8/4K3 w - - 0 1", // 14. Pawn-only endgame
        "7k/8/8/8/8/8/7p/7K w - - 0 1",      // 15. Simple promotion test (pawn vs king)
        "r6r/1b2k1b1/pq1p1p2/1p2pP2/3n3Q/3B1N2/PPP2KPP/R6R w - - 0 20", // 16. Tactically sharp middlegame
        "2r5/3pk3/8/2p5/8/2P5/3K4/8 w - - 0 1",                         // 17. Rook & Pawn endgame
        "q7/8/8/8/8/8/1K6/8 w - - 0 1", // 18. Queen endgame (simple)
        "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 3", // 19. Common opening (Italian Game)
        "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1", // 20. Castling test (Kings & Rooks only)
    ];

    let mut all_results = Vec::new();

    for (i, fen) in fens.iter().enumerate() {
        println!("Benchmarking Position {}/{}...", i + 1, fens.len());
        println!("  FEN: {}", fen);
        if let Some(result) = run_benchmark_for_fen(fen) {
            // Calculate per-move time for this specific FEN
            let avg_time_per_individual_move = if result.num_moves_generated > 0 {
                result.move_gen_time / result.num_moves_generated as u32
            } else {
                Duration::ZERO
            };

            println!("  Move Generation (all moves): {:?}", result.move_gen_time);
            println!(
                "  Move Generation (per move, avg of {}): {:?}",
                result.num_moves_generated, avg_time_per_individual_move
            );
            println!(
                "  Make/Unmake (avg over {} loops): {:?}",
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
    let mut total_moves_generated = 0; // Total moves generated across all benchmarks

    for result in &all_results {
        total_move_gen_time += result.move_gen_time;
        total_make_unmake_time += result.make_unmake_time_per_move * result.num_moves_tested as u32;
        total_moves_tested_for_make_unmake += result.num_moves_tested;
        total_moves_generated += result.num_moves_generated; // Add to total
    }

    let num_fens = all_results.len();
    if num_fens > 0 {
        println!(
            "Average Move Generation (all moves, per position): {:?}",
            total_move_gen_time / num_fens as u32
        );

        // Calculate and print the new overall average per *single* move
        if total_moves_generated > 0 {
            println!(
                "Overall Average Move Generation (per single move): {:?}",
                total_move_gen_time / total_moves_generated as u32
            );
        } else {
            println!("Overall Average Move Generation (per single move): N/A");
        }

        if total_moves_tested_for_make_unmake > 0 {
            println!(
                "Overall Average Make/Unmake (per move loop): {:?}",
                total_make_unmake_time / total_moves_tested_for_make_unmake as u32
            );
        } else {
            println!("Overall Average Make/Unmake (per move loop): N/A (no moves tested)");
        }
    } else {
        println!("No FENs were successfully benchmarked.");
    }
}
