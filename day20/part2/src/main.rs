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

    let mut iters = 50*2;
    let size = map[0].len();
    for line in &mut map {
        for _ in 0..iters {
            line.insert(0, false);
        }
        line.append(&mut vec![false; iters]);
    }
    for _ in 0..iters {
        for _ in 0..iters {
            map.insert(0, vec![false; size + iters * 2]);
            map.push(vec![false; size + iters * 2]);
        }
    }

    iters /= 2;
    for _ in 0..iters {
        let mut new_map = map.clone();
        enhance(&map, &table, &mut new_map);
        map = new_map;
    }

    let mut res = 0;
    for row in map[iters..map.len()-iters].iter() {
        for col in row[iters..map[0].len()-iters].iter() {
    /*
    for row in map.iter() {
        for col in row.iter() {
    */
            res += *col as i32;
        }
    }
    println!("Result: {}", res);

    Ok(())
}

