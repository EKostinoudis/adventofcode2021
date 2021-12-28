use std::fs::File;
use std::io::{self, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut pol: Vec<char> = Vec::new();
    let mut template: Vec<(Vec<char>, char)> = Vec::new();
    for line in lines {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }

        if line.as_ref().unwrap().contains("->") {
            let tmp = line.as_ref()
                .unwrap()
                .split(" -> ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            // println!("{:?}", tmp);
            template.push((tmp[0].chars().collect(), 
                        tmp[1].parse().unwrap()));
        } else {
            pol = line.as_ref().unwrap().chars().collect();
        }
    }

    // println!("{:?}", pol);
    // println!("{:#?}", template);

    for _ in 0..10 {
        let mut new: Vec<(char, usize)> = Vec::new();
        for (i, arr) in pol.windows(2).enumerate() {
            let x = arr[0];
            let y = arr[1];
            // println!("{} {}", x, y);

            for t in &template {
                if t.0[0] == x && t.0[1] == y {
                    new.push((t.1, i+1));
                }
            }
        }

        for n in new.iter().rev() {
            pol.insert(n.1, n.0);
        }
    }
    pol.sort();
    let mut min = 0;
    let mut max = 0;
    let mut count = 0;
    let mut first = true;
    let mut prev = pol[0];
    for p in pol {
        if p != prev {
            if first {
                first = false;
                min = count;
                max = count;
            } else {
                if min > count {
                    min = count;
                }
                if max < count {
                    max = count;
                }
            }
            count = 1;
        } else {
            count += 1;
        }

        prev = p;
    }
    // println!("{:?}", pol);

    println!("Result: {}", max - min);

    Ok(())
}

