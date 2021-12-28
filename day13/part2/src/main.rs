use std::fs::File;
use std::io::{self, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut folds: Vec<(char, usize)> = Vec::new();
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }

        if line.as_ref().unwrap().starts_with("fold along") {
            let tmp = line.as_ref()
                .unwrap()
                .split_whitespace()
                .nth(2)
                .unwrap()
                .to_owned()
                .split('=')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            // println!("{:?}", tmp);
            folds.push((tmp[0].chars().nth(0).unwrap(), tmp[1].parse().unwrap()));
        } else {
            data.push(line
                      .as_ref()
                      .unwrap()
                      .split(',')
                      .map(|x| x.parse().unwrap())
                      .collect()
                     );
        }

    }

    // println!("{:?}", folds);
    // println!("{:#?}", data);

    let mut max = data.iter()
        .fold(vec![0, 0], |mut v_max, v| {
            if v[0] > v_max[0] {
                v_max[0] = v[0];
            }
            if v[1] > v_max[1] {
                v_max[1] = v[1];
            }
            // println!("{:?}", v);
            v_max
        });

    max[0] += 1;
    max[1] += 1;
    // println!("{:#?}", max);

    let mut grid = vec![vec![0;max[0]]; max[1]];

    for d in data {
        grid[d[1]][d[0]] = 1;
    }


    for f in folds {
        if f.0 == 'x' {
            for i in 0..f.1 {
                if (f.1 as i32) - 1 - (i as i32) < 0 || f.1 + 1 + i > max[0] {
                    break;
                }
                // println!("{} - {}", f.1 - 1 - i,f.1 + 1 + i);
                for j in 0..max[1] {
                    grid[j][f.1 - 1 - i] |= grid[j][f.1 + 1 + i]
                }
            }
            max[0] = f.1;
        } else {
            for i in 0..f.1 {
                if (f.1 as i32) - 1 - (i as i32) < 0 || f.1 + 1 + i > max[1] {
                    break;
                }
                // println!("{} - {}", f.1 - 1 - i,f.1 + 1 + i);
                for j in 0..max[0] {
                    grid[f.1 - 1 - i][j] |= grid[f.1 + 1 + i][j]
                }
            }
            max[1] = f.1;
        }
    }

    // println!("{:?}", grid);

    for i in 0..max[1] {
        for j in 0..max[0] {
            if j%5 == 0 {
                print!(" ");
            }

            // print!("{}", grid[i][j]);
            if grid[i][j] == 1 {
                print!("#");
            } else {
                print!(" ");
            }

        }
        println!("");
    }

    Ok(())
}

