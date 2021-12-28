use std::fs::File;
use std::io::{self, BufRead};

fn update_players(players: &mut Vec<Vec<Vec<(u32, bool)>>>, num: u32) {
    for player in players {
        for rows in player {
            for val in rows {
                if val.0 == num {
                    val.1 = true;
                }
            }
        }
    }
}

fn update_winners(players: &Vec<Vec<Vec<(u32, bool)>>>, players_wins: &mut Vec<bool>) -> Option<u32> {
    let mut ret: Option<u32> = None;
    for (p, player) in players.iter().enumerate() {
        // check rows
        for row in player {
            if row.iter().all(|&y| y.1 == true) && !players_wins[p] {
                ret = Some(p as u32);
                players_wins[p] = true;
            }
        }

        // check colls
        for i in 0..5 {
            if player.iter().all(|y| y[i].1 == true) && !players_wins[p] {
                ret = Some(p as u32);
                players_wins[p] = true;
            }
        }
    }
    ret
}

fn sum_of_unmarked(players: &Vec<Vec<Vec<(u32, bool)>>>, player: u32) -> u32 {
    let mut sum: u32 = 0;

    for rows in &players[player as usize] {
        for val in rows {
            if !val.1 {
                sum += val.0;
            }
        }
    }

    sum
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();
    let nums: Vec<u32> = lines.next().unwrap()?.split(',').filter_map(|x| x.parse::<u32>().ok()).collect();
    // println!("nums: {:?}", nums);

    let mut players: Vec<Vec<Vec<(u32, bool)>>> = Vec::new();
    let mut players_wins: Vec<bool> = Vec::new();

    let mut player: i32 = -1;
    let mut col = 0;
    for line in lines {
        // check if empty line
        if line.as_ref().unwrap().is_empty() {
            players.push(vec![vec![(0, false); 5]; 5]);
            players_wins.push(false);
            player += 1;
        } else {

            for (i, num) in line.unwrap().split_whitespace().enumerate() {
                players[player as usize][col][i].0 = num.parse::<u32>().ok().unwrap();
            }
            col += 1;
            col = col % 5;
        }

    }

    for (i, num) in nums.into_iter().enumerate() {
        update_players(&mut players, num);
        if i < 5 { continue };

        let winner = update_winners(&players, &mut players_wins);

        if players_wins.iter().all(|x| *x == true) {
            // println!("{:?}", players_wins);
            match winner {
                Some(x) => {
                    let sum = sum_of_unmarked(&players, x);
                    // println!("player: {}, sum: {}, num: {}", x, sum, num);

                    println!("Reuslt: {}", sum * num);
                    break
                },
                None => continue,
            }
        }
    }

    Ok(())
}

