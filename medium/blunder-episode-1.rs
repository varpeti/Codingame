use std::{collections::HashSet, fmt, fs::File, io::Read, os::unix::prelude::FromRawFd};
macro_rules! r {
    // Reads all
    ($l:expr) => {
        let mut input = String::new();
        let mut stdin = unsafe { File::from_raw_fd(0) };
        stdin.read_to_string(&mut input).unwrap();
        $l = input.split("\n");
    };
}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l;
    r!(l);
    let mut yyxx = p!(l, " ");
    let (yy, xx) = p!(yyxx, usize, usize);
    let mut map: Map = vec![vec![Tile::Empty; xx]; yy];
    let mut start = None;
    let mut teleport_a = None;
    let mut teleport_b = None;
    for y in 0..yy {
        let line = p!(l);
        for (x, c) in line.chars().enumerate() {
            let tile = Tile::from_char(c);
            if tile == Tile::Start {
                start = Some(Pos::new(y, x));
            }
            if tile == Tile::Teleport {
                match teleport_a {
                    None => teleport_a = Some(Pos::new(y, x)),
                    Some(_) => teleport_b = Some(Pos::new(y, x)),
                }
            }
            map[y][x] = tile;
        }
    }
    let teleports = match (teleport_a, teleport_b) {
        (Some(a), Some(b)) => Some((a, b)),
        (None, None) => None,
        err => panic!("Expected two teleports! {:?}", err),
    };
    simulate(map, start.expect("start"), teleports);
}

type Map = Vec<Vec<Tile>>;

fn _print_map(map: &Map, robot: &Pos) {
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match robot.y == y && robot.x == x {
                false => eprint!("{}", tile),
                true => eprint!("o"),
            }
        }
        eprintln!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Unbreakable,
    Breakable,
    Start,
    End,
    Turn(Direction),
    Beer,
    Inverse,
    Teleport,
    Empty,
    Broken,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Unbreakable,
            'X' => Tile::Breakable,
            '@' => Tile::Start,
            '$' => Tile::End,
            'S' => Tile::Turn(Direction::South),
            'E' => Tile::Turn(Direction::East),
            'N' => Tile::Turn(Direction::North),
            'W' => Tile::Turn(Direction::West),
            'B' => Tile::Beer,
            'I' => Tile::Inverse,
            'T' => Tile::Teleport,
            ' ' => Tile::Empty,
            c => panic!("Unexpected tile: '{}'", c),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Unbreakable => '#',
                Tile::Breakable => 'X',
                Tile::Start => '@',
                Tile::End => '$',
                Tile::Turn(dir) => format!("{:?}", dir).chars().next().expect("dir"),
                Tile::Beer => 'B',
                Tile::Inverse => 'I',
                Tile::Teleport => 'T',
                Tile::Empty => ' ',
                Tile::Broken => '.',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    South,
    East,
    North,
    West,
}

impl Direction {
    fn next(&self, pos: &Pos) -> Pos {
        // Underflow is not possible because the map is walled around
        match self {
            Direction::South => Pos::new(pos.y + 1, pos.x),
            Direction::East => Pos::new(pos.y, pos.x + 1),
            Direction::North => Pos::new(pos.y - 1, pos.x),
            Direction::West => Pos::new(pos.y, pos.x - 1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::South => "SOUTH",
                Direction::East => "EAST",
                Direction::North => "NORTH",
                Direction::West => "WEST",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Pos {
        Pos { y, x }
    }
}

fn simulate(mut map: Map, start: Pos, teleports: Option<(Pos, Pos)>) {
    let mut state = State::new(start);
    let mut visited_states = HashSet::<State>::new();
    let mut result = Vec::<Direction>::new();
    loop {
        _print_map(&map, &state.pos);
        visited_states.insert(state.clone());
        let next_pos = state.direction.next(&state.pos);
        let next_tile = &map[next_pos.y][next_pos.x];
        match next_tile {
            Tile::End => {
                state.next(next_pos, &mut result);
                break;
            }
            Tile::Unbreakable => state.next_direction(),
            Tile::Breakable => state.try_break(&mut map, next_pos, &mut result),
            Tile::Start => state.next(next_pos, &mut result),
            Tile::Turn(direction) => state.change_direction(direction, next_pos, &mut result),
            Tile::Beer => state.beer_up(next_pos, &mut result),
            Tile::Inverse => state.inverse(next_pos, &mut result),
            Tile::Teleport => state.teleport(
                &teleports.as_ref().expect("Teleports"),
                next_pos,
                &mut result,
            ),
            Tile::Empty => state.next(next_pos, &mut result),
            Tile::Broken => state.next(next_pos, &mut result),
        }
        if visited_states.contains(&state) {
            eprintln!("Loop detected: {:?}", state);
            println!("LOOP");
            return;
        }
    }
    for direction in result {
        println!("{}", direction);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    direction: Direction,
    direction_prio: usize,
    inverted: bool,
    breaker_mode: bool,
    num_of_broken: usize,
}

impl State {
    fn new(start: Pos) -> State {
        State {
            pos: start,
            direction: Direction::South,
            direction_prio: 0,
            inverted: false,
            breaker_mode: false,
            num_of_broken: 0, // It is used change the robot's state if the map's state is changing
        }
    }
    fn next(&mut self, next_pos: Pos, result: &mut Vec<Direction>) {
        self.pos = next_pos;
        self.direction_prio = 0;
        eprintln!("{}", self.direction);
        result.push(self.direction.clone());
    }
    fn next_direction(&mut self) {
        self.direction_prio += 1; // self.direction_prio-1 cannot be less than 0, so no underflow here
        self.direction = match self.inverted {
            false => [
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
            ],
            true => [
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::South,
            ],
        }
        .get(self.direction_prio - 1)
        .expect("Unable to move?")
        .clone();
    }
    fn change_direction(
        &mut self,
        direction: &Direction,
        next_pos: Pos,
        result: &mut Vec<Direction>,
    ) {
        self.next(next_pos, result);
        self.direction = direction.clone();
    }
    fn try_break(&mut self, map: &mut Map, next_pos: Pos, result: &mut Vec<Direction>) {
        match self.breaker_mode {
            true => {
                map[next_pos.y][next_pos.x] = Tile::Broken;
                self.num_of_broken += 1;
                self.next(next_pos, result)
            }
            false => self.next_direction(),
        }
    }
    fn beer_up(&mut self, next_pos: Pos, result: &mut Vec<Direction>) {
        self.next(next_pos, result);
        self.breaker_mode = !self.breaker_mode;
    }
    fn inverse(&mut self, next_pos: Pos, result: &mut Vec<Direction>) {
        self.next(next_pos, result);
        self.inverted = !self.inverted;
    }
    fn teleport(&mut self, teleports: &(Pos, Pos), next_pos: Pos, result: &mut Vec<Direction>) {
        self.next(next_pos.clone(), result);
        if teleports.0 == next_pos {
            self.pos = teleports.1.clone();
        } else if teleports.1 == next_pos {
            self.pos = teleports.0.clone();
        }
    }
}
