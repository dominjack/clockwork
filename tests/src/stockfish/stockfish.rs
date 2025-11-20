use chess_core::types::{board::board::Board, movelist::MoveList, moves::Move};
use clockwork::search::perft::perft;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

// --- ASSUMPTIONS ---
// This module makes some assumptions about your `chess_core` library.
// You may need to adjust paths or function names if your implementation differs.
//
// 1. `Board` has a `to_fen()` method to get the FEN representation of the current position.
// 2. `Move` has methods to convert to/from a LAN (Long Algebraic Notation) string (e.g., "e2e4", "g1f3").
//    - `to_lan()`
//    - `from_lan(board: &Board, s: &str)`
// 3. `generate_all_moves()` returns all legal moves for the current position.
// --- END ASSUMPTIONS ---
// --- Modified UCI Engine (for Stockfish) ---

/// Manages communication with a UCI (Universal Chess Interface) compatible chess engine like Stockfish.
/// This allows the debugger to use Stockfish as an oracle for correct move generation.
pub struct UciEngine {
    process: std::process::Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
}

impl UciEngine {
    /// Launches the UCI engine process.
    pub fn new(path: &str) -> Result<Self, String> {
        let mut process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn engine '{}': {}", path, e))?;

        let stdin = process
            .stdin
            .take()
            .ok_or("Failed to get stdin".to_string())?;
        let stdout = process
            .stdout
            .take()
            .ok_or("Failed to get stdout".to_string())?;
        let reader = BufReader::new(stdout);

        Ok(UciEngine {
            process,
            stdin,
            reader,
        })
    }

    /// Sends a command to the UCI engine.
    fn send_command(&mut self, command: &str) -> Result<(), String> {
        writeln!(self.stdin, "{}", command)
            .and_then(|_| self.stdin.flush())
            .map_err(|e| format!("Failed to send command '{}': {}", command, e))
    }

    /// Reads lines from the engine's stdout until a specific marker line is found.
    fn read_until(&mut self, marker: &str) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();
        let mut buffer = String::new();
        loop {
            buffer.clear();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => return Err("Engine terminated prematurely.".to_string()),
                Ok(_) => {
                    let line = buffer.trim().to_string();
                    lines.push(line.clone());
                    if line == marker {
                        break;
                    }
                }
                Err(e) => return Err(format!("Error reading from engine: {}", e)),
            }
        }
        Ok(lines)
    }

    /// Initializes the UCI engine and waits for it to be ready.
    pub fn init(&mut self) -> Result<(), String> {
        self.send_command("uci")?;
        self.read_until("uciok")?;
        self.send_command("isready")?;
        self.read_until("readyok")?;
        Ok(())
    }

    /// Sets the board position in the UCI engine using a FEN string.
    pub fn set_position(&mut self, fen: &str) -> Result<(), String> {
        self.send_command("ucinewgame")?;
        self.send_command("isready")?;
        self.read_until("readyok")?;
        self.send_command(&format!("position fen {}", fen))
    }

    /// Runs a perft test on the UCI engine and returns the total node count.
    pub fn perft_total_nodes(&mut self, depth: u8) -> Result<u64, String> {
        self.send_command("isready")?;
        self.read_until("readyok")?;
        self.send_command(&format!("go perft {}", depth))?;

        let mut total_nodes: Option<u64> = None;
        let mut buffer = String::new();

        loop {
            buffer.clear();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => return Err("Engine terminated before perft finished.".to_string()),
                Ok(_) => {
                    let line = buffer.trim();
                    if line.starts_with("Nodes searched:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            total_nodes = parts[2].parse::<u64>().ok();
                            break;
                        }
                    }
                }
                Err(e) => return Err(format!("Error reading perft result: {}", e)),
            }
        }
        total_nodes.ok_or("Failed to parse total nodes from perft output.".to_string())
    }

    /// Runs a perft test on the UCI engine and returns a move-by-move node breakdown.
    pub fn perft_breakdown(&mut self, depth: u8) -> Result<HashMap<String, u64>, String> {
        if depth == 0 {
            return Ok(HashMap::new());
        }

        self.send_command("isready")?;
        self.read_until("readyok")?;
        self.send_command(&format!("go perft {}", depth))?;

        let mut breakdown = HashMap::new();
        let mut buffer = String::new();

        loop {
            buffer.clear();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => {
                    return Err("Engine terminated before perft breakdown finished.".to_string());
                }
                Ok(_) => {
                    let line = buffer.trim();

                    if line.starts_with("Nodes searched:") {
                        break;
                    }

                    if let Some((mv_str, count_str)) = line.split_once(": ") {
                        if let Ok(count) = count_str.parse::<u64>() {
                            breakdown.insert(mv_str.to_string(), count);
                        }
                    }
                }
                Err(e) => return Err(format!("Error reading perft breakdown: {}", e)),
            }
        }
        Ok(breakdown)
    }

    /// Sends the "quit" command to the engine and terminates the process.
    pub fn quit(&mut self) {
        let _ = self.send_command("quit");
        thread::sleep(Duration::from_millis(50));
        let _ = self.process.kill();
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        self.quit();
    }
}

