use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Line {
    p1: [u32; 2],
    p2: [u32; 2],
}

fn add_line(grid: &mut Vec<Vec<u32>>, line: Line) {
    if line.p1[0] == line.p2[0] {
        for i in std::cmp::min(line.p1[1], line.p2[1])..=std::cmp::max(line.p1[1], line.p2[1]) {
            grid[line.p1[0] as usize][i as usize] += 1;
        }
    } else if line.p1[1] == line.p2[1] {
        for i in std::cmp::min(line.p1[0], line.p2[0])..=std::cmp::max(line.p1[0], line.p2[0]) {
            grid[i as usize][line.p1[1] as usize] += 1;
        }
    } else {
        if line.p1[0] < line.p2[0] && line.p1[1] < line.p2[1] {
            for i in 0..=(line.p2[0] - line.p1[0]) {
                grid[(line.p1[0] + i) as usize][(line.p1[1] + i) as usize] += 1;
            }
        } else if line.p1[0] < line.p2[0] && line.p1[1] > line.p2[1] {
            for i in 0..=(line.p2[0] - line.p1[0]) {
                grid[(line.p1[0] + i) as usize][(line.p1[1] - i) as usize] += 1;
            }
        } else if line.p1[0] > line.p2[0] && line.p1[1] < line.p2[1] {
            for i in 0..=(line.p2[1] - line.p1[1]) {
                grid[(line.p1[0] - i) as usize][(line.p1[1] + i) as usize] += 1;
            }
        } else {
            for i in 0..=(line.p1[0] - line.p2[0]) {
                grid[(line.p1[0] - i) as usize][(line.p1[1] - i) as usize] += 1;
            }
        } 
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut all_lines = Vec::<Line>::new();

    for line in lines {
        let points: Vec<u32> = line?.split("->")
            .map(|x| x.trim().split(','))
            .into_iter().flatten()
            .map(|x| x.parse().unwrap())
            .collect();

        all_lines.push( Line {
            p1: [points[0], points[1]],
            p2: [points[2], points[3]],
        })

    }

    let max_val: usize = all_lines.
        iter().
        fold(0, |max, x| std::cmp::max(
                std::cmp::max(
                    *x.p1.iter().max().unwrap(), 
                    *x.p2.iter().max().unwrap()), 
                max)) as usize;
    let max_val = max_val + 1;

    // println!("{:?}", all_lines);
    // println!("max: {}", max_val);
    let mut grid = vec![vec![0; max_val]; max_val];
    for line in all_lines {
        add_line(&mut grid, line);
    }
    // println!("grid: {:?}", grid);

    let result = grid.into_iter().flatten().filter(|&x| x > 1).count();
    println!("Result: {:?}", result);

    Ok(())
}

