use std::{io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn next_input_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

struct Pos {
    x: u32,
    y: u32,
}

struct Data{
    min: Pos,
    max: Pos,
    cur: Pos,
}

impl Data {
    fn update(&mut self, bomb_dir: &str) -> String {
        let bd = bomb_dir.chars().collect::<Vec<_>>();
        match bd[0] { // First char
            'U' => {self.max.y = self.cur.y - 1},
            'D' => {self.min.y = self.cur.y + 1},
            _ => ()
        }
        match bd.last() { // Last char, which can be the first, trick stolen from: Kalwyn
            Some('R') => {self.min.x = self.cur.x + 1},
            Some('L') => {self.max.x = self.cur.x - 1},
            _ => ()
        }
        self.cur.x = (self.max.x + self.min.x)/2;
        self.cur.y = (self.max.y + self.min.y)/2;
        format!{"{} {}", self.cur.x, self.cur.y}
    }
}

fn input() -> Data {
    let line = next_input_line();
    let inputs = line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], u32) - 1; // width of the building.
    let h = parse_input!(inputs[1], u32) - 1; // height of the building.
    next_input_line(); // n
    let line = next_input_line();
    let inputs = line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], u32);
    let y0 = parse_input!(inputs[1], u32);

    Data{min: Pos{x: 0, y: 0}, max: Pos{x: w, y: h}, cur: Pos{x: x0, y: y0}}
}

fn main() {
    
    let mut data = input();
    loop { println!("{}", data.update(next_input_line().trim())); }
}

