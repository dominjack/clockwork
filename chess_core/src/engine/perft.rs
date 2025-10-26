use crate::types::board::board::Board;

pub fn perft(board: &mut Board, ply: u8) -> u64 {
	let mut num_nodes:u64 = 0;
    let moves = board.generate_all_moves();

    if ply == 1 {
        return moves.len() as u64;
    }
    for mv in moves.iter(){
        board.apply_move(mv);
        num_nodes += perft(board, ply-1);
        board.undo_move(mv);
    }
    
    num_nodes
}




/// ######################################################
/// ################### TESTING ##########################
/// ######################################################


#[cfg(test)]
mod legal_move_count_tests {
    use crate::{engine::perft, types::board::board::Board};

    #[test]
    fn perft_1() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 1);
        assert_eq!(num, 20);
    }

    #[test]
    fn perft_2() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 2);
        assert_eq!(num, 400);
    }

    #[test]
    fn perft_3() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 3);
        assert_eq!(num, 8902);
    }

    #[test]
    fn perft_4() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 4);
        assert_eq!(num, 197281);
    }

    #[test]
    fn perft_5() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 5);
        assert_eq!(num, 4865609);
    }

    #[test]
    fn perft_6() {
        let mut board = Board::start();
        let num = perft::perft(&mut board, 6);
        assert_eq!(num, 119060324);
    }


}
