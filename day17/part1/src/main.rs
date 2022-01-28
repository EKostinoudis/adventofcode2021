use std::fs::File;
use std::io::{self, BufRead};
use itertools::iproduct;

fn get_max_vel(mut x_v: i32, mut  y_v: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut y_max = 0;
    loop {
        x += x_v;
        y += y_v;
        if x_v > 0 {
            x_v -= 1;
        } else if x < 0 {
            x_v += 1;
        }
        y_v -= 1;

        if y > y_max {
            y_max = y;
        }

        match (xmin <= x && x <= xmax, ymin <= y && y <= ymax) {
            (true, true) => return Some(y_max),
            (_, false) => {
                if y < ymin && y_v <= 0 {
                    return None;
                }
            },
            (false, _) => {
                if x_v == 0 {
                    return None;
                }
            },
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.nth(0).unwrap()?;

    let (left, right) = line[15..]
        .split_once(", y=")
        .unwrap();
    let (xmin, xmax) = left
        .split_once("..")
        .unwrap();
    let xmin: i32 = xmin.parse().unwrap();
    let xmax: i32 = xmax.parse().unwrap();
    let (ymin, ymax) = right
        .split_once("..")
        .unwrap();
    let ymin: i32 = ymin.parse().unwrap();
    let ymax: i32 = ymax.parse().unwrap();

    // println!("{}, {} | {}, {}", xmin, xmax, ymin, ymax);

    let ys: Vec<i32> = iproduct!(0..500, 0..500)
        .filter_map(|(x_v, y_v)| get_max_vel(x_v, y_v, xmin, xmax, ymin, ymax))
        .collect();


    println!("Result: {}", ys.iter().max().unwrap());

    Ok(())
}

