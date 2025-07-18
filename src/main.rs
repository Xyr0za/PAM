use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
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

// Binary search for a target key in a sorted slice of Rows
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

// Read list of u64 search terms from a file (one per line)
fn read_search_terms(path: &str) -> std::io::Result<Vec<u64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut terms = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        match line.trim().parse::<u64>() {
            Ok(num) => terms.push(num),
            Err(_) => {
                eprintln!("Warning: invalid u64 on line {}: '{}'", i + 1, line);
            }
        }
    }

    Ok(terms)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <binary_data_file> <search_terms_file>", args[0]);
        std::process::exit(1);
    }

    let binary_file_path = &args[1];
    let terms_file_path = &args[2];

    let file = File::open(binary_file_path)?;
    let mut reader = BufReader::new(file);

    let load_start = Instant::now();

    // Read the binary file into buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let row_size = 16;
    if buffer.len() % row_size != 0 {
        eprintln!("File size is not a multiple of 16 bytes; invalid format.");
        std::process::exit(1);
    }

    let num_rows = buffer.len() / row_size;
    let mut rows = Vec::with_capacity(num_rows);
    for i in 0..num_rows {
        let offset = i * row_size;
        let key = u64::from_le_bytes(buffer[offset..offset + 8].try_into().unwrap());
        let value = u64::from_le_bytes(buffer[offset + 8..offset + 16].try_into().unwrap());
        rows.push(Row { key, value });
    }

    println!("Loaded {} rows from '{}'", num_rows, binary_file_path);
    println!("Time elapsed loading data: {:?}", load_start.elapsed());

    let search_terms = read_search_terms(terms_file_path)?;
    println!("Searching {} terms from '{}'", search_terms.len(), terms_file_path);

    let search_start = Instant::now();

    for term in search_terms {
        let acgt = u64_to_acgt(term);
        match binary_search_dna(&rows, term) {
            Some(idx) => {
                println!("✓ FOUND: {} ({}): {:?}", acgt, term, rows[idx]);
            }
            None => {
                println!("✗ NOT FOUND: {} ({})", acgt, term);
            }
        }
    }

    println!("Time elapsed searching: {:?}", search_start.elapsed());

    Ok(())
}
