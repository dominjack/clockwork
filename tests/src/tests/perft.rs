#[cfg(test)]
mod perft_test {
    use chess_core::types::board::board::Board;
    use clockwork::search::perft::perft;
    use std::str::FromStr;

    #[test]
    fn perft_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        // Expected node counts for depths 1-5
        let expected_nodes = [
            20,          // Depth 1
            400,         // Depth 2
            8_902,       // Depth 3
            197_281,     // Depth 4
            4_865_609,   // Depth 5
            119_060_324, // Depth 6
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_kiwipete() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let expected_nodes = [
            48,          // Depth 1
            2_039,       // Depth 2
            97_862,      // Depth 3
            4_085_603,   // Depth 4
            193_690_690, // Depth 5
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_position_3() {
        // Note: This FEN has a Rook on b4, which is different from some other "Position 3" FENs.
        // The results below are for this specific FEN.
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let expected_nodes = [
            14,      // Depth 1
            191,     // Depth 2
            2_812,   // Depth 3
            43_238,  // Depth 4
            674_624, // Depth 5
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_position_4() {
        // NOTE: Corrected FEN to the standard "Position 4" from Chess Programming Wiki.
        // The FEN in your test file seemed to have typos (e.g., BBP1P3).
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let expected_nodes = [
            6,          // Depth 1
            264,        // Depth 2
            9_467,      // Depth 3
            422_333,    // Depth 4
            15_833_292, // Depth 5
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_position_5() {
        // This is a known, tricky FEN.
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let expected_nodes = [
            44,         // Depth 1
            1_486,      // Depth 2
            62_379,     // Depth 3
            2_103_487,  // Depth 4
            89_941_194, // Depth 5
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_position_6() {
        // Using the FEN from your test file.
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let expected_nodes = [
            46,          // Depth 1
            2_079,       // Depth 2
            89_890,      // Depth 3
            3_894_594,   // Depth 4
            164_075_551, // Depth 5
        ];

        for (i, expected) in expected_nodes.iter().enumerate() {
            let depth = (i + 1) as u8;
            let mut board = Board::from_fen(fen).unwrap();
            let num = perft(&mut board, depth as u8);
            assert_eq!(
                num.unwrap(),
                *expected,
                "Initial position: FEN {} at depth {}",
                fen,
                depth
            );
        }
    }

    #[test]
    fn perft_illegal_ep_move_1() {
        let fen = "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1";
        let depth = 6;
        let expected = 1134888;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Illegal ep move #1: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_illegal_ep_move_2() {
        let fen = "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1";
        let depth = 6;
        let expected = 1015133;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Illegal ep move #2: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_ep_capture_checks_opponent() {
        let fen = "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1";
        let depth = 6;
        let expected = 1440467;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "EP Capture Checks Opponent: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_short_castling_gives_check() {
        let fen = "5k2/8/8/8/8/8/8/4K2R w K - 0 1";
        let depth = 6;
        let expected = 661072;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Short Castling Gives Check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_long_castling_gives_check() {
        let fen = "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1";
        let depth = 6;
        let expected = 803711;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Long Castling Gives Check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_castle_rights() {
        let fen = "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1";
        let depth = 4;
        let expected = 1274206;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Castle Rights: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_castling_prevented() {
        let fen = "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1";
        let depth = 4;
        let expected = 1720476;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Castling Prevented: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_promote_out_of_check() {
        let fen = "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1";
        let depth = 6;
        let expected = 3821001;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Promote out of Check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_discovered_check() {
        let fen = "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1";
        let depth = 5;
        let expected = 1004658;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Discovered Check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_promote_to_give_check() {
        let fen = "4k3/1P6/8/8/8/8/K7/8 w - - 0 1";
        let depth = 6;
        let expected = 217342;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Promote to give check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_under_promote_to_give_check() {
        let fen = "8/P1k5/K7/8/8/8/8/8 w - - 0 1";
        let depth = 6;
        let expected = 92683;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Under Promote to give check: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_self_stalemate() {
        let fen = "K1k5/8/P7/8/8/8/8/8 w - - 0 1";
        let depth = 6;
        let expected = 2217;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Self Stalemate: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_stalemate_and_checkmate_1() {
        let fen = "8/k1P5/8/1K6/8/8/8/8 w - - 0 1";
        let depth = 7;
        let expected = 567584;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Stalemate & Checkmate 1: FEN {} at depth {}",
            fen,
            depth
        );
    }

    #[test]
    fn perft_stalemate_and_checkmate_2() {
        let fen = "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1";
        let depth = 4;
        let expected = 23527;
        let mut board = Board::from_fen(fen).unwrap();
        let num = perft(&mut board, depth);
        assert_eq!(
            num.unwrap(),
            expected,
            "Stalemate & Checkmate 2: FEN {} at depth {}",
            fen,
            depth
        );
    }
}
