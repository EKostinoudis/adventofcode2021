use std::fs::File;
use std::io::{self, BufRead};
// use std::cmp;


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data_: Vec<Vec<i64>> = lines
        .map(|line| line
             .unwrap()
             .chars()
             .map(|x| x.to_digit(10).unwrap() as i64)
             .collect()
             )
        .collect();
    
    let mut data: Vec<Vec<i64>> = vec![vec![0; data_[0].len()*5]; data_.len()*5];
    for i in 0..5 {
        for j in 0..data_.len() {
            for k in 0..data_[j].len() {
                data[j + i*data_.len()][k] = data_[j][k] + i as i64;
                if data[j + i*data_.len()][k] > 9 {
                    data[j + i*data_.len()][k] -= 9;
                }
            }
        }
    }

    for i in 1..5 {
        for j in 0..data.len() {
            for k in 0..data_[0].len() {
                data[j][k + i*data_[0].len()] = data[j][k] + i as i64;
                if data[j][k + i*data_.len()] > 9 {
                    data[j][k + i*data_.len()] -= 9;
                }
            }
        }
    }

    // (risk, visited)
    let mut risk: Vec<Vec<(i64, bool)>> = vec![vec![(std::i64::MAX, false); data[0].len()]; data.len()];

    // risk[0][0] = data[0][0];
    risk[0][0].0 = 0;

    while risk.iter().flatten().any(|x: &(i64, bool)| x.1 == false) {
        // find min dist of the not visited
        let mut min: i64 = std::i64::MAX;
        let mut min_idx: (usize, usize) = (0, 0);
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if risk[i][j].1 == false && risk[i][j].0 < min {
                    min = risk[i][j].0;
                    min_idx.0 = i;
                    min_idx.1 = j;
                }
            }
        }


        let i = min_idx.0;
        let j = min_idx.1;
        risk[i][j].1 = true;

        // top cell
        if i > 0 {
            if risk[i-1][j].1 == false {
                let new_dist = risk[i][j].0 + data[i-1][j];
                if new_dist < risk[i-1][j].0 {
                    risk[i-1][j].0 = new_dist;
                }
            }
        }
        // bottom cell
        if i < data.len()-1 {
            if risk[i+1][j].1 == false {
                let new_dist = risk[i][j].0 + data[i+1][j];
                if new_dist < risk[i+1][j].0 {
                    risk[i+1][j].0 = new_dist;
                }
            }
        }
        // left cell
        if j > 0 {
            if risk[i][j-1].1 == false {
                let new_dist = risk[i][j].0 + data[i][j-1];
                if new_dist < risk[i][j-1].0 {
                    risk[i][j-1].0 = new_dist;
                }
            }
        }
        // right cell
        if j < data[0].len()-1 {
            if risk[i][j+1].1 == false {
                let new_dist = risk[i][j].0 + data[i][j+1];
                if new_dist < risk[i][j+1].0 {
                    risk[i][j+1].0 = new_dist;
                }
            }
        }
    }

    println!("Result: {}", risk[data.len()-1][data[0].len()-1].0);

    Ok(())
}

