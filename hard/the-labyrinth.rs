use std::{io, fmt};

/*
macro_rules! r { // Reads all
    ($l:expr) => {
        let mut input = String::new();
        let mut stdin = unsafe { File::from_raw_fd(0) };
        stdin.read_to_string(&mut input).unwrap();
        $l = input.split("\n");
    };
}*/

macro_rules! rs { // Reads a single line
    ($l:expr) => {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        $l = s.split("\n"); //Dirty way to turn into an iter...
    };
}

macro_rules! p {
    ($x:expr) => ($x.next().unwrap().trim());
    ($x:expr, $t:ident) => (p!($x).parse::<$t>().unwrap());
    ($x:expr, $s:expr) => (p!($x).split($s));
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum NodeType {Start, ControlRoom, Space, Wall, Unkown}

#[derive(Clone, Debug)]
struct Node {
    t: NodeType,
}


impl Node {
    fn new(c: &char) -> Node {
        let mut node = Node{t: NodeType::Unkown};
        node.t = match c {
            'T' => NodeType::Start,
            'C' => NodeType::ControlRoom,
            '.' => NodeType::Space,
            '#' => NodeType::Wall,
            '?' => NodeType::Unkown,
            err => panic!("Unexpected char: {}", err),
        };
        node
    }

    fn as_char(&self) -> char {
        match self.t {
            NodeType::Start => 'T',
            NodeType::ControlRoom => 'C',
            NodeType::Space => '.',
            NodeType::Wall => '#',
            NodeType::Unkown => '?',
        }
    }
}

struct Maze {
    maze: Vec<Vec<Node>>,
}

impl Maze {
    fn new(r: usize, c: usize) -> Maze {
        Maze{maze: vec![vec![Node::new(&'?'); c]; r]}
    }

    fn set(&mut self, x: usize, y: usize, new_c: &char) -> &Self {
        let new_c = Node::new(new_c);
        if let Some(row) = self.maze.get_mut(y) {
            if let Some(c) = row.get_mut(x) {
                if c.t == NodeType::Unkown {c.t = new_c.t; return self;}
                if c.t != new_c.t {eprintln!("Error? {} {} {:?} {:?}", x, y, c, new_c)}
            }
        }
        self
    }
    
    fn get(&self, x: usize, y: usize) -> Node {
        if let Some(row) = self.maze.get(y) {
            if let Some(c) = row.get(x) {
                return c.clone();
            }
        }
        eprintln!("Error? {} {}", x, y);
        Node::new(&'?')
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for row in self.maze.iter() {
            for c in row.iter() {
                res.push(c.as_char())
            }
            res.push('\n');
        };
        write!(f, "{}", res)
    }
}


fn todo_remove_me_solve(maze: &Maze, rx: usize, ry: usize, todo_remove_me_state: &mut bool) -> String
{
    if *todo_remove_me_state { return "LEFT".to_string();}
    if maze.get(rx+1, ry).t != NodeType::Wall {
        return "RIGHT".to_string();
    }
    *todo_remove_me_state = true;
    "LEFT".to_string()
}

fn main() {
    let mut l; rs!(l);
    let mut i = p!(l, " ");
    let r = p!(i, usize); // number of rows.
    let c = p!(i, usize); // number of columns.
    let a = p!(i, i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.

    let mut maze = Maze::new(r, c);

    let mut todo_remove_me_state = false;
    
    // game loop
    loop {
        let mut l; rs!(l);
        let mut i = p!(l, " ");
        let ry = p!(i, usize); // row where Rick is located.
        let rx = p!(i, usize); // column where Rick is located.
        for my in 0..r {
            let mut l; rs!(l);
            for (mx, c) in p!(l).chars().enumerate() {
                maze.set(mx, my, &c);
            }
        }
        eprintln!("{:#?}", maze);
        
        println!("{}", todo_remove_me_solve(&maze, rx, ry, &mut todo_remove_me_state));
    }
}

