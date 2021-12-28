use std::fs::File;
use std::io::{self, BufRead};

fn update_vec(data: &mut [u64]) {
    let first = data[0];
    for i in 0..8 {
        data[i] = data[i+1];
    }
    data[8] = first;
    data[6] += first;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    // let mut data = Vec::<u32>::new();
    let data = &mut [0u64; 9];

    for x in lines.next().unwrap()?.split(',').map(|x| x.parse::<usize>().unwrap()) {
        data[x] += 1;
    }

    // println!("data: {:?}", data);
    for _ in 0..256 {
        update_vec(data);
    }

    println!("Result: {}", data.iter().sum::<u64>());

    Ok(())
}

