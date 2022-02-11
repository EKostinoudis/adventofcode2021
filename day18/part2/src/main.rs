use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

fn explode(data: &mut Vec<(i32, u32)>) {
    loop {
        let max_depth_idx = data.iter().enumerate().reduce(|max, item| {
            let depth = item.1.0;
            if depth > max.1.0 {
                item
            } else {
                max
            }
        }).unwrap();

        if max_depth_idx.1.0 < 5 { break; }
        let max_depth_idx = max_depth_idx.0; // idx of max depth

        let removed = data.remove(max_depth_idx);
        let (rd, rv) = data[max_depth_idx];
        if removed.0 != rd { 
            panic!("Not the correct pair. Depths: {}, {}", removed.0, rd); 
        }
        if max_depth_idx != 0 {
            data[max_depth_idx-1].1 += removed.1;
        } 
        if max_depth_idx != data.len() - 1 {
            data[max_depth_idx+1].1 += rv;
        }
        data[max_depth_idx] = (removed.0 - 1, 0);
    }
}

fn split(data: &mut Vec<(i32, u32)>) -> bool {
    // find a value > 9
    let idx = data.iter().enumerate().find(|x| x.1.1 > 9);

    match idx {
        None =>  return false,
        Some(idx) => {
            let idx = idx.0; // idx of max depth

            let (d, v) = data[idx];
            data[idx] = (d + 1, v / 2);
            data.insert(idx+1, (d + 1, v - v/2));
            return true
        },
    }
}

fn add(left: &Vec<(i32, u32)>, right: &Vec<(i32, u32)>) -> Vec<(i32, u32)> {
    left.into_iter().chain(right).map(|x| (x.0 + 1, x.1)).collect()
}

fn magnitude(data: &mut Vec<(i32, u32)>) -> u32 {
    while data.len() != 1 {
        let max_depth_idx = data.iter().enumerate().reduce(|max, item| {
            let depth = item.1.0;
            if depth > max.1.0 {
                item
            } else {
                max
            }
        }).unwrap();

        let max_depth_idx = max_depth_idx.0; // idx of max depth

        let removed = data.remove(max_depth_idx);
        let (rd, rv) = data[max_depth_idx];
    
        data[max_depth_idx] = (rd-1, removed.1 * 3 + rv * 2);
    }

    data[0].1
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<(i32, u32)>> = Vec::new();
    for (i, line) in lines.enumerate() {
        data.push(vec![]);
        let line = line.unwrap();
        let mut depth = 0;
        for cc in line.chars() {
            match cc {
                '[' => depth += 1,
                ']' => depth -= 1,
                c if c.is_ascii_digit() => {
                    let num = c.to_digit(10).unwrap() as u32;
                    data[i].push((depth, num));
                },
                _ => (),
            }
        }
    }

    let res = data.iter().permutations(2).map(|x| {
        let mut left = x[0].clone();
    
        left = add(&left, &x[1]);
        loop {
            explode(&mut left);
            let changed = split(&mut left);
            if !changed { break; }
        }
        magnitude(&mut left)
    }).max().unwrap();

    println!("Result: {}", res);

    Ok(())
}

