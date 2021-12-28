use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn get_num(line: &mut Vec<Vec<String>>) -> u32 {
    let mut nums = HashMap::new();
    let mut one: String = "".to_string();
    let mut four: String = "".to_string();
    // size 2: 1
    // size 3: 7
    // size 4: 4
    // size 7: 8
    for val in &line[0] {
        if val.len() == 2 {
            nums.insert(val, 1);
            one = val.to_string();
        } else if val.len() == 3 {
            nums.insert(val, 7);
        } else if val.len() == 4 {
            nums.insert(val, 4);
            four = val.to_string();
        } else if val.len() == 7 {
            nums.insert(val, 8);
        }
    }

    // on size 6
    //   if not both from 1 (size 2) are present
    //     is 6
    //   else if all from 4 (size 4) are present
    //     is 9
    //   else
    //     is 0
    // on size 5
    //   if all from 1 (size 2) are present
    //     is 3
    //   else if 1 miss from 4 (size 4)
    //     is 5
    //   else
    //     is 2
    for val in &line[0] {
        if val.len() == 6 {
            if one.chars().map(|x| val.contains(x)).any(|x| !x) {
                nums.insert(val, 6);
            } else if four.chars().map(|x| val.contains(x)).all(|x| x) {
                nums.insert(val, 9);
            } else {
                nums.insert(val, 0);
            }
        } else if val.len() == 5 {
            if one.chars().map(|x| val.contains(x)).all(|x| x) {
                nums.insert(val, 3);
            } else if four.chars().filter(|x| val.contains(*x)).count() == 3 {
                nums.insert(val, 5);
            } else {
                nums.insert(val, 2);
            }
        }
    }
    // println!("{:#?}", nums);

    let mut result: u32 = 0;
    let mut mult: u32 = 1;
    for val in line[1].iter().rev() {
        for (chars, num) in &nums {
            if chars.len() == val.len() && chars.chars().map(|x| val.contains(x)).all(|x| x) {
                result += num * mult;
                mult *= 10;
                // println!("{:#?} {:#?}", val, num);
            }
        }
    }

    // println!("");
    // println!("{}", result);
    result
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data: Vec<Vec<Vec<String>>> = lines.map(
        |line| line
            .unwrap()
            .split('|')
            .map(|data|
                 data
                 .split_whitespace()
                 .map(|x| x.to_string())
                 .collect::<Vec<String>>()
                ).collect::<Vec<Vec<String>>>()
        ).collect();
    // println!("{:#?}", data);

    let mut result = 0;
    for mut line in data {
        result += get_num(&mut line);
    }

    println!("Result: {}", result);

    Ok(())
}

