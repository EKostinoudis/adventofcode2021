use std::fs::File;
use std::io::{self, BufRead};

fn update_vec(data: &mut Vec<u32>) {
    let mut new_data_count = 0;
    for i in data.into_iter().rev() {
        if *i == 0 {
            *i = 6;
            new_data_count += 1;
        } else {
            *i -= 1;
        }
    }

    for _ in 0..new_data_count {
        data.push(8);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    let mut data = Vec::<u32>::new();

    data.extend(lines.next().unwrap()?.split(',').map(|x| x.parse::<u32>().unwrap()));

    // println!("data: {:?}", data);
    for _ in 0..80 {
        update_vec(&mut data);
    }

    println!("Result: {}", data.len());

    Ok(())
}

