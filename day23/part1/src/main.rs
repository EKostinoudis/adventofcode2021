use core::fmt;
use std::{fs, cmp::{Ordering, min, max}, collections::{BinaryHeap, HashMap}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    A,
    B,
    C,
    D,
}

impl Type {
    fn cost(&self) -> i32 {
        match self {
            Type::A => 1,
            Type::B => 10,
            Type::C => 100,
            Type::D => 1000,
        }
    }

    fn home_col(&self) -> i32 {
        match self {
            Type::A => 3,
            Type::B => 5,
            Type::C => 7,
            Type::D => 9,
        }
    }

    fn char(&self) -> char {
        match self {
            Type::A => 'A',
            Type::B => 'B',
            Type::C => 'C',
            Type::D => 'D',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Amphipod {
    t: Type,
    pos: (i32, i32),
}

impl Amphipod {
    fn is_in_home(&self) -> bool {
        match self.t {
            Type::A => (self.pos.0 == 2 && self.pos.1 == 3) || (self.pos.0 == 3 && self.pos.1 == 3),
            Type::B => (self.pos.0 == 2 && self.pos.1 == 5) || (self.pos.0 == 3 && self.pos.1 == 5),
            Type::C => (self.pos.0 == 2 && self.pos.1 == 7) || (self.pos.0 == 3 && self.pos.1 == 7),
            Type::D => (self.pos.0 == 2 && self.pos.1 == 9) || (self.pos.0 == 3 && self.pos.1 == 9),
        }
    }

    fn is_outside(&self) -> bool {
        self.pos.0 == 1
    }

    fn is_inside(&self) -> bool {
        self.pos.0 > 1
    }

    fn cost(&self, pos: (i32, i32)) -> i32 {
        ((self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs()) * self.t.cost()
    }

    fn moves(&self, state: &Vec<Amphipod>) -> Vec<(i32,i32)> {
        let mut ret: Vec<(i32,i32)> = Vec::new();

        if self.is_in_home() {
            if  self.pos.0 == 3 || 
               (self.pos.0 == 2 && state.iter().any(|a| a.pos.1 == self.pos.1 && a.pos.0 == 3 && a.is_in_home())) {
                // doesn't need to move
                return ret;
            }
        }

        if self.is_outside() {
            let col = self.t.home_col();
            let min_col = min(col, self.pos.1);
            let max_col = max(col, self.pos.1);

            // check it can go to the home
            let mut i: i32 = 3; // row to be moved (if posible)
            for a in state {
                if a.pos == self.pos { continue; }

                if a.is_outside() && a.pos.1 >= min_col && a.pos.1 <= max_col {
                    // can't move
                    return ret;
                }

                // is inside

                if a.pos.1 != col {
                    continue;
                }

                if a.pos.0 == 3 {
                    if !a.is_in_home() {
                        // can't move
                        return ret;
                    } else {
                        i = 2;
                    }
                }

                if a.pos.0 == 2 {
                    return ret;
                }
            }

            ret.push((i, col));
        } else {
            // is inside
            // posible outside cols: 1, 2, 4, 6, 8, 10, 11
            'outer: for col in [1, 2, 4, 6, 8, 10, 11] {
                let min_col = min(col, self.pos.1);
                let max_col = max(col, self.pos.1);

                for a in state {
                    if a.is_outside() && a.pos.1 >= min_col && a.pos.1 <= max_col {
                        // println!("  a:{} ; {:?}", col, a.pos);
                        continue 'outer;
                    }

                    if a.is_inside() && self.pos.0 == 3 && a.pos.1 == self.pos.1 {
                        if a.pos == self.pos { continue; } // same amphipod
                        // println!("  b:{} ; {:?}", col, a.pos);
                        continue 'outer;
                    }
                }
                ret.push((1, col));
            }
        }

        // println!("  {:?}", ret);
        ret
    }
}

fn is_over(state: &Vec<Amphipod>) -> bool {
    state.iter().all(|x| x.is_in_home())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    state: Vec<Amphipod>,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str: [Vec<char>; 5] = ["#############".chars().collect(),
                                       "#...........#".chars().collect(),
                                       "###.#.#.#.###".chars().collect(),
                                       "  #.#.#.#.#  ".chars().collect(),
                                       "  #########  ".chars().collect()];
                                 
        for a in &self.state {
            str[a.pos.0 as usize][a.pos.1 as usize] = a.t.char();
        }
        let mut res = writeln!(f, "Cost {}", self.cost);
        for line in str {
            for c in line {
                res = write!(f, "{}", c);
                if res.is_err() {
                    return res;
                }
            }
            res = writeln!(f, "");
            if res.is_err() {
                return res;
            }
        }

        res
    }
}

fn get_next_moves(node: &Node) -> Vec<Node> {
    let mut ret: Vec<Node> = Vec::new();
    for (i, a) in node.state.iter().enumerate() {
        for nmove in a.moves(&node.state) {
            let mut new_state = node.state.clone();
            let added_cost = a.cost(nmove);
            new_state[i].pos = nmove;
            ret.push(Node { state: new_state, cost: node.cost + added_cost });
        }
    }
    
    ret
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut state: Vec<Amphipod> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'A' => state.push(Amphipod{t: Type::A, pos: (i as i32, j as i32)}),
                'B' => state.push(Amphipod{t: Type::B, pos: (i as i32, j as i32)}),
                'C' => state.push(Amphipod{t: Type::C, pos: (i as i32, j as i32)}),
                'D' => state.push(Amphipod{t: Type::D, pos: (i as i32, j as i32)}),
                _ => (),
            }
        }
    }

    // println!("{:?}", state);

    let node = Node {state, cost: 0};

    let mut min_heap: BinaryHeap<Node> = BinaryHeap::new();
    min_heap.push(node);

    let mut closed: HashMap<Vec<Amphipod>, i32> = HashMap::new();

    while let Some(node) = min_heap.pop() {
        if is_over(&node.state) {
            println!("Result: {}", node.cost);
            break;
        }

        // println!("{}", node);

        closed.insert(node.state.clone(), node.cost);

        let moves = get_next_moves(&node);
        for m in moves {
            // println!("---{}", m);
            if closed.contains_key(&m.state) {
                continue;
            }
            
            min_heap.push(m.clone());
        }
        
    }
    
    Ok(())
}

