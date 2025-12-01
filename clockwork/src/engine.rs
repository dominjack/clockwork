use crate::api::uci::uci::UciCommand;
use crate::evaluate::evaluate::{evaluate_for, evaluate_relative};
use crate::search::absearch::ABSearch;
use crate::search::config::EngineConfig;
use crate::search::iterative::iterative_absearch;
use crate::search::perft::{self, start_perft};
use crate::search::thread::SearchThread;
use crate::search::timecontrol::TimeControl;
use crate::search::transposition::TranspositionTable;
use chess_core::types::board::board::Board;
use chess_core::types::board::parse::FenError;
use chess_core::types::moves::Move;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Engine {
    pub board: Board,
    pub terminator: Arc<AtomicBool>,
    pub config: EngineConfig,
    pub cache: Arc<Mutex<TranspositionTable>>,
}

impl Engine {
    pub fn new() -> Self {
        let config: EngineConfig = Default::default();
        Engine {
            board: Board::start(),
            terminator: Default::default(),
            config: Default::default(),
            cache: Arc::new(Mutex::new(TranspositionTable::new(config.tt_size_mb))),
        }
    }

    pub fn set_position(&mut self, fen: String, moves: Vec<String>) -> Result<(), FenError> {
        self.board = Board::from_fen(&fen)?;
        for mv in moves {
            let _mv = Move::from_lan(&self.board, &mv);
            self.board.apply_move(&_mv);
        }
        Ok(())
    }

    pub fn set_terminator(&mut self, set: bool) {
        self.terminator.store(set, Ordering::Relaxed);
    }

    pub fn reset(&mut self) {
        self.board = Board::start();
        self.set_terminator(false);
    }

    pub fn go(&mut self, time_control: TimeControl) {
        self.terminator.store(false, Ordering::Relaxed);
        let mut thread =
            SearchThread::new(time_control, self.cache.clone(), self.terminator.clone());
        let mut board = self.board.clone();

        thread::spawn(move || {
            iterative_absearch(board, thread);
        });
    }

    pub fn perft(&mut self, depth: u8) {
        let mut board = self.board.clone();
        thread::spawn(move || {
            start_perft(&mut board, depth);
        });
    }

    pub fn eval(&mut self) {
        println!("{}", evaluate_for(&self.board, self.board.state.color).0)
    }

    pub fn execute(&mut self, command: UciCommand) {
        match command {
            UciCommand::UciNewGame => self.reset(),
            UciCommand::Handshake => uci_handshake(),
            UciCommand::IsReady => println!("readyok"),
            UciCommand::Position { fen, moves } => match self.set_position(fen, moves) {
                Err(_) => println!("Failed to parse position"),
                Ok(_) => (),
            },
            UciCommand::Quit => self.set_terminator(true),
            UciCommand::Stop => self.set_terminator(true),
            UciCommand::SetOption { name, value } => {
                if let Some(val) = value {
                    match name.as_str() {
                        "Hash" => {
                            if let Ok(mb) = val.parse::<usize>() {
                                self.config.tt_size_mb = mb;
                                let mut tt = self.cache.lock().unwrap();
                                *tt = TranspositionTable::new(mb);
                            }
                        }
                        "Threads" => {
                            if let Ok(threads) = val.parse::<usize>() {
                                self.config.threads = threads;
                            }
                        }
                        _ => {}
                    }
                }
            }
            UciCommand::Hash => (),
            UciCommand::Go { time_control } => {
                self.go(time_control);
            }
            UciCommand::Eval => self.eval(),
            UciCommand::Perft { depth } => {
                self.perft(depth);
            }
        }
    }
}

fn uci_handshake() {
    println!("id name Clockwork");
    println!("id author Dominik Schiwietz");
    println!("option name Threads type spin default 1 min 1 max 1");
    println!("option name Hash type spin default 16 min 1 max 1024");
    println!("uciok");
}
