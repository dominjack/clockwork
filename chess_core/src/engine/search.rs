use crate::types::board::board::Board;
use crate::types::moves::Move;
use crate::types::color::Color;
use crate::types::board::transposition::{TranspositionTable, TableEntryFlag};
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
//use crate::api::uci::uci::{UciInfo, post_uci_info};
use crate::types::board::internalstate::GameState;


const MAX_QUIESCENCE_PLY: u8 = 32;
const MAX_DEPTH: i8 = 64;

fn count_hash_occurrences(hashes_array: &[u64; 100], target_hash: u64) -> usize {
    let mut count = 0;
    for &hash in hashes_array.iter() {
        if hash == target_hash {
            count += 1;
        }
    }
    count
}


pub fn negamax(
    board: &mut Board,
    tt: &mut TranspositionTable,
    depth: i8,
    total_depth: i8,
    mut alpha: i32,
    beta: i32,
    end_time: Instant
) -> (i32, Option<Move>, u64, u8) {
    let mut q_depth = 0u8;

    let multiplier = if board.state.color == Color::White { 1 } else { -1 };

    
    // At a leaf node, return the static evaluation.
    if depth == 0 {
        // We only care about the score and nodes from qsearch
        let (score, _, nodes, depth) = quiescence_search(board, alpha, beta, 0, end_time);
        q_depth = q_depth.max(depth);
        return (score, None, nodes, q_depth);
    }

    let zobrist_hash = board.hash();

    if count_hash_occurrences(&board.state.hash_history, zobrist_hash) >= 2{
        return (0, None, 0, 0);
    }

    // Probe the TT for an existing entry for this position
    if let Some(entry) = tt.probe(zobrist_hash) {
        // If the stored search was deep enough, we can use the result
        if entry.depth >= depth {
            match entry.flags {
                // We found an exact score, so we can return it immediately.
                TableEntryFlag::Exact => {
                    return (entry.score as i32, Some(entry.best_move), 1, q_depth);
                }
                // The stored score is a lower bound. It might raise our alpha.
                TableEntryFlag::LowerBound => {
                    alpha = alpha.max(entry.score as i32);
                }
                // The stored score is an upper bound. It might lower our beta.
                TableEntryFlag::UpperBound => {
                    // This case is implicitly handled by the alpha-beta check
                }
                TableEntryFlag::None => {} // Should not happen with a valid entry
            }
            // If the bounds now overlap, we can prune
            if alpha >= beta {
                return (entry.score as i32, Some(entry.best_move), 1, q_depth);
            }

        }
    }

    let mut best_score = i32::MIN + 1; // Use a value slightly above MIN to avoid overflows
    let mut best_move = None;
    let mut moves = board.generate_all_moves();

    match board.state.game_state {
        GameState::BlackWin => {return ((-i32::MAX + (total_depth - depth) as i32) * multiplier, None, 1, q_depth)},
        GameState::WhiteWin => {return ((i32::MAX - (total_depth - depth) as i32) * multiplier, None, 1, q_depth)},
        GameState::Draw => {return (0, None, 1, q_depth)},
        GameState::InProgress => {}
    }
    
    let original_alpha = alpha;
    let mut nodes = 0u64;

    
    
    if let Some(entry) = tt.probe(zobrist_hash) {
        moves.move_to_front(&entry.best_move);
    }

    for mv in moves.iter() {
        if end_time <= Instant::now() {
            /*
            if let Some(_best_move) = best_move {
                let flag = if best_score <= original_alpha {
                    TableEntryFlag::UpperBound // Failed low, score is at most best_score
                } else if best_score >= beta {
                    TableEntryFlag::LowerBound // Failed high (cutoff), score is at least best_score
                } else {
                    TableEntryFlag::Exact      // Score is exact within the alpha-beta window
                };
                tt.store(zobrist_hash, _best_move, best_score as i16, depth, flag);
                return (best_score, best_move, nodes, q_depth);
             */
            return (0, None, 0,0);
            
        }
        board.apply_move(&mv);
        
        let (mut score, _, num, depth) = negamax(board, tt, depth - 1, total_depth, -beta, -alpha, end_time);
        q_depth = q_depth.max(depth);
        nodes += num;
        score = -score;
        board.undo_move(&mv);


        if score > best_score {
            best_score = score;
            best_move = Some(*mv);
        }
        
        alpha = alpha.max(best_score);
        if alpha >= beta {
            break; // Pruning
        }
    }


    let flag = if best_score <= original_alpha {
        TableEntryFlag::UpperBound // Failed low, score is at most best_score
    } else if best_score >= beta {
        TableEntryFlag::LowerBound // Failed high (cutoff), score is at least best_score
    } else {
        TableEntryFlag::Exact      // Score is exact within the alpha-beta window
    };
    
    // We must have a move unless it was a terminal node, handled above.
    if let Some(mv) = best_move {
        tt.store(zobrist_hash, mv, best_score as i16, depth, flag);
    }
    
    (best_score, best_move, nodes, q_depth)
}



