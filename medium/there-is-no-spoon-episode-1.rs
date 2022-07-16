use std::{io, collections::HashMap, fmt};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn next_input() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

struct Node{x: u8, y: u8}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}


fn main() {
    
    let _width = parse_input!(next_input(), u8); // the number of cells on the X axis
    let height = parse_input!(next_input(), u8); // the number of cells on the Y axis
    let mut xs = HashMap::<u8, Vec<u8>>::new();
    let mut ys = HashMap::<u8, Vec<u8>>::new();
    let mut nodes = Vec::<Node>::new();
    for y in 0..height as u8 {
        let line = next_input().trim_matches('\n').to_string(); // width characters, each either 0 or .
        for (x, c) in line.chars().enumerate() {
            let x = x as u8;
            match c {
                '0' => {
                    nodes.push(Node{x, y});
                    match xs.get_mut(&x) {
                        Some(vy) => { vy.push(y); },
                        None => { xs.insert(x, vec![y]); },
                    };
                    match ys.get_mut(&y) {
                        Some(vx) => { vx.push(x); },
                        None => { ys.insert(y, vec![x]); },
                    };
                },
                '.' => {},
                oth => panic!("Unexcepted char '{}'!", oth),
            };
        }
    }

    for node in nodes {

        let right_neighbour = match ys.get(&node.y) {
            Some(vx) => {
                match vx.iter().find(|x| **x > node.x) {
                    Some(x) => { Some(Node{y: node.y, x: *x}) },
                    None => { None }
                }
            },
            None => { None },
        };
        let below_neighbour = match xs.get(&node.x) {
            Some(vy) => {
                match vy.iter().find(|y| **y > node.y) {
                    Some(y) => { Some(Node{x: node.x, y: *y}) },
                    None => { None }
                }
            },
            None => { None },
        };
        println!("{:?} {} {}", node, 
            match right_neighbour { Some(n) => {format!("{:?}", n)}, None => {"-1 -1".to_string()} },
            match below_neighbour { Some(n) => {format!("{:?}", n)}, None => {"-1 -1".to_string()} },
        )
    }
}

