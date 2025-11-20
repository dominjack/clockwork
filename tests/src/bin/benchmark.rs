use tests::performance::performance::test;

/// This binary runs the performance benchmark for the chess_core crate.
fn main() {
    // A variety of positions is crucial for a good average.
    // Use openings, middlegames, and endgames with different levels of complexity.
    test();
}
