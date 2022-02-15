use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;


fn get_offset(s1: &Vec<Vec<i32>>, s2: &Vec<Vec<i32>>) -> Option<((i32, i32, i32), u32)> {
    // for every entry in s2 find the offsets
    for t in 0..24 {
        for e2 in s2 {
            for e1 in s1 {
                let e2 = transform(&e2, t);
                let (dx, dy, dz) = (e2.0 - e1[0], e2.1 - e1[1], e2.2 - e1[2]);

                // check if we have at least 12 same points
                if s2.iter().filter(|x| {
                    let x = transform(&x, t);
                    s1.contains(&vec![x.0 - dx, x.1 - dy, x.2 - dz])
                }).count() >= 12 {
                    return Some(((dx, dy, dz), t));
                }
            }
        }
    }
    None
}

fn transform(v: &Vec<i32>, t: u32) -> (i32, i32, i32) {
    match t {
         0 => ( v[0],  v[1],  v[2]),
         1 => ( v[0], -v[1], -v[2]),
         2 => ( v[0],  v[2], -v[1]),
         3 => ( v[0], -v[2],  v[1]),
         4 => (-v[0],  v[1], -v[2]),
         5 => (-v[0],  v[2],  v[1]),
         6 => (-v[0], -v[1],  v[2]),
         7 => (-v[0], -v[2], -v[1]),

         8 => ( v[2],  v[0],  v[1]),
         9 => ( v[2], -v[0], -v[1]),
        10 => ( v[1],  v[0], -v[2]),
        11 => ( v[1], -v[0],  v[2]),
        12 => (-v[2],  v[0], -v[1]),
        13 => (-v[1],  v[0],  v[2]),
        14 => (-v[2], -v[0],  v[1]),
        15 => (-v[1], -v[0], -v[2]),

        16 => ( v[1],  v[2],  v[0]),
        17 => ( v[1], -v[2], -v[0]),
        18 => ( v[2],  v[1], -v[0]),
        19 => ( v[2], -v[1],  v[0]),
        20 => (-v[1],  v[2], -v[0]),
        21 => (-v[2],  v[1],  v[0]),
        22 => (-v[1], -v[2],  v[0]),
        23 => (-v[2], -v[1], -v[0]),
        _ => panic!(),
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut data: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut idx = 0;
    for line in lines {
        if line.as_ref().unwrap().is_empty() {
            idx += 1;
            continue;
        }

        if line.as_ref().unwrap().starts_with("---") {
            data.push(Vec::new());
        } else {
            data[idx].push(line
                      .as_ref()
                      .unwrap()
                      .split(',')
                      .map(|x| x.parse().unwrap())
                      .collect()
                     );
        }

    }

    let mut beacons = data[0].clone();
    let mut found = vec![false; data.len()-1];
    let mut offsets = Vec::new();
    offsets.push((0, 0, 0));

    while found.iter().any(|&x| !x) {
        for (i, d) in data[1..].iter().enumerate() {
            if found[i] { continue; }

            if let Some(offset) = get_offset(&beacons, &d) {
                let ((dx, dy, dz), t) = offset;
                for x in d.iter() {
                    let x = transform(&x, t);
                    let x = vec![x.0 - dx, x.1 - dy, x.2 - dz];

                    offsets.push((dx, dy, dz));

                    if !beacons.contains(&x) {
                        beacons.push(x);
                    }
                }
                found[i] = true;
            }
        }
    }

    let max = offsets.iter().permutations(2).map(|v| {
        let x = v[0];
        let y = v[1];
        (x.0 - y.0).abs() +
        (x.1 - y.1).abs() +
        (x.2 - y.2).abs()
    }).max().unwrap();
    println!("Result: {}", max);

    Ok(())
}

