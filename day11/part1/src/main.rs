use std::fs::File;
use std::io::{self, BufRead};

fn update_adj(data: &mut Vec<Vec<u32>>, x: usize, y: usize) {
    if x > 0 && y > 0 {
        data[x-1][y-1] += 1;
        if data[x-1][y-1] == 10 {
            update_adj(data, x-1, y-1);
        }
    }
    if x < data.len() - 1 && y < data[x].len() - 1 {
        data[x+1][y+1] += 1;
        if data[x+1][y+1] == 10 {
            update_adj(data, x+1, y+1);
        }
    }
    if x > 0 && y < data[x].len() - 1 {
        data[x-1][y+1] += 1;
        if data[x-1][y+1] == 10 {
            update_adj(data, x-1, y+1);
        }
    }
    if x < data.len() - 1 && y > 0 {
        data[x+1][y-1] += 1;
        if data[x+1][y-1] == 10 {
            update_adj(data, x+1, y-1);
        }
    }
    if x > 0 {
        data[x-1][y] += 1;
        if data[x-1][y] == 10 {
            update_adj(data, x-1, y);
        }
    }
    if y > 0 {
        data[x][y-1] += 1;
        if data[x][y-1] == 10 {
            update_adj(data, x, y-1);
        }
    }
    if x < data.len() - 1 {
        data[x+1][y] += 1;
        if data[x+1][y] == 10 {
            update_adj(data, x+1, y);
        }
    }
    if y < data[x].len() - 1 {
        data[x][y+1] += 1;
        if data[x][y+1] == 10 {
            update_adj(data, x, y+1);
        }
    }
}

fn update(data: &mut Vec<Vec<u32>>) -> u32 {
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            data[x][y] += 1;
        }
    }

    let mut to_update: Vec<(usize, usize)> = Vec::new();
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == 10 {
                to_update.push((x,y));
            }
        }
    }

    for (x, y) in to_update.iter() {
        update_adj(data, *x, *y);
    }

    let mut flashes = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] > 9 {
                data[x][y] = 0;
                flashes += 1;
            }
        }
    }

    flashes
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<u32>> = lines.map(
        |line| line
            .unwrap()
            .chars()
            .map(|x|  x
                .to_digit(10)
                .unwrap()
            )
            .collect::<Vec<u32>>()
        ).collect();

    // println!("{:#?}", data);

    let mut result = 0;
    for _ in 0..100 {
        result += update(&mut data);
        // println!("{:?}", data);
    }

    println!("Result: {}", result);

    Ok(())
}

