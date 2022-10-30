use std::{io::Read, fs::File, os::unix::prelude::FromRawFd, collections::HashMap};

#[derive(Debug)]
enum Direction {R, L}

impl Direction {
    fn new(input: &str) -> Result<Direction, String> {
        match input {
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            err => Err(format!("Invalid direction: {}", err)),
        }
    }
}

#[derive(Debug)]
struct Action {
    w: i32,
    m: Direction,
    next: String,
}

impl Action {
    fn new(input: &str) -> Result<Action, String> {
        let mut i = input.split(" ");
        Ok(Action{
            w: i.next().ok_or("missing w")?.parse::<i32>().map_err(|_| "parseError at w")?,
            m: Direction::new(i.next().ok_or("missing m")?)?,
            next: i.next().ok_or("missing next")?.to_string(),
        })
    }
}

#[derive(Debug)]
struct State {
    actions: Vec<Action>
}

impl State {
    fn new(input: &str) -> Result<State, String> {
        let mut actions = Vec::<Action>::new();
        for sa in input.split(";") {
            actions.push(Action::new(sa).map_err(|err|format!("Invalid input: '{}', Err: {}", sa, err))?);
        }
        Ok(State{actions})
    }
}

#[derive(Debug)]
struct TuringMachine {
    states: HashMap::<String, State>,
    current_state: String,
    tape: Vec<i32>,
    position: i32,
    actions_performed: u32,
}

impl TuringMachine {
    fn run(&mut self) {
        loop { 
            if self.current_state == "HALT" { eprintln!("HALT"); return; }
    
            let actions = match self.states.get(&self.current_state) {
                Some(state) => &state.actions,
                None => {eprintln!("STATE"); return;},
            };
    
            let action_id = match self.tape.get(self.position as usize) {
                Some(action_id) => *action_id as usize,
                None => {eprintln!("TAPE"); return;},
            };
    
            let action = match actions.get(action_id) {
                Some(action) => action,
                None => {eprintln!("ACTION"); return;},
            };
    
            self.tape[self.position as usize] = action.w;
            match action.m {
                Direction::L => self.position -= 1,
                Direction::R => self.position += 1,
            }
            self.current_state = action.next.clone();
            self.actions_performed += 1;
        }
    }

    fn get_result(&self) -> String{
        let mut tape = String::new();
        for cell in self.tape.iter() {
            tape = format!("{}{}", tape, cell);
        }
        format!("{}\n{}\n{}", self.actions_performed, self.position, tape)
    }
}

macro_rules! i { ($x:expr) => ($x.next().unwrap().trim())}
macro_rules! pi { ($x:expr, $t:ident) => (i!($x).parse::<$t>().unwrap()) }
macro_rules! pil { ($x:expr, $s:expr) => (i!($x).split($s)) }

fn main() {
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();
    let mut l = input.split("\n");

    let mut inputs = pil!(l, " ");
    let _s = pi!(inputs, i32);
    let t = pi!(inputs, usize);
    let x = pi!(inputs, i32);
    let start = i!(l).to_string(); 
    let n = pi!(l, usize);
    let mut states = HashMap::<String, State>::new();
    for _ in 0..n {
       let mut inputs = pil!(l,":");
       let name = i!(inputs).to_string();
       let state = State::new(i!(inputs)).unwrap();
       states.insert(name, state);
    }

    let mut tm = TuringMachine{states, current_state: start, tape: vec![0; t], position: x, actions_performed: 0};
    eprintln!("{:?}", tm);
    tm.run();
    println!("{}", tm.get_result());
}
