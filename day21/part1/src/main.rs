use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    // if position is 10 it read 0, but works the same
    let mut pos = input.lines().map(|l| (l.bytes().last().unwrap() - b'0') as u32).collect::<Vec<_>>();

    let mut score = vec![0u32; 2];
    let mut counter = 1u32;

    loop {
        // player 1
        pos[0] = ((pos[0] + (counter * 3 + 3) - 1) % 10 ) + 1;
        score[0] += pos[0];
        counter += 3;
        if score[0] >= 1000 { break; }

        // player 2
        pos[1] = ((pos[1] + (counter * 3 + 3) - 1) % 10 ) + 1;
        score[1] += pos[1];
        counter += 3;
        if score[1] >= 1000 { break; }
    }



    println!("Result: {}", (counter-1) * score.iter().min().unwrap());

    Ok(())
}

