use chess_core::magic::io::{precalc_magics_to_file};


fn main() {
    println!("Starting magic number generation...");
    precalc_magics_to_file().expect("Failed to calculate magics");
    println!("Magic number generation finished.");
    //println!("{:?}", res);
}

//
