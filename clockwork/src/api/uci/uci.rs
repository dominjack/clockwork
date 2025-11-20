use crate::search::config::EngineConfig;
use crate::search::perft::perft;
use crate::search::score::Score;
use crate::search::timecontrol::TimeControl;
use crate::search::variation::Variation;
use chess_core::types::board::board::Board;
use chess_core::types::color::Color;
use chess_core::types::moves::Move;
use std::error::Error;
use std::io;
use std::process;
use std::str::FromStr;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

/// A struct to hold the information that is sent to the GUI during a search.
#[derive(PartialEq, Debug)]
pub enum UciCommand {
    Handshake,
    IsReady,
    UciNewGame,
    Position { fen: String, moves: Vec<String> },
    Go { time_control: TimeControl },
    SetOption,
    Stop,
    Quit,
    Hash,
    Eval,
    Perft { depth: u8 },
}

pub struct UciUpdate {
    pub depth: u8,
    pub seldepth: u8,
    pub score: Score,
    pub nodes: u64,
    pub nps: f64,
    pub time: Duration,
    pub pv: Variation,
}

pub fn parse_uci(cmd: &str, color: Color) -> Result<UciCommand, ()> {
    let commands: Vec<&str> = cmd.trim().split_whitespace().collect();
    if commands.is_empty() {
        return Err(());
    }
    match commands[0] {
        "uci" => Ok(UciCommand::Handshake),
        "isready" => Ok(UciCommand::IsReady),
        "ucinewgame" => Ok(UciCommand::UciNewGame),
        "position" => parse_position(&commands[1..]),
        "go" => parse_go(&commands[1..], color),
        "setoption" => Ok(UciCommand::SetOption),
        "stop" => Ok(UciCommand::Stop),
        "quit" => Ok(UciCommand::Quit),
        "hash" => Ok(UciCommand::Hash),
        "eval" => Ok(UciCommand::Eval),
        "perft" => parse_perft(&commands[1..]),
        _ => Err(()),
    }
}

fn parse_perft(commands: &[&str]) -> Result<UciCommand, ()> {
    Ok(UciCommand::Perft {
        depth: parse(commands.get(0).copied())?,
    })
}

fn parse_go(commands: &[&str], color: Color) -> Result<UciCommand, ()> {
    Ok(UciCommand::Go {
        time_control: parse_time(commands, color)?,
    })
}

fn parse_time(commands: &[&str], color: Color) -> Result<TimeControl, ()> {
    let mut time: u64 = 0;
    let mut increment: u64 = 0;
    let mut moves_left: Option<u64> = None;

    for chunk in commands.chunks(2) {
        let (token, value) = (chunk[0], chunk.get(1).copied());

        match token {
            "infinite" => return Ok(TimeControl::Infinite),
            "depth" => return Ok(TimeControl::Depth(parse(value)?)),
            "movetime" => return Ok(TimeControl::FixedTime(parse(value)?)),

            "wtime" if color == Color::White => time = parse(value)?,
            "btime" if color == Color::Black => time = parse(value)?,
            "winc" if color == Color::White => increment = parse(value)?,
            "binc" if color == Color::Black => increment = parse(value)?,
            "movestogo" => moves_left = Some(parse(value)?),

            _ => continue,
        }
    }
    if time == 0 && increment == 0 {
        return Ok(TimeControl::Infinite);
    }

    match moves_left {
        Some(moves) => Ok(TimeControl::Tournament(time, increment, moves)),
        None => Ok(TimeControl::Incremental(time, increment)),
    }
}

fn parse_position(commands: &[&str]) -> Result<UciCommand, ()> {
    if commands.len() < 2 {
        return Err(());
    }

    let fen = match commands[0] {
        "startpos" => String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        "fen" if commands.len() >= 7 => commands[1..7].join(" "),
        _ => return Err(()),
    };

    let moves = match commands.iter().position(|&t| t == "moves") {
        Some(index) => commands[(index + 1)..]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        None => vec![],
    };

    Ok(UciCommand::Position { fen, moves })
}

/// Posts the search information to the GUI.
pub fn post_uci_info(info: UciUpdate) {
    // 1. Prepare the score string (mate or centipawn)
    let score_string = if let Some(mate_in) = info.score.checkmate_in() {
        format!("score mate {}", mate_in)
    } else {
        format!("score cp {}", info.score.0)
    };

    // 3. Assemble and print the final string in a single call
    let uci_string = format!(
        "info depth {} seldepth {} {} nodes {} nps {:.0} time {} {}",
        info.depth,
        info.seldepth,
        score_string,
        info.nodes,
        info.nps,
        info.time.as_millis(),
        info.pv
    );

    println!("{}", uci_string);
}

/// Posts the best move to the GUI.
pub fn post_uci_bestmove(mv: Move) {
    println!("bestmove {}", mv);
}

fn parse<T: std::str::FromStr>(value: Option<&str>) -> Result<T, ()> {
    value.and_then(|v| v.parse().ok()).ok_or(())
}
