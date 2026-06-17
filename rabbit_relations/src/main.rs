use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n = 34;
    let k = 3;

    let mut mature_rp: u128 = 0;
    let mut newborn_rp: u128 = 1;
    let mut month = 1;

    loop {
        month += 1;
        let tmp_matrure_rp = mature_rp;
        mature_rp += newborn_rp;

        newborn_rp = tmp_matrure_rp * k;
        if month == n {
            break;
        }
    }
    println!("{}", mature_rp + newborn_rp);

    Ok(())
}
