use chess_core::types::{board::board::Board, moves::Move};

use crate::{
    api::uci::uci::{UciUpdate, post_uci_bestmove, post_uci_info},
    search::{
        absearch::ABSearch, config::SearchParams, score::Score, thread::SearchThread,
        transposition::TranspositionTable, variation::Variation,
    },
};

pub fn iterative_absearch(mut board: Board, mut thread: SearchThread) {
    let max_depth = thread.tc.max_depth();
    let mut alpha = -Score::INFINITY;
    let mut beta = Score::INFINITY;

    let mut bestmove = None;
    thread.tt.lock().unwrap().age();

    for depth in 1..(max_depth + 1) as u8 {
        //thread.tt.lock().unwrap().clear();
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
