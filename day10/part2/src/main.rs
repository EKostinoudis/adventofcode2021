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

    let mut scores = Vec::<u64>::new();

    'outer: for line in data.iter() {
        let mut temp = Vec::<char>::new();

        for &c in line.iter() {
            if c == '{' || c == '[' || c == '(' || c == '<' {
                temp.push(c);
                continue;
            } 

            let p = temp.pop();
            match p {
                Some(x) => {
                    if c == ')' {
                        if x != '(' {
                            continue 'outer;
                        }
                    } else if c == ']' {
                        if x != '[' {
                            continue 'outer;
                        }
                    } else if c == '}' {
                        if x != '{' {
                            continue 'outer;
                        }
                    } else if c == '>' {
                        if x != '<' {
                            continue 'outer;
                        }
                    }
                },
                None => continue
            }
        }

        if temp.len() != 0 {
            let mut score: u64 = 0;
            for &c in temp.iter().rev() {
                score *= 5;
                if c == '(' {
                    score += 1;
                } else if c == '[' {
                    score += 2;
                } else if c == '{' {
                    score += 3;
                } else if c == '<' {
                    score += 4;
                }
            }

            scores.push(score);
        }

    }

    scores.sort();
    // println!("{:#?}", scores);
    println!("Result: {}", scores[scores.len() / 2]);

    Ok(())
}

