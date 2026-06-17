use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("test.fasta")?;
    let reader = BufReader::new(file);

    let mut current_code: String = String::from("");

    let mut max_gc: (String, f64) = ("".to_string(), 0.0);

    let mut gc_count: u32 = 0;
    let mut len: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with(">") {
            if current_code != "" {
                let percentage: f64 = (gc_count as f64 / len as f64) * 100.0;
                if max_gc.1 < percentage {
                    max_gc = (current_code, percentage);
                }
            }
            current_code = line[1..].to_string();
            len = 0;
            gc_count = 0;
        } else {
            if current_code == "" {
                return Err("Invalid file format".into());
            }
            for byte in line.into_bytes() {
                if byte == b'G' || byte == b'C' {
                    gc_count += 1;
                    len += 1;
                } else if byte == b'A' || byte == b'T' {
                    len += 1;
                }
            }
        }
    }
    if current_code != "" {
        let percentage: f64 = (gc_count as f64 / len as f64) * 100.0;
        if max_gc.1 < percentage {
            max_gc = (current_code, percentage);
        }
    }
    println!("{}\n{:.6}", max_gc.0, max_gc.1);
    Ok(())
}
