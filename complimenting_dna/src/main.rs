use std::{
    fs::File,
    io::{self, BufWriter, Read, Seek, SeekFrom, Write},
};

fn main() -> io::Result<()> {
    let mut reader = File::open("dna.txt")?;
    let file_len = reader.metadata()?.len();
    let mut cursor_pos = file_len;

    let mut buffer = [0; 65536];

    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::with_capacity(65536, output_file);

    while cursor_pos > 0 {
        let chuck_size = if cursor_pos >= 65536 {
            65536
        } else {
            cursor_pos as usize
        };
        cursor_pos -= chuck_size as u64;
        reader.seek(SeekFrom::Start(cursor_pos))?;
        reader.read_exact(&mut buffer[..chuck_size])?;

        for byte in &mut buffer[..chuck_size] {
            match byte {
                b'A' => *byte = b'T',
                b'C' => *byte = b'G',
                b'T' => *byte = b'A',
                b'G' => *byte = b'C',
                _ => {}
            }
        }
        buffer[..chuck_size].reverse();

        writer.write_all(&buffer[..chuck_size])?;
    }
    writer.flush()?;

    Ok(())
}
