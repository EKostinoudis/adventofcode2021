use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let mut prev: u32 = lines.next().unwrap()?.parse().unwrap();

    let mut current: u32;
    let mut result: u32 = 0;

    for line in lines {
        current = line?.parse().unwrap();
        if current > prev {
            result += 1;
        }
        prev = current;
    }

    println!("Result: {}", result);

    Ok(())
}

