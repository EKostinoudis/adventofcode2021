use std::fs;

fn get_window(map: &Vec<Vec<bool>>, row: usize, col: usize) -> Vec<bool> {
    let mut res = Vec::new();

    for m in map[row-1..row+2].iter() {
        res.extend_from_slice(&m[col-1..col+2]);
    }

    res
}

fn enhance(map: &Vec<Vec<bool>>, table: &Vec<bool>, new_map: &mut Vec<Vec<bool>>) {
    for row in 1..(map.len()-1) {
        for col in 1..(map[row].len()-1) {
            let idx: usize = get_window(map, row, col)
                .iter()
                .fold(0, |res, &bit| (res << 1) ^ bit as usize);
            new_map[row][col] = table[idx];
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (table, map) = input.split_once("\n\n").unwrap();
    let table = table
        .replace("\n", "")
        .chars()
        .map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("wrong input"),
            }
        }).collect::<Vec<_>>();

    let mut map: Vec<Vec<bool>> = map
        .strip_suffix("\n")
        .unwrap()
        .split('\n')
        .map(|l| l
             .chars()
             .map(|c| {
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("wrong input"),
                }
             }).collect()
         ).collect();

    let size = map[0].len();
    for line in &mut map {
        line.insert(0, false);
        line.insert(0, false);
        line.insert(0, false);
        line.insert(0, false);
        line.insert(0, false);
        line.insert(0, false);
        line.append(&mut vec![false; 6]);
    }
    map.insert(0, vec![false; size + 12]);
    map.insert(0, vec![false; size + 12]);
    map.insert(0, vec![false; size + 12]);
    map.insert(0, vec![false; size + 12]);
    map.insert(0, vec![false; size + 12]);
    map.push(vec![false; size + 12]);
    map.push(vec![false; size + 12]);
    map.push(vec![false; size + 12]);
    map.push(vec![false; size + 12]);
    map.push(vec![false; size + 12]);
    map.push(vec![false; size + 12]);

    let mut new_map = map.clone();
    enhance(&map, &table, &mut new_map);

    let map = new_map;
    let mut new_map = map.clone();
    enhance(&map, &table, &mut new_map);
    let map = new_map;

    let mut res = 0;
    for row in map[2..map.len()-2].iter() {
        for col in row[2..map[0].len()-2].iter() {
            res += *col as i32;
        }
    }
    println!("Result: {}", res);

    Ok(())
}