// --- The Recursive Debugger ---

/// Recursively compares the perft results of the local engine and Stockfish
/// to find the first move generation discrepancy.
///
/// # Returns
/// `true` if a mismatch was found and the recursion went deeper, `false` otherwise.
fn debug_perft_recursive(my_board: &mut Board, stockfish: &mut UciEngine, depth: u8) -> bool {
    if depth == 0 {
        return false;
    }

    // 1. Get total node counts from both engines for the current position.
    let my_total_nodes = perft(my_board, depth).unwrap();

    stockfish.set_position(&my_board.to_fen()).unwrap();
    let stockfish_total_nodes = stockfish.perft_total_nodes(depth).unwrap();

    // 2. If the totals match, this branch is correct.
    if my_total_nodes == stockfish_total_nodes {
        return false;
    }

    // --- MISMATCH FOUND ---
    println!("\n--- Mismatch Found at Depth {} ---", depth);
    println!("FEN: {}", my_board.to_fen());
    println!("  Clockwork (Yours): {}", my_total_nodes);
    println!("  Stockfish (Oracle): {}", stockfish_total_nodes);

    // 3. Get the per-move breakdown for the next ply to pinpoint the error.
    stockfish.set_position(&my_board.to_fen()).unwrap();
    let sf_breakdown = stockfish.perft_breakdown(depth).unwrap_or_default();

    let my_moves = my_board.generate_all_moves();
    let mut my_breakdown = HashMap::new();
    for mv in my_moves.iter() {
        if my_board.apply_move(mv).is_ok() {
            let child_nodes = perft(my_board, depth - 1).unwrap();
            my_board.undo_move(mv);
            my_breakdown.insert(mv.to_lan(), child_nodes);
        }
    }

    // 4. Compare the two breakdowns to find the exact move with the discrepancy.
    let mut all_moves: Vec<_> = sf_breakdown.keys().chain(my_breakdown.keys()).collect();
    all_moves.sort();
    all_moves.dedup();

    if depth == 1 {
        for mv in my_moves.iter() {
            let lan = mv.to_lan();
            print!("{}, ", lan);
        }
        println!("");
        println!("SF: {:?}", sf_breakdown.keys());
    }

    let mut first_mismatched_move: Option<String> = None;

    println!("\n  Breakdown (Clockwork vs Stockfish):");
    for mv_str in all_moves {
        let my_nodes = my_breakdown.get(mv_str).cloned();
        let sf_nodes = sf_breakdown.get(mv_str).cloned();

        if my_nodes != sf_nodes {
            println!(
                "  - {}: {} (Clockwork) vs {} (Stockfish) <-- MISMATCH",
                mv_str,
                my_nodes.map_or("Missing".to_string(), |n| n.to_string()),
                sf_nodes.map_or("Missing".to_string(), |n| n.to_string())
            );
            if first_mismatched_move.is_none() {
                first_mismatched_move = Some(mv_str.clone());
            }
        }
    }

    // 5. Recurse into the first mismatched move to go deeper.
    if let Some(bad_move_str) = first_mismatched_move {
        println!("Bad move: {}", bad_move_str);

        let mv = Move::from_lan(my_board, &bad_move_str);

        my_board.apply_move(&mv);
        debug_perft_recursive(my_board, stockfish, depth - 1);
        my_board.undo_move(&mv);
        return true;
    }

    false
}

/// Main entry point for the perft debugger.
///
/// This function initializes and configures both the local `clockwork` engine and the `stockfish`
/// engine. It then kicks off the recursive debugging process by calling `debug_perft_recursive`.
///
/// # Arguments
/// * `fen` - The Forsyth-Edwards Notation (FEN) string representing the board state to test.
/// * `depth` - The maximum depth for the perft search and comparison.
pub fn test_perft_deep(fen: &str, depth: u8) {
    println!("--- Perft Debugger Initializing ---");
    println!("Comparing Clockwork vs. Stockfish");

    // --- Setup Stockfish ---
    let mut stockfish = match UciEngine::new("stockfish") {
        Ok(engine) => engine,
        Err(e) => {
            println!("Failed to start Stockfish: {}", e);
            println!("Please ensure 'stockfish' is in your system PATH.");
            return;
        }
    };
    if let Err(e) = stockfish.init() {
        println!("Failed to initialize Stockfish: {}", e);
        return;
    }
    println!("Stockfish (Oracle) initialized.");

    // --- Setup Your Engine ---
    let mut my_board = match Board::from_fen(fen) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to parse FEN for Clockwork: {:?}", e);
            return;
        }
    };
    println!("Clockwork (Your Engine) initialized.");

    // --- Run the Debugger ---
    println!("\nStarting recursive debug for FEN: {}", fen);
    println!("Max Depth: {}", depth);
    if !debug_perft_recursive(&mut my_board, &mut stockfish, depth) {
        println!("\n--- SUCCESS ---");
        println!(
            "All node counts match Stockfish exactly to depth {}.",
            depth
        );
    } else {
        println!("\n--- DEBUGGING COMPLETE ---");
        println!("Traced error to the deepest point found.");
    }
}
