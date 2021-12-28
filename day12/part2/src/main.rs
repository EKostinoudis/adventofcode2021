use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn goto_next(graph: &HashMap<String, Vec<String>>, n: &Vec<String>, single: &mut Vec<String>, twice: Option<()>) -> u32 {
    let mut ret: u32 = 0;

    for p in n {
        match &p[..] {
            "start" => continue,
            "end" => ret += 1,
            x => {
                if single.contains(&x.to_string()) {
                    if twice == None {
                        ret += goto_next(graph, graph.get(x).unwrap(), single, Some(()));
                    }
                    continue;
                }

                if x.chars().all(|c| matches!(c, 'a'..='z')) {
                    let mut single2 = single.clone();
                    single2.push(x.to_string());
                    ret += goto_next(graph, graph.get(x).unwrap(), &mut single2, twice);
                } else {
                    ret += goto_next(graph, graph.get(x).unwrap(), single, twice);
                }
            },
        }
    }

    ret
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data: Vec<Vec<String>> = lines.map(
        |line| line
            .unwrap()
            .split('-')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
        ).collect();

    // println!("{:#?}", data);

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for con in data {
        let ref p1 = &con[0];
        let ref p2 = &con[1];

        graph.entry(p1.to_string()).or_insert_with(Vec::new).push(p2.to_string());
        graph.entry(p2.to_string()).or_insert_with(Vec::new).push(p1.to_string());
    }


    let mut single: Vec<String> = Vec::new();
    let start = &graph["start"];
    let result = goto_next(&graph, start, &mut single, None);

    // println!("{:#?}", graph);
    println!("Result: {}", result);

    Ok(())
}

