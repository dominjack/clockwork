use crate::magic::magicgen::generate_all_magics_and_attacks;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// FIXME: The file paths are hardcoded. They should be relative to the project root.
const MAGIC_FILE_PATH: &str = "chess_core/src/data/magics.rs";
const ATTACK_FILE_PATH: &str = "chess_core/src/data/attacks.rs";

pub fn precalc_magics_to_file() -> std::io::Result<()> {
    let (rook_magics, rook_attacks, bishop_magics, bishop_attacks) =
        generate_all_magics_and_attacks();

    // --- Write magics.rs ---
    let path = Path::new(MAGIC_FILE_PATH);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write the file header.
    writeln!(
        writer,
        "// This file is auto-generated. Do not edit manually."
    )?;
    writeln!(
        writer,
        "// Run the program containing precalc_magics_to_file to regenerate it."
    )?;
    writeln!(writer)?;
    writeln!(writer, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(writer, "#[repr(C)] // Optional: for layout guarantees")?;
    writeln!(writer, "pub struct MagicEntry {{")?;
    writeln!(writer, "    pub mask: u64,")?;
    writeln!(writer, "    pub magic: u64,")?;
    writeln!(writer, "    pub shift: u8,")?;
    writeln!(writer, "    pub offset: usize,")?;
    writeln!(writer, "    pub size: usize,")?;
    writeln!(writer, "}}\n")?;

    writeln!(writer, "impl MagicEntry {{")?;
    writeln!(
        writer,
        "    #[rustfmt::skip] // Optional: if lines get too long"
    )?;
    writeln!(
        writer,
        "    pub const fn new(mask: u64, magic: u64, shift: u8, offset: usize, size: usize) -> Self {{"
    )?;
    writeln!(
        writer,
        "        Self {{ mask, magic, shift, offset, size }}"
    )?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}\n")?;

    // Write the ROOK_MAGICS array.
    writeln!(
        writer,
        "#[rustfmt::skip] // Optional: to prevent reformatting"
    )?;
    writeln!(
        writer,
        "pub static ROOK_MAGICS: [MagicEntry; {}] = [",
        rook_magics.len()
    )?;
    for (index, entry) in rook_magics.iter().enumerate() {
        writeln!(
            writer,
            "    MagicEntry::new({:#018X}, {:#018X}, {}, {}, {}), // Index {}",
            entry.mask.0, entry.magic, entry.shift, entry.offset, entry.size, index
        )?;
    }
    writeln!(writer, "];")?;

    // Write the BISHOP_MAGICS array.
    writeln!(
        writer,
        "#[rustfmt::skip] // Optional: to prevent reformatting"
    )?;
    writeln!(
        writer,
        "pub static BISHOP_MAGICS: [MagicEntry; {}] = [",
        bishop_magics.len()
    )?;
    for (index, entry) in bishop_magics.iter().enumerate() {
        writeln!(
            writer,
            "    MagicEntry::new({:#018X}, {:#018X}, {}, {}, {}), // Index {}",
            entry.mask.0, entry.magic, entry.shift, entry.offset, entry.size, index
        )?;
    }
    writeln!(writer, "];")?;

    writer.flush()?;

    // --- Write attacks.rs ---
    let path = Path::new(ATTACK_FILE_PATH);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file); // Use a buffered writer for efficiency

    // Write the file header.
    writeln!(
        writer,
        "// This file is auto-generated. Do not edit manually."
    )?;
    writeln!(
        writer,
        "// Run the program containing precalc_magics_to_file to regenerate it."
    )?;
    writeln!(writer)?;

    // Write the BISHOP_ATTACKS array.
    writeln!(
        writer,
        "pub static BISHOP_ATTACKS: [u64; {}] = [",
        bishop_attacks.len()
    )?;
    for (_index, entry) in bishop_attacks.iter().enumerate() {
        writeln!(writer, "    {:#018X},", entry.0)?;
    }
    writeln!(writer, "];")?;

    writeln!(writer)?;

    // Write the ROOK_ATTACKS array.
    writeln!(
        writer,
        "pub static ROOK_ATTACKS: [u64; {}] = [",
        rook_attacks.len()
    )?;
    for (_index, entry) in rook_attacks.iter().enumerate() {
        writeln!(writer, "    {:#018X},", entry.0)?;
    }
    writeln!(writer, "];")?;
    Ok(())
}
