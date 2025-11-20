use chess_core::magic::io::precalc_magics_to_file;

/// This binary generates the magic bitboard numbers and saves them to a file.
/// Magic bitboards are used for efficient generation of sliding piece moves (rooks and bishops).
fn main() {
    println!("Starting magic number generation...");
    precalc_magics_to_file().expect("Failed to calculate magics");
    println!("Magic number generation finished.");
}
