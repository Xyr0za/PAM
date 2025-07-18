use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

#[derive(Debug)]
struct Row {
    key: u64,
    value: u64,
}

const ITERS: usize = 1_000_000_000;

/// Lexicographically compare two u64 keys as DNA sequences (2 bits per base).
/// Returns:
/// -1 if a < b
///  0 if a == b
///  1 if a > b
fn lex_compare_dna_bits(a: u64, b: u64) -> i8 {
    // 64 bits, 2 bits per base => 32 bases total
    // compare from MSB to LSB, 2 bits at a time
    for i in (0..32).rev() {
        let shift = i * 2;
        let a_chunk = (a >> shift) & 0b11;
        let b_chunk = (b >> shift) & 0b11;
        if a_chunk < b_chunk {
            return -1;
        } else if a_chunk > b_chunk {
            return 1;
        }
    }
    0
}

/// Binary search for `target` in a sorted slice of Rows by DNA lex order key.
/// Returns Some(index) if found, None otherwise.
// fn binary_search_dna(rows: &[Row], target: u64) -> Option<usize> {
//     let mut low = 0;
//     let mut high = rows.len();
//
//     while low < high {
//         let mid = (low + high) / 2;
//         match lex_compare_dna_bits(rows[mid].key, target) {
//             0 => return Some(mid),
//             cmp if cmp < 0 => low = mid + 1,
//             _ => high = mid,
//         }
//     }
//     None
// }


fn binary_search_dna(rows: &[Row], target: u64) -> Option<usize> {
    let mut low = 0;
    let mut high = rows.len();

    while low < high {
        let mid = (low + high) / 2;
        match rows[mid].key.cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = BufReader::with_capacity(32 * 1024 * 1024, File::open(&args[1])?);

    let search_term: u64 = args[2].parse().unwrap();

    let mut rows = Vec::with_capacity(ITERS);
    let mut buf = [0u8; 16];

    let start = Instant::now();

    let mut i = 0;
    while file.read_exact(&mut buf).is_ok() {
        if i >= ITERS {
            break;
        }

        let key = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let value = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        rows.push(Row { key, value });
        i += 1;
    }


    // println!("First few entries:");
    // for row in rows.iter() {
    //     let dna = row.key;
    //     println!(
    //         "{:064b} {} {:064b}",
    //         dna,
    //         dna,
    //         row.value
    //     );
    // }

    rows.sort_by(|a, b| {
        let cmp = lex_compare_dna_bits(a.key, b.key);
        cmp.cmp(&0) // converts i8 to std::cmp::Ordering
    });

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    // println!("First few 'sorted' entries:");
    // for row in rows.iter() {
    //     let dna = row.key;
    //     println!(
    //         "{:064b} {} {:064b}",
    //         dna,
    //         dna,
    //         row.value
    //     );
    // }

    match binary_search_dna(&rows, search_term) {
        Some(idx) => {
            println!("Found key at index {}: {:?}", idx, rows[idx]);
        }
        None => {
            println!("Key not found");
        }
    }

    Ok(())
}