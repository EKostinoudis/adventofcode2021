use std::fs::File;
use std::io::{self, BufRead};

fn check(data: &mut Vec<Vec<u32>>, p: (usize, usize)) -> u32 {
    if data[p.0][p.1] == 9 {
        return 0;
    }

    // update data
    data[p.0][p.1] = 9;

    let mut ret = 1;
    if p.0 > 0 && data[p.0 - 1][p.1] != 9 {
        ret += check(data, (p.0 - 1, p.1));
    }
    if p.0 < (data.len() - 1) && data[p.0 + 1][p.1] != 9 {
        ret += check(data, (p.0 + 1, p.1));
    }
    if p.1 > 0 && data[p.0][p.1 - 1] != 9 {
        ret += check(data, (p.0, p.1 - 1));
    }
    if p.1 < (data[p.0].len() - 1) && data[p.0][p.1 + 1] != 9 {
        ret += check(data, (p.0, p.1 + 1));
    }

    ret
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<u32>> = lines.map(
        |line| line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10)
                .unwrap()
                )
            .collect::<Vec<u32>>()
        ).collect();

    // println!("{:#?}", data);

    // iterate over the data and calculate the basins
    // create a recursive function in which first called with
    let mut basin = Vec::<u32>::new();
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == 9 {
                continue;
            }

            basin.push(check(&mut data, (x, y)));
        }
    }
    basin.sort();
    // println!("basins: {:?}", basin);

    println!("Result: {}", basin.iter().rev().take(3).fold(1, |acc, x| acc * x));

    Ok(())
}

