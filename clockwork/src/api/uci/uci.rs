use chess_core::types::board::board::Board;
use chess_core::types::moves::Move;
use std::error::Error;
use std::io;
use std::process;
use std::str::FromStr;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;
use std::time::Duration;
use chess_core::engine::perft::perft;




pub struct UciInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub score_cp: Option<i32>,
    pub score_mate: Option<i32>,
    pub nodes: u64,
    pub nps: u64,
    pub time: Duration,
    pub pv: Vec<Move>,
}


pub fn uci_listener() {
    eprintln!("UCI starting up...");
    
    let mut board = Board::start();
    
    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let commands: Vec<&str> = input.trim().split_whitespace().collect();
        
        
        if let Some(&command) = commands.get(0) {
            match command {
                "uci" => uci_handshake(),
                "isready" => is_ready(),
                "ucinewgame" => board = Board::start(), 
                "position" => {
                    if let Err(e) = handle_position_command(&mut board, &commands) {
                        eprintln!("Error parsing position: {}", e);
                    }
                },
                "go" => {
                    handle_go_command(&board, &commands);
                },
                "stop" => {
                    // This command is handled implicitly by the search stopping
                    // when it sees "stop" or a new "go" command. We'll use atomics for this.
                },
                "quit" => {
                    eprintln!("Quitting.");
                    process::exit(0);
                }
                _ => {
                    eprintln!("Unknown command: {}", command);
                }
            }
        }
    }
}

fn uci_handshake() {
    println!("id name Blaze");
    println!("id author Dominik Schiwietz");
    
    // Signal that the initial handshake is done
    println!("uciok");
}

fn is_ready() {
    println!("readyok");
}


fn handle_position_command(board: &mut Board, commands: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut moves_start_index = None;

    if commands.get(1) == Some(&"startpos") {
        *board = Board::start();
        moves_start_index = Some(3); // "position", "startpos", "moves"
    } else if commands.get(1) == Some(&"fen") {
        // The FEN string can contain spaces, so we need to reconstruct it.
        // "position", "fen", "rnbqkbnr/...", "w", "KQkq", "-", "0", "1", "moves"
        let fen_parts: Vec<&str> = commands.iter().skip(2).take_while(|&&c| c != "moves").cloned().collect();
        let fen = fen_parts.join(" ");
        *board = Board::from_str(&fen).unwrap();
        moves_start_index = Some(2 + fen_parts.len() + 1); // "position", "fen", <fen_parts>, "moves"
    }

    if let Some(start_index) = moves_start_index {
        if commands.get(start_index - 1) == Some(&"moves") {
            for move_str in commands.iter().skip(start_index) {
                let mov = Move::from_lan(board, move_str);
                board.apply_move(&mov);
            }
        }
    }

    Ok(())
}


// This function will spawn the search thread
fn handle_go_command(board: &Board, commands: &[&str]) {
    let mut search_board = board.clone();

    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_clone = stop_signal.clone();

    if commands.len() == 3 && commands[1] == "perft"{
        // Clone board and parse depth before spawning the thread to avoid borrowing issues
        let mut search_board = board.clone();
        let depth = commands[2].parse::<u8>().unwrap();
        thread::spawn(move || {
            let total_nodes = perft(&mut search_board, depth);
            println!("{}", total_nodes);
        });
    }else{
        thread::spawn(move || {
            let best_move = search_in_thread(&mut search_board, stop_clone);
            match best_move {
                Some(mv) => {
                    println!("bestmove {}", mv.to_lan());
                }
                None => {
                    println!("bestmove n/a");
                }
            }
        });
    }

}


fn search_in_thread(board: &mut Board, stop_signal: Arc<AtomicBool>) -> Option<Move>{
   let ret = None;
   ret
}

pub fn post_uci_info(info: UciInfo) {
    // Start with the base "info" command
    let mut uci_string = String::from("info");

    // Add depth and selective depth
    uci_string.push_str(&format!(" depth {}", info.depth));
    uci_string.push_str(&format!(" seldepth {}", info.seldepth));

    // Add score. It can be either centipawns (cp) or mate in X.
    if let Some(mate_in) = info.score_mate {
        uci_string.push_str(&format!(" score mate {}", mate_in));
    } else if let Some(centipawns) = info.score_cp {
        uci_string.push_str(&format!(" score cp {}", centipawns));
    }

    // Add node count, nodes per second, and time
    uci_string.push_str(&format!(" nodes {}", info.nodes));
    uci_string.push_str(&format!(" nps {}", info.nps));
    uci_string.push_str(&format!(" time {}", info.time.as_millis()));

    // Add the Principal Variation (PV)
    if !info.pv.is_empty() {
        uci_string.push_str(" pv");
        for mv in info.pv {
            uci_string.push_str(&format!(" {}", mv.to_lan()));
        }
    }

    // Print the final string to standard output, which the GUI will read.
    println!("{}", uci_string);
}