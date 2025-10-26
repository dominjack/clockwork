use crate::magic::magicgen::generate_magics_attack_tables;
use std::fs::{File};
use std::io::{BufWriter, Write};
use std::path::Path;

const MAGIC_FILE_PATH: &str = "/Users/dominik/Documents/code/chess/blaze/src/data/magics.rs";
const ATTACK_FILE_PATH: &str = "/Users/dominik/Documents/code/chess/blaze/src/data/attacks.rs";

pub fn precalc_magics_to_file() -> std::io::Result<()> {
    // Generate and write ROOK_MAGICS to the file.
    let rook_magics_attacks = generate_magics_attack_tables(true);
    let rook_magics = rook_magics_attacks.0; 
    let rook_attacks = rook_magics_attacks.1; 

     // Generate and write BISHOP_MAGICS to the file.
    let bishop_magics_attacks = generate_magics_attack_tables(false);
    let bishop_magics = bishop_magics_attacks.0; 
    let bishop_attacks = bishop_magics_attacks.1; 


    // Create or truncate the output file.
    let path = Path::new(MAGIC_FILE_PATH);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file); // Use a buffered writer for efficiency


    // Write the header and MagicEntry struct definition to the file.
    writeln!(writer, "// This file is auto-generated. Do not edit manually.")?;
    writeln!(writer, "// Run the program containing precalc_magics_to_file to regenerate it.")?;
    writeln!(writer)?;
    writeln!(writer, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(writer, "#[repr(C)] // Optional: for layout guarantees")?;
    writeln!(writer, "pub struct MagicEntry {{")?;
    writeln!(writer, "    pub mask: u64,")?; // Storing mask as u64 directly
    writeln!(writer, "    pub magic: u64,")?;
    writeln!(writer, "    pub shift: u8,")?;
    writeln!(writer, "    pub offset: usize,")?;
    writeln!(writer, "    pub size: usize,")?;
    writeln!(writer, "}}\n")?;

    // Write the const constructor for MagicEntry to the file.
    writeln!(writer, "impl MagicEntry {{")?;
    writeln!(writer, "    #[rustfmt::skip] // Optional: if lines get too long")?;
    writeln!(writer, "    pub const fn new(mask: u64, magic: u64, shift: u8, offset: usize, size: usize) -> Self {{")?;
    writeln!(writer, "        Self {{ mask, magic, shift, offset, size }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}\n")?;

    
    writeln!(writer, "#[rustfmt::skip] // Optional: to prevent reformatting")?;
    writeln!(writer, "pub static ROOK_MAGICS: [MagicEntry; {}] = [", rook_magics.len())?;
    for (index, entry) in rook_magics.iter().enumerate() {
        writeln!(
            writer,
            "    MagicEntry::new({:#018X}, {:#018X}, {:2}, {:5}, {:3}), // Index {:2}",
            entry.mask.0, // Assuming entry.mask is a Bitboard struct with a .0 field for the u64 value
            entry.magic,
            entry.shift,
            entry.offset,
            entry.size,
            index
        )?;
    }
    writeln!(writer, "];\n")?;

   
    writeln!(writer, "#[rustfmt::skip] // Optional: to prevent reformatting")?;
    writeln!(writer, "pub static BISHOP_MAGICS: [MagicEntry; {}] = [", bishop_magics.len())?;
    for (index, entry) in bishop_magics.iter().enumerate() {
        writeln!(
            writer,
            "    MagicEntry::new({:#018X}, {:#018X}, {:2}, {:5}, {:3}), // Index {:2}",
            entry.mask.0, // Assuming entry.mask is a Bitboard struct with a .0 field for the u64 value
            entry.magic,
            entry.shift,
            entry.offset,
            entry.size,
            index
        )?;
    }
    writeln!(writer, "];")?;

    // Ensure all buffered data is written to the file.
    writer.flush()?;


    let path = Path::new(ATTACK_FILE_PATH);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file); // Use a buffered writer for efficiency


    // Write Bishop attacks
    writeln!(writer, "// This file is auto-generated. Do not edit manually.")?;
    writeln!(writer, "// Run the program containing precalc_magics_to_file to regenerate it.")?;
    writeln!(writer)?;
    writeln!(writer, "pub static BISHOP_ATTACKS: [u64; {}] = [", bishop_attacks.len())?;
    for (_index, entry) in bishop_attacks.iter().enumerate() {
        writeln!(
            writer,
            "    {:#018X},",
            entry.0
        )?;
    }
    writeln!(writer, "];\n")?;

    writeln!(writer)?;
    writeln!(writer, "pub static ROOK_ATTACKS: [u64; {}] = [", rook_attacks.len())?;
    for (_index, entry) in rook_attacks.iter().enumerate() {
        writeln!(
            writer,
            "    {:#018X},",
            entry.0
        )?;
    }
    writeln!(writer, "];\n")?;
    Ok(())
}
