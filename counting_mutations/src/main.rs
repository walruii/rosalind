use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line1 = lines.next().ok_or("File has less than 1 line")??;
    let line2 = lines.next().ok_or("File has less than 2 lines")??;

    let mut count = 0;

    for (ch1, ch2) in line1.chars().zip(line2.chars()) {
        if ch1 != ch2 {
            count += 1;
        }
    }
    println!("{count}");

    Ok(())
}
