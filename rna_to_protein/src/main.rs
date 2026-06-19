use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_rna_codon_table() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();

    table.insert("UUU", "F");
    table.insert("UUC", "F");
    table.insert("UUA", "L");
    table.insert("UUG", "L");
    table.insert("UCU", "S");
    table.insert("UCC", "S");
    table.insert("UCA", "S");
    table.insert("UCG", "S");
    table.insert("UAU", "Y");
    table.insert("UAC", "Y");
    table.insert("UAA", "");
    table.insert("UAG", "");
    table.insert("UGU", "C");
    table.insert("UGC", "C");
    table.insert("UGA", "");
    table.insert("UGG", "W");

    table.insert("CUU", "L");
    table.insert("CUC", "L");
    table.insert("CUA", "L");
    table.insert("CUG", "L");
    table.insert("CCU", "P");
    table.insert("CCC", "P");
    table.insert("CCA", "P");
    table.insert("CCG", "P");
    table.insert("CAU", "H");
    table.insert("CAC", "H");
    table.insert("CAA", "Q");
    table.insert("CAG", "Q");
    table.insert("CGU", "R");
    table.insert("CGC", "R");
    table.insert("CGA", "R");
    table.insert("CGG", "R");

    table.insert("AUU", "I");
    table.insert("AUC", "I");
    table.insert("AUA", "I");
    table.insert("AUG", "M"); // Start
    table.insert("ACU", "T");
    table.insert("ACC", "T");
    table.insert("ACA", "T");
    table.insert("ACG", "T");
    table.insert("AAU", "N");
    table.insert("AAC", "N");
    table.insert("AAA", "K");
    table.insert("AAG", "K");
    table.insert("AGU", "S");
    table.insert("AGC", "S");
    table.insert("AGA", "R");
    table.insert("AGG", "R");

    table.insert("GUU", "V");
    table.insert("GUC", "V");
    table.insert("GUA", "V");
    table.insert("GUG", "V");
    table.insert("GCU", "A");
    table.insert("GCC", "A");
    table.insert("GCA", "A");
    table.insert("GCG", "A");
    table.insert("GAU", "D");
    table.insert("GAC", "D");
    table.insert("GAA", "E");
    table.insert("GAG", "E");
    table.insert("GGU", "G");
    table.insert("GGC", "G");
    table.insert("GGA", "G");
    table.insert("GGG", "G");

    table
}

fn main() -> Result<(), Box<dyn Error>> {
    let rna_table = get_rna_codon_table();
    let input_file = File::open("rna.txt")?;
    let mut lines = BufReader::new(input_file).lines();
    // let output_file = File::create("output.txt")?;
    // let writer = BufWriter::new(output_file);

    let rna: String = lines.next().ok_or("some")??;
    let mut protein = String::new();
    if rna.len() % 3 != 0 {
        return Err("Invalid RNA".into());
    }
    let mut l = 0;
    let mut r = 2;
    while r < rna.len() {
        let part = &rna[l..=r];
        println!("{part}");
        let letter = rna_table[part];
        protein += letter;
        l += 3;
        r += 3;
    }
    println!("{protein}");

    Ok(())
}
