use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let mut var1: u32 = lines.next().unwrap()?.parse().unwrap();
    let mut var2: u32 = lines.next().unwrap()?.parse().unwrap();
    let mut var3: u32 = lines.next().unwrap()?.parse().unwrap();
    let mut var4: u32;

    let mut prev_sum: u32 = var1 + var2 + var3;
    let mut current_sum: u32 = var2 + var3;
    let mut result: u32 = 0;

    for line in lines {
        var4 = line?.parse().unwrap();
        current_sum += var4;
        if current_sum > prev_sum {
            result += 1;
        }

        // update sums
        prev_sum += var4;
        prev_sum -= var1;
        current_sum -= var2;

        // update values
        var1 = var2;
        var2 = var3;
        var3 = var4;
    }

    println!("Result: {}", result);

    Ok(())
}

