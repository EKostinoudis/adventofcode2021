use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    // lines.map(|line| println!("{:?}", line?.split(',').skip(1).split_whitespace()));
    let data: Vec<Vec<String>> = lines.map(
        |line| line
            .unwrap()
            .split('|')
            .skip(1).next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
        ).collect();
    // println!("{:#?}", data);

    // get elements with size 2, 3, 4, 7
    let result = data
        .iter()
        .flatten()
        .filter(|x| 
                (x.len() >= 2 && x.len() <= 4) || x.len() == 7)
        .count();

    println!("Result: {}", result);

    Ok(())
}

