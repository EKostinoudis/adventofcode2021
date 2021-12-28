use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    // let mut prev: u32 = lines.next().unwrap()?.parse().unwrap();

    let bits = lines.by_ref().next().unwrap()?.len();

    /*
    // reset iterator
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut num: u32;
    let mut arr = vec![0; bits];
    for line in lines {
        num = u32::from_str_radix(&line?, 2).unwrap();
        for i in 0..bits {
            if num & (1 << i) != 0 {
                arr[i] += 1;
            } else {
                arr[i] -= 1;
            }
        }
    }
    */

    let file = File::open("input.txt")?;
    let mut data: Vec<String> = io::BufReader::new(file).lines().filter_map(|x| x.ok()).collect();

    for pos in 0..bits {
        if data.len() == 1 {
            break;
        }
        let mut arr = vec![0; bits];
        let mut num: u32;
        for line in &data {
            num = u32::from_str_radix(&line, 2).unwrap();
            for i in 0..bits {
                if num & (1 << i) != 0 {
                    arr[i] += 1;
                } else {
                    arr[i] -= 1;
                }
            }
        }

        let bit = arr[bits - pos - 1];

        data.retain(|x| {
            let ret = {
                let mut a = true;
                if bit < 0 {
                    if x.chars().nth(pos).unwrap() == '1' {
                        a = false;
                    }
                } else {
                    if x.chars().nth(pos).unwrap() == '0' {
                        a = false;
                    }
                }
                a
            };
            ret
        })
    }


    let file = File::open("input.txt")?;
    let mut data2: Vec<String> = io::BufReader::new(file).lines().filter_map(|x| x.ok()).collect();

    for pos in 0..bits {
        if data2.len() == 1 {
            break;
        }
        let mut arr = vec![0; bits];
        let mut num: u32;
        for line in &data2 {
            num = u32::from_str_radix(&line, 2).unwrap();
            for i in 0..bits {
                if num & (1 << i) != 0 {
                    arr[i] += 1;
                } else {
                    arr[i] -= 1;
                }
            }
        }

        let bit = arr[bits - pos - 1];

        data2.retain(|x| {
            let ret = {
                let mut a = true;
                if bit >= 0 {
                    if x.chars().nth(pos).unwrap() == '1' {
                        a = false;
                    }
                } else {
                    if x.chars().nth(pos).unwrap() == '0' {
                        a = false;
                    }
                }
                a
            };
            ret
        })
    }

    let data = u32::from_str_radix(&data[0], 2).unwrap();
    let data2 = u32::from_str_radix(&data2[0], 2).unwrap();

    // println!("data: {}, {}", data, data2);
    println!("Result: {}", data * data2);

    Ok(())
}

