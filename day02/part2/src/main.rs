use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut hor: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in lines {
        // let data: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();
        let data: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();
        let direction = data[0];
        let num: u32 = data[1].parse().unwrap();
        // println!("{:?}", data);

        match direction {
            "forward" => {hor += num as i32;
                          depth += aim * num as i32},
            "down" => aim += num as i32,
            "up" => aim -= num as i32,
            _ => panic!(),
        }
    }

    println!("Resut: {}", hor * depth);

    Ok(())
}

