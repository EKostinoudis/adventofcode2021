use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    // let mut prev: u32 = lines.next().unwrap()?.parse().unwrap();

    let bits = lines.by_ref().next().unwrap()?.len();

    // reset iterator
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut num: u32;
    let mut arr = vec![0; bits];
    for line in lines {
        num = u32::from_str_radix(&line?, 2).unwrap();
        for i in 0..bits {
            if num & (1 << i) != 0 {
                arr[i] += 1;
            } else {
                arr[i] -= 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, val) in arr.iter().enumerate() {
        // println!("i, val: {}, {}", i, val);
        if *val >= 0 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    println!("Result: {}", gamma * epsilon);

    Ok(())
}

