use chess_core::types::{board::board::Board, moves::Move};

use crate::{
    api::uci::uci::{UciUpdate, post_uci_bestmove, post_uci_info},
    search::{
        absearch::ABSearch, config::SearchParams, score::Score, thread::SearchThread,
        transposition::TranspositionTable, variation::Variation,
    },
};

/// Performs an iterative deepening alpha-beta search.
///
/// This function starts with a search of depth 1 and then iteratively increases the depth,
/// using the results from the previous search to improve move ordering for the next one.
/// It sends UCI info updates after each depth is completed.
///
/// # Arguments
/// * `board` - The board state to search from.
/// * `thread` - The search thread, which contains the time control and transposition table.
pub fn iterative_absearch(mut board: Board, mut thread: SearchThread) {
    let max_depth = thread.tc.max_depth();
    let mut alpha = -Score::INFINITY;
    let mut beta = Score::INFINITY;

    let mut bestmove = None;

    for depth in 1..(max_depth + 1) as u8 {
        let score =
            ABSearch::new(&mut board, &mut thread).search(SearchParams::new(alpha, beta, depth));

        if thread.is_over() {
            break;
        }

        let mut pv = Variation::new();

        thread.get_pv(&mut board, depth as usize, &mut pv);
        bestmove = pv.get_first();

        let update: UciUpdate = UciUpdate {
            depth,
            seldepth: thread.seldepth as u8,
            score: score,
            nodes: thread.nodes as u64,
            nps: thread.nodes as f64 / thread.start_time.elapsed().as_secs_f64(),
            time: thread.start_time.elapsed(),
            pv: pv,
        };
        post_uci_info(update);
    }

    if let Some(mv) = bestmove {
        post_uci_bestmove(mv);
    } else {
        panic!();
    }
}
