use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data: Vec<Vec<u32>> = lines.map(
        |line| line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10)
                .unwrap()
                )
            .collect::<Vec<u32>>()
        ).collect();

    // println!("{:#?}", data);


    let mut result: u32 = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            let v = data[x][y];
            if (!(x > 0) || data[x-1][y] > v) && 
                (!(x < (data.len() - 1)) || data[x+1][y] > v) && 
                (!(y > 0) || data[x][y-1] > v) &&
                (!(y < (data[x].len() - 1)) || data[x][y+1] > v) {
                result += v + 1;
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}

