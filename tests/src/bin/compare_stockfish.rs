use tests::stockfish::stockfish::test_perft_deep;

/// This binary runs a deep perft test and compares the results with Stockfish.
/// It is used to verify the correctness of the move generation.
fn main() {
    test_perft_deep("8/8/8/1Ppp3r/RK3p1k/8/4P1P1/8 w - c6 1 3", 5);
}
