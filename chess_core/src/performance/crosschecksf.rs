use std::str::FromStr;
use crate::stockfish::stockfish::UciEngine;
use crate::types::board::board::Board;
use crate::engine::perft::perft;

pub fn find_wrong_positions_perft(ply: u8, fen: &str) {
    let mut board = Board::from_str(fen).unwrap();
    let mut stockfish = UciEngine::new().unwrap();
    stockfish.set_position(fen).unwrap();
    let num_sf = stockfish.perft(ply).unwrap();
    let num_my = perft(&mut board, ply);
    println!("{}  {}", num_my, num_sf);

    if num_my != num_sf{
        if ply == 1{
            println!("Fen: {}", fen);
            println!("{}", board);
            panic!();
        }
        println!("Found mismatch, {} != {}", num_my, num_sf);
        let moves = board.generate_all_moves();
        for mv in moves.iter() {
            let mut _board = board.clone();
            _board.apply_move(mv);
            let _fen = _board.to_fen();
            find_wrong_positions_perft(ply-1, &_fen);
        }
    }
}