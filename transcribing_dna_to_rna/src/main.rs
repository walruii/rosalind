use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

fn main() -> io::Result<()> {
    let input_file = File::open("dna.txt")?;
    let mut reader = BufReader::with_capacity(65536, input_file);
    let mut buffer = [0; 65536];

    let output_file = File::create("output_file.txt")?;
    let mut writer = BufWriter::with_capacity(65536, output_file);

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        for byte in &mut buffer[..n] {
            if *byte == b'T' {
                *byte = b'U'
            }
        }
        writer.write_all(&buffer[..n])?;
    }
    writer.flush()?;
    Ok(())
}
