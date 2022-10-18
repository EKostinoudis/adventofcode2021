use std::{fs, cmp};

#[derive(Debug, Clone, Copy)]
struct Cube {
    status: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32)
}

fn volumn(c: &Cube) -> isize {
    ((c.x.1 - c.x.0 + 1) * (c.y.1 - c.y.0 + 1) * (c.z.1 - c.z.0 + 1)) as isize
}

fn overlap(c1: &Cube, c2: &Cube, status: bool) -> Option<Cube> {
    let x = (cmp::max(c1.x.0, c2.x.0), cmp::min(c1.x.1, c2.x.1));
    let y = (cmp::max(c1.y.0, c2.y.0), cmp::min(c1.y.1, c2.y.1));
    let z = (cmp::max(c1.z.0, c2.z.0), cmp::min(c1.z.1, c2.z.1));
    
    if x.0 > x.1 || y.0 > y.1 || z.0 > z.1 {
        None
    } else {
        Some(Cube {
            status,
            x,
            y,
            z
        })
    }
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
                .parse::<i32>()
                .unwrap()
            )
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        data.push(Cube{
            status,
            x: (d[0][0], d[0][1]),
            y: (d[1][0], d[1][1]),
            z: (d[2][0], d[2][1]),
        });
    }
    let cube_overlap = Cube{
            status: true,
            x: (-50, 50),
            y: (-50, 50),
            z: (-50, 50),
           };

    let data = data.iter().filter_map(|x| overlap(x, &cube_overlap, x.status)).collect::<Vec<_>>();
    // println!("{:#?}", data);

    let mut cubes: Vec<Cube> = Vec::new();
    for cube in data.iter() {
        cubes.extend(cubes.iter().filter_map(|x| {
            overlap(cube, &x, !x.status)
        }).collect::<Vec<_>>());
        if cube.status {
            cubes.push(cube.clone());
        }
    }
    let mut result: isize = 0;
    for c in cubes.into_iter() {
        let sign = if c.status {1} else {-1};
        result += volumn(&c) * sign;
    }
    println!("{:#?}", result);

    Ok(())
}
