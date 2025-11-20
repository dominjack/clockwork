/// ######################################################
/// ################### TESTING ##########################
/// ######################################################

#[cfg(test)]
mod tests {
    use chess_core::types::board::board::Board;
    use chess_core::types::color::Color;
    use chess_core::types::piece::{Piece, PieceType};
    use chess_core::types::square::Square;

    struct FenTestCase {
        fen: &'static str,
        // Expected bitboards for all 12 piece types, in the order defined above
        expected_piece_bbs: [u64; 6],
        expected_active_color: Color,
        expected_castling_raw: u8,
        expected_en_passant: Option<Square>,
        expected_halfmove: u8,
        expected_fullmove: usize,
    }

    const FEN_TEST_SUITE: &[FenTestCase] = &[
        FenTestCase {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            expected_piece_bbs: [
                0x000000000000FF00 | 0x00FF000000000000,
                0x0000000000000042 | 0x4200000000000000,
                0x0000000000000024 | 0x2400000000000000,
                0x0000000000000081 | 0x8100000000000000,
                0x0000000000000008 | 0x0800000000000000,
                0x0000000000000010 | 0x1000000000000000,
            ],
            expected_active_color: Color::White,
            expected_castling_raw: 15, // KQkq (binary 1111)
            expected_en_passant: None, // Assuming Square::None exists
            expected_halfmove: 0,
            expected_fullmove: 1,
        },
        FenTestCase {
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            expected_piece_bbs: [
                0x000000001000EF00 | 0x00FF000000000000,
                0x0000000000000042 | 0x4200000000000000,
                0x0000000000000024 | 0x2400000000000000,
                0x0000000000000081 | 0x8100000000000000,
                0x0000000000000008 | 0x0800000000000000,
                0x0000000000000010 | 0x1000000000000000,
            ],
            expected_active_color: Color::Black,
            expected_castling_raw: 15,             // KQkq
            expected_en_passant: Some(Square::E3), // Assuming Square::E3 exists or its representation
            expected_halfmove: 0,                  // Pawn move resets halfmove clock
            expected_fullmove: 1,                  // Fullmove number increments after Black's move
        },
    ];

    #[test]
    fn test_fen_parsing() {
        for (i, case) in FEN_TEST_SUITE.iter().enumerate() {
            let board = Board::from_fen(case.fen);

            let context_msg = format!("Test case #{} (FEN: '{}')", i, case.fen);

            match board {
                Ok(board) => {
                    for piece_idx in 0..PieceType::COUNT {
                        assert_eq!(
                            board.pieces[piece_idx], case.expected_piece_bbs[piece_idx],
                            "Bitboard mismatch for piece index {}. {}",
                            piece_idx, context_msg
                        );
                    }
                    assert_eq!(
                        board.state.color, case.expected_active_color,
                        "Active color mismatch. {}",
                        context_msg
                    );
                    assert_eq!(
                        board.state.castling.0, case.expected_castling_raw,
                        "Castling rights mismatch. {}",
                        context_msg
                    );
                    assert_eq!(
                        board.state.en_passant, case.expected_en_passant,
                        "En passant square mismatch. {}",
                        context_msg
                    );
                    assert_eq!(
                        board.state.halfmove_clock, case.expected_halfmove,
                        "Halfmove clock mismatch. {}",
                        context_msg
                    );
                    assert_eq!(
                        board.state.fullmove_number, case.expected_fullmove,
                        "Fullmove number mismatch. {}",
                        context_msg
                    );
                }
                Err(e) => panic!("Failed to parse FEN: {}. Error: {:?}", case.fen, e),
            }
        }
    }
}
