use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    // if position is 10 it read 0, but works the same
    let pos = input.lines().map(|l| (l.bytes().last().unwrap() - b'0') as u8).collect::<Vec<_>>();

    let mut wins = vec![0u64; 2];

    /*
     * possible 3 rolls for a player:
     * 3 : 1
     * 4 4 4 : 3
     * 5 5 5 5 5 5 : 6
     * 6 6 6 6 6 6 6 : 7
     * 7 7 7 7 7 7 : 6
     * 8 8 8 : 3
     * 9 : 1
     */
    let rolls = [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)];
    // state: s1, s2, p1, p1, turn, mult
    let mut state: Vec<(u8, u8, u8, u8, u8, u64)> = Vec::new();
    state.push((0, 0, pos[0], pos[1], 0, 1));

    while let Some(s) = state.pop() {
        let (s1, s2, p1, p2, turn, mult) = s;
        for (dice, curr_mult) in rolls {
            if turn == 0 {
                let new_p1 = ((p1 + dice - 1) % 10 ) + 1;
                let new_s1 = s1 + new_p1;
                if new_s1 < 21 {
                    state.push((new_s1, s2, new_p1, p2, 1, mult*curr_mult));
                } else {
                    wins[0] += mult*curr_mult;
                }
            } else {
                let new_p2 = ((p2 + dice - 1) % 10 ) + 1;
                let new_s2 = s2 + new_p2;
                if new_s2 < 21 {
                    state.push((s1, new_s2, p1, new_p2, 0, mult*curr_mult));
                } else {
                    wins[1] += mult*curr_mult;
                }
            }
        }
    }

    println!("Result: {}", wins.iter().max().unwrap());

    Ok(())
}

