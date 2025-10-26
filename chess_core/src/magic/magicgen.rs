use crate::types::bitboard::Bitboard;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
    pub offset: usize, // Offset into a global attack table
    pub size: usize,   // Number of entries for this square's attack table
}


pub fn generate_rook_blocker_mask(sq: u8) -> Bitboard {
    let mut mask = Bitboard::new(0);
    let r = sq / 8;
    let f = sq % 8;

    for i in 1..7 {
        if i != f { mask.set_bit(r * 8 + i); }
    }
   
    for i in 1..7 {
        if i != r { mask.set_bit(i * 8 + f); }
    }
    if r != 0 { 
        mask = mask & !0b11111111;
    }
    if r != 7 { 
        mask = mask & !(0b11111111 << 8*7);
    }
    if f != 0 { 
        mask = mask & !0b100000001000000010000000100000001000000010000000100000001;
    }
    if f != 7 { 
        mask = mask & !(0b100000001000000010000000100000001000000010000000100000001 << 7);
    }
    mask
}

pub fn generate_bishop_blocker_mask(sq: u8) -> Bitboard {
    let mut mask = Bitboard::new(0);
    let mut t1 = true;
    let mut t2 = true;
    let mut t3 = true;
    let mut t4 = true;
    

    for i in 1..7 {
            if sq>i*7 && t1{
                if mask.is_edge(sq-i*7){
                    t1 = false;
                }
                if t1{mask.set_bit(sq-i*7)};
            }
            if sq+i*7<64 && t2{
                if mask.is_edge(sq+i*7){
                    t2 = false;
                }
                if t2{mask.set_bit(sq+i*7)};
            }
            if sq>i*9 && t3{
                if mask.is_edge(sq-i*9){
                    t3 = false;
                }
                if t3{mask.set_bit(sq-i*9)};
            }
            if sq+i*9<64 && t4{
                if mask.is_edge(sq+i*9){
                    t4 = false;
                }
                if t4{mask.set_bit(sq+i*9)};
            }
    }
    mask = mask & !0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;
    mask
}

pub fn get_blocker_subsets(mask: Bitboard) -> Vec<Bitboard> {
    let mut subsets = Vec::new();
    let mut subset = Bitboard::new(0);
    loop {
        subsets.push(subset); // Add the current subset
        subset = subset-mask & mask; // Get the next subset
        if subset == 0 { // We've looped back to the start or handled the empty set
            break;
        }
    }
    // Ensure the empty set is included if not already
    if !subsets.contains(&Bitboard::new(0)) {
        subsets.push(Bitboard::new(0));
    }
    // And the full mask itself
    if !subsets.contains(&mask) && mask != 0 {
        // This logic might need refinement depending on the loop's exact behavior with the full mask
    }
    subsets.sort(); // Optional, for consistency
    subsets.dedup(); // Optional, for consistency
    subsets
}

pub fn calculate_rook_attacks(sq: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::new(0);
    let r_start = sq / 8;
    let f_start = sq % 8;

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // (df, dr)

    for (df, dr) in dirs.iter() {
        for i in 1..8 {
            let next_f = f_start as i8 + df * i;
            let next_r = r_start as i8 + dr * i;

            if next_f >= 0 && next_f < 8 && next_r >= 0 && next_r < 8 {
                let current_sq_idx = (next_r * 8 + next_f) as usize;
                attacks |= 1u64 << current_sq_idx;
                if (blockers & (1u64 << current_sq_idx)) != 0 { // Hit a blocker
                    break;
                }
            } else { // Off board
                break;
            }
        }
    }

    attacks
}

fn calculate_bishop_attacks(sq: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::new(0);
    let r_start = sq / 8;
    let f_start = sq % 8;

    let dirs = [(1, 1), (-1, 1), (1, -1), (-1, -1)]; // (df, dr)

    for (df, dr) in dirs.iter() {
        for i in 1..8 {
            let next_f = f_start as i8 + df * i;
            let next_r = r_start as i8 + dr * i;

            if next_f >= 0 && next_f < 8 && next_r >= 0 && next_r < 8 {
                let current_sq_idx = (next_r * 8 + next_f) as usize;
                attacks |= 1u64 << current_sq_idx;
                if (blockers & (1u64 << current_sq_idx)) != 0 { // Hit a blocker
                    break;
                }
            } else { // Off board
                break;
            }
        }
    }
    attacks
}


pub fn generate_magic(sq:u8, is_rook:bool, num_index_bits:u8, offset: usize) -> Option<MagicEntry>{
    let blockers_subset = if is_rook {
        get_blocker_subsets(generate_rook_blocker_mask(sq))
    } else {
        get_blocker_subsets(generate_bishop_blocker_mask(sq))
    };

    let blocker_mask = if is_rook {
        generate_rook_blocker_mask(sq)
    } else {
        generate_bishop_blocker_mask(sq)
    };

    let mut attacks = Vec::<Bitboard>::new();
    for &blockers in &blockers_subset{
        if is_rook{
            attacks.push(calculate_rook_attacks(sq as usize, blockers));
        }else{
            attacks.push(calculate_bishop_attacks(sq as usize, blockers));
        }
    }

    let mut rng = rand::rng();
    let max_len = 1u64<<(num_index_bits);
    let mut best_len = max_len as usize;
    let mut magic_entry: MagicEntry = MagicEntry {mask: blocker_mask,magic: 0, shift: 0, offset: offset, size: 0};
    for _attempt in 0..1_000_000_000{
        let magic_candidate: u64 = rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>(); // "Sparse" random numbers often work better
        let mut used_indices = vec![0u64; 1 << num_index_bits];
        let mut occupied = vec![false; 1 << num_index_bits];
        let mut possible = true;

        for n in 0..blockers_subset.len(){
            let blocker = blockers_subset[n];
            let attack = attacks[n];
            let index = ((blocker * magic_candidate).0 >> 64-num_index_bits) as usize;
            if occupied[index]{
                possible = false;
                break;
            }
            occupied[index] = true;
            used_indices[index] = attack.0;
        }
        if possible{
            let len = occupied.iter().rposition(|&x| x != false).unwrap_or(0) + 1;
            if len < best_len{
                println!("Size ({}): {} of {}, optimal: {}, attempt: {}", sq, len, max_len, attacks.len(), _attempt);
                magic_entry = MagicEntry {
                    mask: blocker_mask,
                    magic: magic_candidate,
                    shift: 64 - num_index_bits,
                    offset: offset,
                    size: len,
                };
                best_len = len;
                if len == attacks.len(){
                    break;
                }
            }
        } 
    }  
    if best_len < max_len as usize{
        Some(magic_entry)
    }else{
        println!("No magic found for square {}, {}", sq, attacks.len());
        None
    }
}

