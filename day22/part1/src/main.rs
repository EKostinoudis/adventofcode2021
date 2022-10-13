use std::{fs, io::Read};

#[derive(Debug)]
struct Cube {
    status: bool,
    x: (u32, u32),
    y: (u32, u32),
    z: (u32, u32)
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let data: &mut Vec<Cube> = &mut Vec::new();
    for line in input.lines() {
        let mut d = line.split(" ");
        let status: bool = d.next().unwrap() == "on";

        let d= d.next().unwrap().split(',').map(|x| x
            .split("..")
            .map(|x| x
                .split("=")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap()
                
            )
            .collect::<Vec<_>>()
            
        ).collect::<Vec<_>>();

        data.push(Cube{
            status: status,
            x: (d[0][0], d[0][1]),
            y: (d[1][0], d[1][1]),
            z: (d[2][0], d[2][1]),
        });
    }


    println!("{:#?}", data);

    Ok(())
}
