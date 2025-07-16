use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn slice_file(file_path: &str, output_file: &str) -> io::Result<()> {
    // Open the input file
    let mut file = File::open(file_path)?;

    // Create a buffer to store bytes (16 bytes here, but adjust if needed)
    let mut buffer = [0u8; 16];

    // Open the output file to write the first 8 bytes
    let mut output = File::create(output_file)?;

    // Variable to track if we have written the first 8 bytes
    let mut bytes_written = 0;

    loop {
        // Read a chunk of bytes
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break; // EOF reached
        }

        // Write the first 8 bytes if not written yet
        if bytes_written < 8 {
            let to_write = &buffer[..8.min(bytes_read)];
            output.write_all(to_write)?;

            // Update the number of bytes we've written
            bytes_written += to_write.len();
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1]; // Input file path
    let output_file = &args[2]; // Output file path

    // Call slice_file with the provided arguments
    slice_file(file_path, output_file)?;

    Ok(())
}
