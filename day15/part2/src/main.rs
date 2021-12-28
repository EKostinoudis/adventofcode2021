use std::fs::File;
use std::io::{self, BufRead};
// use std::cmp;


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data_: Vec<Vec<i32>> = lines
        .map(|line| line
             .unwrap()
             .chars()
             .map(|x| x.to_digit(10).unwrap() as i32)
             .collect()
             )
        .collect();
    
    let mut data: Vec<Vec<i32>> = vec![vec![0; data_[0].len()*5]; data_.len()*5];
    for i in 0..5 {
        for j in 0..data_.len() {
            for k in 0..data_[j].len() {
                data[j + i*data_.len()][k] = data_[j][k] + i as i32;
                if data[j + i*data_.len()][k] > 9 {
                    data[j + i*data_.len()][k] -= 9;
                }
            }
        }
    }

    for i in 1..5 {
        for j in 0..data.len() {
            for k in 0..data_[0].len() {
                data[j][k + i*data_[0].len()] = data[j][k] + i as i32;
                if data[j][k + i*data_.len()] > 9 {
                    data[j][k + i*data_.len()] -= 9;
                }
            }
        }
    }

    // println!("data {:?}", data);

    // TODO: prob need to implement Dijkstra's algorithm
    let mut risk: Vec<Vec<i32>> = vec![vec![-1; data[0].len()]; data.len()];

    // risk[0][0] = data[0][0];
    risk[0][0] = 0;

    // for i in 1..=cmp::max(data.len(), data[0].len()) {
    'outer: for i in 1.. {
        // walk diagonally
        for j in 0..=i {
            // each iter we check the [j][i-j] cell
            // check the adjacent cells and update the risk

            if j >= data.len() || i-j >= data[0].len(){
                continue;
            }

            // println!("{} {}", j, i-j);

            // left cell
            if i-j > 0 {
                if risk[j][i-j] == -1 {
                    risk[j][i-j] = risk[j][i-j-1] + data[j][i-j];
                } else if risk[j][i-j-1] + data[j][i-j] < risk[j][i-j] {
                    risk[j][i-j] = risk[j][i-j-1] + data[j][i-j];
                }
            }

            // top cell
            if j > 0 {
                if risk[j][i-j] == -1 {
                    risk[j][i-j] = risk[j-1][i-j] + data[j][i-j];
                } else if risk[j-1][i-j] + data[j][i-j] < risk[j][i-j] {
                    risk[j][i-j] = risk[j-1][i-j] + data[j][i-j];
                }
            }
            if j == data.len() - 1 && i-j == data[0].len() - 1 {
                break 'outer;
            }
        }
    }

    // println!("risk {:#?}", risk);

    println!("Result: {}", risk[data.len()-1][data[0].len()-1]);

    Ok(())
}