pub fn quiescence_search(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    ply: u8, // Tracks the depth of the quiescence search itself
    end_time: Instant
) -> (i32, Option<Move>, u64, u8) { // We return Option<Move> for consistency, but it's often unused
    //if end_time <= Instant::now() {
        //return (0, None, 0, 0);
    //}
    let mut max_q_depth = 0u8;

    let multiplier = if board.state.color == Color::White { 1 } else { -1 };
    
    if ply >= MAX_QUIESCENCE_PLY {
        return (board.eval() * multiplier, None, 1, ply);
    }

    let mut nodes = 1u64;
    // 1. "Stand Pat" Score: First, get the evaluation of the current position.
    // This represents the score we can get if we choose not to make any more captures.
    
    let stand_pat_score = board.eval() * multiplier;

    // 2. Alpha-Beta Pruning check with the stand-pat score.
    // If our static eval is already better than what the opponent can guarantee,
    // we can cut off the search. We assume we can at least reach this score.
    if stand_pat_score >= beta {
        return (stand_pat_score, None, nodes, ply);
    }
    
    // Raise alpha. We can at least achieve the stand-pat score.
    alpha = alpha.max(stand_pat_score);
    let mut best_move = None;

    // 3. Generate and search only capture moves.
    // It's CRITICAL to order these moves, e.g., using MVV-LVA.
    // Most Valuable Victim - Least Valuable Attacker is highly effective here.
    let mut noisy = board.generate_noisy_moves(); // You'll need to implement this helper
    // TODO: Sort `captures` using MVV-LVA for huge performance gains.

    for mv in noisy.iter() {
        board.apply_move(mv);
        let (mut score, _, num, q_depth) = quiescence_search(board, -beta, -alpha, ply + 1, end_time);
        max_q_depth = max_q_depth.max(q_depth);
        score = -score;
        nodes += num;
        board.undo_move(mv);

        if score > stand_pat_score {
            if score >= beta {
                // This capture is "too good" and the opponent will avoid this line.
                // Return beta as this is a lower bound on the score.
                return (beta, Some(*mv), nodes, max_q_depth);
            }
            // A new best capture was found.
            alpha = alpha.max(score);
            best_move = Some(*mv);
        }
    }
    
    // If a capture improved our position, return that new alpha score.
    // Otherwise, return the original stand-pat score.
    (alpha, best_move, nodes, max_q_depth)
}

// In your main search initiator function (the one that calls negamax)

use std::time::Instant;

pub fn iterative_deepening_search(board: &mut Board, search_time_limit: Duration, stop_signal: Arc<AtomicBool>) -> Option<Move>{
    let start_time = Instant::now();
    let end_time = start_time + search_time_limit;
    let mut total_nodes = 0;
    let mut principal_variation: Vec<Move> = Vec::new();
    let mut tt = TranspositionTable::new(64);


    for depth in 1i8..=MAX_DEPTH { // MAX_DEPTH is a constant like 64
        if start_time.elapsed() >= search_time_limit || stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        
        // --- Call your core search function ---
        let (mut score, best_move_for_depth, nodes_this_iteration, q_depth) = negamax(
            board,
            &mut tt, // your transposition table
            depth,
            depth, 
            i32::MIN + 1,
            i32::MAX ,
            end_time,
        );
        total_nodes += nodes_this_iteration;
        

        //let multiplier = if board.state.color == Color::White { 1 } else { -1 };
        //score *= multiplier;


        // After each depth, update the PV and post the UCI info
        if let Some(mv) = best_move_for_depth {
             // You need to reconstruct the PV from your transposition table
             principal_variation = vec!(best_move_for_depth.clone().unwrap_or_default());
        } else {
        }
        // --- Check for time up ---


        // --- Prepare and send the UCI update ---
        let elapsed_time = start_time.elapsed();
        let nps = if elapsed_time.as_millis() > 0 {
            (total_nodes as u128 / elapsed_time.as_millis() *1000) as u64
        } else {
            0 // Avoid division by zero
        };

        let mut score_mate: Option<i32> = None;
        if score <= -i32::MAX + 512{
            score_mate = Some((score + i32::MAX))
    
        }else if score >= i32::MAX - 512{
            score_mate = Some(-(score - i32::MAX))
        }

        let uci_update = UciInfo {
            depth: depth as u8,
            seldepth: q_depth, // You would need to track this from qsearch
            score_cp: Some(score), // Or parse for mate scores
            score_mate: score_mate,      //
            nodes: total_nodes,
            nps,
            time: elapsed_time,
            pv: principal_variation.clone(),
        };

        post_uci_info(uci_update);

        
    }

    // Finally, send the best move found from the last completed iteration
    if let Some(best_move) = principal_variation.get(0) {
        return Some(*best_move)
    }else{
        return None
    }
}