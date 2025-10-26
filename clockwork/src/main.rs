use clockwork::engine::engine::negamax;
use chess_core::types::{board::board::Board, moves::Move};
use std::time::{Duration, Instant};

fn main() {
    let mut board = Board::start();
    let st = Instant::now();
    let out = negamax(&mut board, 6);
    let duration = st.elapsed();
    println!("{}, {}, searched {} Nodes in {}s ({}kNps)", out.0, out.1.unwrap_or(Move::NULL).to_string(), out.2, duration.as_secs_f32(), out.2 as f32/duration.as_secs_f32()/1000.);
}
