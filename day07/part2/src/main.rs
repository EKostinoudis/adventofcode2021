use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    let mut data = Vec::<i32>::new();

    data.extend(lines.next().unwrap()?.split(',').map(|x| x.parse::<i32>().unwrap()));

    let min = *data.iter().min().unwrap() as u32;
    let max = *data.iter().max().unwrap() as u32;
    // println!("Range: {}:{}", min, max);

    let fuel: Vec<i32> = (min..=max)
        .map(|pos: u32| data.iter()
             .fold(0, |acc, x| {
                 let abs = (x - (pos as i32)).abs();
                 acc + ((abs * (abs + 1)) / 2)
             }))
        .collect();
    // println!("Fuel: {:?}", fuel);

    // println!("Result: {}", fuel.iter().enumerate().map(|(x, y)| (y, x)).min().unwrap().1 + (min as usize));
    println!("Result: {}", fuel.iter().min().unwrap());

    Ok(())
}

