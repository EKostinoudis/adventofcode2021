use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut pol: Vec<char> = Vec::new();
    // let mut template: Vec<(Vec<char>, char)> = Vec::new();
    let mut template: HashMap<String, (Vec<String>, usize)> = HashMap::new();
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
            let c: char = tmp[1].parse().unwrap();
            let c1: char = tmp[0].chars().nth(0).unwrap();
            let c2: char = tmp[0].chars().nth(1).unwrap();
            let v1: String = vec![c1, c].iter().collect();
            let v2: String = vec![c, c2].iter().collect();
            template.insert(tmp[0].clone(), (vec![v1, v2], 0));
        } else {
            pol = line.as_ref().unwrap().chars().collect();
        }
    }


    // println!("{:?}", pol);
    // println!("{:#?}", template);

    for (_i, arr) in pol.windows(2).enumerate() {
        let s: String = arr.iter().collect();
        template.get_mut(&s).unwrap().1 += 1;
    }

    for _ in 0..40 {
        let mut new_entries: Vec<(String, usize)> = Vec::new();
        let mut rem_entries: Vec<(String, usize)> = Vec::new();

        for (k, v) in &template {
            new_entries.push((v.0[0].to_owned(), v.1));
            new_entries.push((v.0[1].to_owned(), v.1));
            rem_entries.push((k.to_owned(), v.1));
        }
        for (k, v) in new_entries.into_iter() {
            template.get_mut(&k).unwrap().1 += v;
        }
        for (k, v) in rem_entries.into_iter() {
            template.get_mut(&k).unwrap().1 -= v;
        }
    }

    // println!("{:#?}", template);

    let mut res: HashMap<char, u128> = HashMap::new();
    for (k, v) in &template {
        let key: char = k.chars().nth(0).unwrap();
        if res.contains_key(&key) {
            // res[&key] += v.1;
            *res.get_mut(&key).unwrap() += v.1 as u128;
        } else {
            res.insert(key, v.1 as u128);
        }
    }

    // add the last
    *res.get_mut(&pol[pol.len()-1]).unwrap() += 1;

    // println!("{:#?}", res);
    
    let mut min = 0;
    let mut max = 0;
    let mut first = true;
    for (_k, &v) in &res {
        if first {
            first = false;
            min = v;
            max = v;
        } else {
            if min > v {
                min = v;
            }
            if max < v {
                max = v;
            }
        }
    }

    println!("Result: {}", max - min);

    Ok(())
}

