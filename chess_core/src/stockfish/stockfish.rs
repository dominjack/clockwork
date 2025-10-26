use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio, ChildStdin, ChildStdout};
use std::time::Duration;
use std::thread;

/// Manages communication with a UCI chess engine like Stockfish.
pub struct UciEngine {
    process: std::process::Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
}

impl UciEngine {
    /// Launches the UCI engine.
    pub fn new() -> Result<Self, String> {
        let mut process = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()) // Capture stderr too, for debugging
            .spawn()
            .map_err(|e| format!("Failed to spawn engine '{}': {}", "stockfish", e))?;

        let stdin = process.stdin.take().ok_or("Failed to get stdin".to_string())?;
        let stdout = process.stdout.take().ok_or("Failed to get stdout".to_string())?;
        let reader = BufReader::new(stdout);

        Ok(UciEngine { process, stdin, reader })
    }

    /// Sends a command to the UCI engine.
    fn send_command(&mut self, command: &str) -> Result<(), String> {
        // println!("> {}", command); // For debugging UCI communication
        writeln!(self.stdin, "{}", command)
            .and_then(|_| self.stdin.flush())
            .map_err(|e| format!("Failed to send command '{}': {}", command, e))
    }

    /// Reads lines until a specific marker is found or timeout.
    fn read_until(&mut self, marker: &str) -> Result<Vec<String>, String> {
        let mut lines = Vec::new();
        let mut buffer = String::new();
        // Note: A more robust solution would use non-blocking reads or select/poll
        // if this were part of a larger async application. For a simple perft tool,
        // blocking reads with timeouts (not easily done with just BufReader::read_line)
        // or just reading are common. Here, we'll just read.
        // A simple timeout mechanism isn't built into BufReader directly.
        // For real applications, consider crates like `subprocess` or async operations.

        loop {
            buffer.clear();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => return Err("Engine terminated prematurely or stream ended.".to_string()), // EOF
                Ok(_) => {
                    let line = buffer.trim().to_string();
                    // println!("< {}", line); // For debugging UCI communication
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

    /// Sets the board position using a FEN string.
    pub fn set_position(&mut self, fen: &str) -> Result<(), String> {
        self.send_command("ucinewgame")?; // Good practice before new position
        self.send_command("isready")?;    // Ensure it's ready after ucinewgame
        self.read_until("readyok")?;
        self.send_command(&format!("position fen {}", fen))
    }

    /// Runs perft to a given depth and returns the total node count.
    pub fn perft(&mut self, depth: u8) -> Result<u64, String> {
        self.send_command("isready")?; // Make sure it's ready before a 'go' command
        self.read_until("readyok")?;

        self.send_command(&format!("go perft {}", depth))?;

        let total_nodes: Option<u64>;
        let mut buffer = String::new();

        // Stockfish perft output ends with "Nodes searched: <count>"
        // It also prints individual move counts before that.
        loop {
            buffer.clear();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => return Err("Engine terminated before perft finished.".to_string()),
                Ok(_) => {
                    let line = buffer.trim();
                    // println!("< {}", line); // Debugging
                    if line.starts_with("Nodes searched:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            total_nodes = parts[2].parse::<u64>().ok();
                            break; // Found the total nodes line
                        }
                    }
                    // If depth is 1, you could also count the lines that look like "<move>: 1"
                    // to get the number of legal moves directly.
                }
                Err(e) => return Err(format!("Error reading perft result: {}", e)),
            }
        }
        total_nodes.ok_or("Failed to parse total nodes from perft output.".to_string())
    }

    /// Quits the engine.
    pub fn quit(&mut self) -> Result<(), String> {
        self.send_command("quit").ok(); // Try to quit, ignore error if already closing
        // Wait a brief moment for the process to exit, then kill if necessary
        thread::sleep(Duration::from_millis(100));
        let _ = self.process.try_wait(); // Check if exited
        if self.process.try_wait().map_or(true, |s| s.is_none()) { // if still running
             let _ = self.process.kill(); // Force kill
        }
        Ok(())
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        let _ = self.quit(); // Ensure engine is quit when UciEngine instance goes out of scope
    }
}
