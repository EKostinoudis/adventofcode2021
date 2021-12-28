use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data: Vec<Vec<char>> = lines.map(
        |line| line
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
        ).collect();

    // println!("{:#?}", data);

    let mut result: u64 = 0;
    let mut temp = Vec::<char>::new();


    for line in data.iter() {
        for &c in line.iter() {
            if c == '{' || c == '[' || c == '(' || c == '<' {
                temp.push(c);
            } else if c == ')' {
                let p = temp.pop();
                if p == None || p != Some('(') {
                    result += 3;
                }
            } else if c == ']' {
                let p = temp.pop();
                if p == None || p != Some('[') {
                    result += 57;
                }
            } else if c == '}' {
                let p = temp.pop();
                if p == None || p != Some('{') {
                    result += 1197;
                }
            } else if c == '>' {
                let p = temp.pop();
                if p == None || p != Some('<') {
                    result += 25137;
                }
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}

