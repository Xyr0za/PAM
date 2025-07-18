use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use rayon::prelude::*;

#[derive(Debug)]
struct Row {
    key: u64,
    value: u64,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let mut file = BufReader::with_capacity(32 * 1024 * 1024, File::open(input_path)?);

    let mut rows: Vec<(Row, usize)> = Vec::new();
    let mut buf = [0u8; 16];
    let mut index = 0;

    while file.read_exact(&mut buf).is_ok() {
        let key = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let value = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        rows.push((Row { key, value }, index));
        index += 1;
    }

    // Parallel sort by the Row's key
    rows.par_sort_by_key(|(row, _)| row.key);

    let mut writer = BufWriter::with_capacity(32 * 1024 * 1024, File::create(output_path)?);

    // Write key and original index as two consecutive u64s
    for (row, idx) in rows.iter() {
        writer.write_all(&row.key.to_le_bytes())?;
        writer.write_all(&(*idx as u64).to_le_bytes())?;
    }
    writer.flush()?;

    Ok(())
}
