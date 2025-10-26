use std::path::Path;
use std::str::FromStr;
use chess_core::stockfish::stockfish;
use chess_core::types::board::board::Board;
use chess_core::types::board::internalstate::GameState;
use chess_core::types::board::{self, movegen};
use chess_core::types::lists::MoveList;
use chess_core::types::moves::{self, Move};
use std::time::{Duration, Instant};
use chess_core::engine::perft::perft;
use chess_core::types::square::Square;
use chess_core::types::moves::MoveType;
use chess_core::types::piece::Piece;
use chess_core::types::color::Color;
use chess_core::types::bitboard::Bitboard;
use chess_core::performance::crosschecksf::find_wrong_positions_perft;
use chess_core::types::board::transposition::TranspositionTable;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use chess_core::types::board::parse_pgn::{PgnGame};
//use blaze::types::game_result::GameResult;






fn main() {
    //uci_listener();
    //let mut num_nodes:u64 = 0;
    //let mut board = Board::from_str("6k1/prp1Rpp1/7r/8/5PKp/P7/2P4P/8 b - - 3 37").unwrap();

    //let mut board = Board::start();
    //board.apply_move(&Move::new_from_squares(Square::D8, Square::A5, MoveType::Normal));


    //iterative_deepening_search(&mut board, Duration::from_secs(10), Arc::new(AtomicBool::new(false)));
    //let num_nodes = perft(&mut board, 6);
    
    


    //println!("Searched {} Nodes in {}s ({}kN/s)", num_nodes,  st.elapsed().as_secs_f32(), num_nodes as u128 / std::cmp::max(st.elapsed().as_millis(), 1));
    //let path = Path::new("/Users/dominik/Documents/code/chess/blaze/src/nn/training/data/lichess_db_standard_rated_2013-01.pgn");
    //let games = load_pgn_games_from_file(path).expect("Failed to load PGN games");
    // You can now use `games` as needed, or remove this line if not needed.
}   
