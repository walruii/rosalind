use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let input_file = File::open("test_dna.txt")?;

    let mut buffer = [0; 65536];
    let mut reader = io::BufReader::with_capacity(buffer.len(), input_file);

    let output_file = File::create("dna_counts.txt")?;
    let mut writer = BufWriter::new(output_file);

    let mut lookup = [0u64; 256];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for &byte in &buffer[..n] {
            lookup[byte as usize] += 1;
        }
    }
    let output_string = format!(
        "{} {} {} {}",
        lookup[b'A' as usize], lookup[b'C' as usize], lookup[b'G' as usize], lookup[b'T' as usize]
    );

    writer.write_all(output_string.as_bytes())?;
    println!("{}", output_string);
    writer.flush()?;

    Ok(())
}
