use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

#[derive(Debug)]
struct Row {
    key: u64,
    value: u64,
}

// Convert a 64-bit integer into a 32-base DNA sequence string (A, C, G, T)
fn u64_to_acgt(num: u64) -> String {
    let mut result = String::with_capacity(32);
    for i in (0..32).rev() {
        let bits = (num >> (i * 2)) & 0b11;
        let base = match bits {
            0b00 => 'A',
            0b01 => 'C',
            0b10 => 'G',
            0b11 => 'T',
            _ => unreachable!(),
        };
        result.push(base);
    }
    result
}

const MAX_ITERS: usize = 1_000_000_000;

// Binary search for a target key in a sorted slice of Rows
fn binary_search_dna(rows: &[Row], target: u64) -> Option<usize> {
    println!("{}", MAX_ITERS);

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

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <search_term>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let search_term: u64 = args[2].parse().expect("Invalid search term");

    let file = File::open(file_path)?;
    let mut reader = BufReader::with_capacity(32 * 1024 * 1024, file);

    let mut rows = Vec::with_capacity(MAX_ITERS);
    let mut buf = [0u8; 16];

    let start = Instant::now();

    for _ in 0..MAX_ITERS {
        if reader.read_exact(&mut buf).is_err() {
            break;
        }
        let key = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let value = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        rows.push(Row { key, value });
    }

    println!("Time elapsed loading data: {:?}", start.elapsed());
    println!("Searching for: {}", u64_to_acgt(search_term));

    let match_start = Instant::now();

    match binary_search_dna(&rows, search_term) {
        Some(idx) => {
            println!("Found key at index {}: {:?}", idx, rows[idx]);
        }
        None => {
            println!("Key not found");
        }
    }

    println!("Time elapsed matching data: {:?}", match_start.elapsed());

    Ok(())
}
