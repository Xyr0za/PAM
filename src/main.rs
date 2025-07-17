use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

#[derive(Debug)]
struct Row {
    key: u64,
    value: u64,
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <search_key>", args[0]);
        return Ok(());
    }

    let search_key: u64 = args[2].parse::<u64>().expect("Invalid search key");

    let mut file = BufReader::with_capacity(32 * 1024 * 1024, File::open(&args[1])?);

    let mut rows = Vec::with_capacity(14_000_000_000);
    let mut buf = [0u8; 16];

    while file.read_exact(&mut buf).is_ok() {
        let key = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let value = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        rows.push(Row { key, value });
    }

    let duration = start.elapsed();
    println!("Load: {:.2?}", duration);

    // Print first few entries
    println!("First few entries:");
    for (i, row) in rows.iter().take(5).enumerate() {
        println!("Row {}: key={}, value={}", i, row.key, row.value);
    }

    let search = Instant::now();

    // Search using binary search
    if let Some(row) = binary_search(&rows, search_key) {
        println!("Found: key={}, value={}", row.key, row.value);
    } else {
        println!("Key {} not found.", search_key);
    }

    let search_duration = search.elapsed();
    println!("Search: {:.2?}", search_duration);

    Ok(())
}

fn binary_search(rows: &[Row], target: u64) -> Option<&Row> {
    let mut low = 0;
    let mut high = rows.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if rows[mid].key == target {
            return Some(&rows[mid]);
        } else if rows[mid].key < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    None
}