pub fn generate_type_magics(index_bits: u8, is_rook: bool, offset: usize) -> Vec<MagicEntry>{
    let mut magics = Vec::<MagicEntry>::new();
    for sq in 0..64u8{
        if is_rook{
            if let Some(magic) = generate_magic(sq, true, index_bits, offset){
                magics.push(magic);
            }else{
                panic!("No magic found for square {}", sq);
            }
        } else{
            if let Some(magic) = generate_magic(sq, false, index_bits, offset){
                magics.push(magic);
             }else{
            panic!("No magic found for square {}", sq);
        }
    }}
    println!("Found {} magics", magics.len());
    magics
}



pub fn generate_type_attack_tables(magics: &Vec<MagicEntry>, is_rook: bool) -> Vec<Vec<Bitboard>>{
    println!("magics len {}", magics.len());
    let mut attack_tables = Vec::<Vec<Bitboard>>::new();
        if is_rook{
            for i in 0..64usize{
                let magic = magics[i];
                let blockers_subset = get_blocker_subsets(magic.mask);
                let mut table = vec![Bitboard::new(0); magic.size];
                for &blockers in &blockers_subset{
                    let attacks = calculate_rook_attacks(i, blockers);
                    let index = (blockers * magic.magic).0 >> magic.shift;
                    table[index as usize] = attacks;
                }
                attack_tables.push(table);
            }
        }else{
            for i in 0..64usize{
                let magic = magics[i];
                let blockers_subset = get_blocker_subsets(magic.mask);
                let mut table = vec![Bitboard::new(0); magic.size];
                for &blockers in &blockers_subset{
                    let attacks = calculate_bishop_attacks(i, blockers);
                    let index = (blockers * magic.magic).0 >> magic.shift;
                    table[index as usize] = attacks;
                }
                if let Some(last_nonzero_index) = table.iter().rposition(|&x| x != 0) {
                    let new_length = last_nonzero_index + 1;
                    table.truncate(new_length);
                } 
                attack_tables.push(table);
            }
        }
    attack_tables
}

pub fn reformat_magics_attack_tables(mut magics: Vec<MagicEntry>, attack_tables: Vec<Vec<Bitboard>>)
 -> (Vec<MagicEntry>, Vec<Bitboard>) {
    let mut offset = 0;
    for i in 0..magics.len(){
        magics[i].offset = offset;
        magics[i].size = attack_tables[i].len();
        offset += attack_tables[i].len();
    }
    let table: Vec<Bitboard> = attack_tables.into_iter().flatten().collect();
    (magics, table)
}


pub fn generate_magics_attack_tables(is_rook: bool) -> (Vec<MagicEntry>, Vec<Bitboard>){
    if is_rook{
        let rook_magics = generate_type_magics(13, true, 0);
        let rook_attack_table = generate_type_attack_tables(&rook_magics, true);
        return reformat_magics_attack_tables(rook_magics, rook_attack_table)
    }else{
        let bishop_magics = generate_type_magics(12, false, 0);
        let bishop_attack_table = generate_type_attack_tables(&bishop_magics, false);
        reformat_magics_attack_tables(bishop_magics, bishop_attack_table)
    }
}



/// ######################################################
/// ################### TESTING ##########################
/// ######################################################



#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_number_blockers_rook() {
        let mask_rook = generate_rook_blocker_mask(0);
        let blockers = get_blocker_subsets(mask_rook);
        assert_eq!(blockers.len(), 4096);

        let mask_rook = generate_rook_blocker_mask(12);
        let blockers = get_blocker_subsets(mask_rook);
        assert_eq!(blockers.len(), 1024);

        let mask_rook = generate_rook_blocker_mask(22);
        let blockers = get_blocker_subsets(mask_rook);
        assert_eq!(blockers.len(), 1024);

        let mask_rook = generate_rook_blocker_mask(8);
        let blockers = get_blocker_subsets(mask_rook);
        assert_eq!(blockers.len(), 2048);

        let mask_rook = generate_rook_blocker_mask(63);
        let blockers = get_blocker_subsets(mask_rook);
        assert_eq!(blockers.len(), 4096);
    }

    #[test]
    fn test_number_blockers_bishop() {
        let mask_bishop = generate_bishop_blocker_mask(0);
        let blockers = get_blocker_subsets(mask_bishop);
        assert_eq!(blockers.len(), 64);

        let mask_bishop = generate_bishop_blocker_mask(1);
        let blockers = get_blocker_subsets(mask_bishop);
        assert_eq!(blockers.len(), 32);

        let mask_bishop = generate_bishop_blocker_mask(36);
        let blockers = get_blocker_subsets(mask_bishop);
        assert_eq!(blockers.len(), 512);

    }
}

