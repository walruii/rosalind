use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     let n = 34;
//     let k = 3;

//     let mut mature_rp: u128 = 0;
//     let mut newborn_rp: u128 = 1;
//     let mut month = 1;

//     loop {
//         month += 1;
//         let tmp_matrure_rp = mature_rp;
//         mature_rp += newborn_rp;

//         newborn_rp = tmp_matrure_rp * k;
//         if month == n {
//             break;
//         }
//     }
//     println!("{}", mature_rp + newborn_rp);

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let n = 34;
    let k = 3;

    if n == 1 || n == 2 {
        println!("1");
        return Ok(());
    }

    let mut prev2: u128 = 1;
    let mut prev1: u128 = 1;

    for _ in 3..=n {
        let curr = prev1 + k * prev2;
        prev2 = prev1;
        prev1 = curr;
    }

    println!("{prev1}",);
    Ok(())
}
