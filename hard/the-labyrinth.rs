use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    hash::Hash,
};

use std::io;
macro_rules! rs {
    // Reads a single line
    ($l:expr) => {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        $l = s.split("\n"); // Dirty way to turn into an iter...
    };
}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l;
    rs!(l);
    let mut i = p!(l, " ");
    let (number_of_rows, number_of_columns, _alarm_time) = p!(i, usize, usize, usize);

    let mut maze = Maze::new(number_of_rows, number_of_columns);
    let mut state = State::Explore;
    loop {
        let mut l;
        rs!(l);
        let mut i = p!(l, " ");
        let rick = p!(i, usize, usize);
        let rick = Pos::new(rick.1, rick.0);

        // Update maze
        for my in 0..number_of_rows {
            let mut l;
            rs!(l);
            for (mx, c) in p!(l).chars().enumerate() {
                maze.set(mx, my, &c);
            }
        }
        // Update state if ControlRoom is reached
        if maze.get(rick.x, rick.y).t == NodeType::ControlRoom {
            state = State::ToTheEnd;
        }

        eprintln!("{:#?}", maze);
        println!("{}", get_direction(&maze, &rick, &mut state));
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum NodeType {
    Start,
    ControlRoom,
    Space,
    Wall,
    Unkown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    t: NodeType,
}

impl Node {
    fn new(c: &char) -> Node {
        let mut node = Node {
            t: NodeType::Unkown,
        };
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
        Maze {
            maze: vec![vec![Node::new(&'?'); c]; r],
        }
    }

    fn set(&mut self, x: usize, y: usize, new_c: &char) -> &Self {
        let new_c = Node::new(new_c);
        if let Some(row) = self.maze.get_mut(y) {
            if let Some(c) = row.get_mut(x) {
                if c.t == NodeType::Unkown {
                    c.t = new_c.t;
                    return self;
                }
                if c.t != new_c.t {
                    eprintln!("Error? {} {} {:?} {:?}", x, y, c, new_c)
                }
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
        }
        write!(f, "{}", res)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Explore,
    ToControl,
    ToTheEnd,
}

fn get_direction(maze: &Maze, rick: &Pos, state: &mut State) -> String {
    let goal = match state {
        State::Explore => NodeType::Unkown,
        State::ToControl => NodeType::ControlRoom,
        State::ToTheEnd => NodeType::Start,
    };

    if let Some(path) = astar(
        rick,
        |pos| get_neighs(pos, maze),
        |_| 0, // TODO heuristic
        |pos| maze.get(pos.x, pos.y).t == goal,
    ) {
        eprintln!("{:?}", path);
        let next = &path[path.len() - 2]; // -1 is where rick is
        return (match (
            next.x as isize - rick.x as isize,
            next.y as isize - rick.y as isize,
        ) {
            (-1, 0) => "LEFT",
            (1, 0) => "RIGHT",
            (0, -1) => "UP",
            (0, 1) => "DOWN",
            (dx, dy) => panic!(format!("{} {}", dx, dy)),
        })
        .to_string();
    }
    // No other Unkown room is reachable
    *state = State::ToControl;
    get_direction(maze, rick, state)
}

fn get_neighs(pos: &Pos, maze: &Maze) -> Vec<(Pos, usize)> {
    vec![
        (Pos::new(pos.x + 1, pos.y), 1_usize),
        (Pos::new(pos.x - 1, pos.y), 1_usize),
        (Pos::new(pos.x, pos.y + 1), 1_usize),
        (Pos::new(pos.x, pos.y - 1), 1_usize),
    ]
    .into_iter()
    .filter(|(pos, _)| maze.get(pos.x, pos.y).t != NodeType::Wall)
    .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

fn astar<N, FN, FH, FS>(
    start_node: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> Vec<(N, usize)>,
    FH: FnMut(&N) -> usize,
    FS: FnMut(&N) -> bool,
{
    let mut priority_queue = BinaryHeap::<AstarNode<N>>::new();
    let mut cost_and_parent = HashMap::<N, (usize, Option<N>)>::new();
    let mut visited = HashSet::<N>::new();

    priority_queue.push(AstarNode::new(start_node.clone(), 0, 0));
    cost_and_parent.insert(start_node.clone(), (0, None));

    while let Some(current) = priority_queue.pop() {
        if visited.contains(&current.node) {
            continue;
        }
        if success(&current.node) {
            let mut path = Vec::<N>::new();
            let mut current = current.node;
            while let Some((_, Some(parent))) = cost_and_parent.get(&current) {
                path.push(current);
                current = parent.clone();
            }
            path.push(start_node.clone());
            return Some(path);
        }
        visited.insert(current.node.clone());
        for (successor, move_cost) in successors(&current.node) {
            if visited.contains(&successor) {
                continue;
            }
            let tentative_cost = current.cost + move_cost;
            if tentative_cost
                > cost_and_parent
                    .get(&successor)
                    .unwrap_or(&(usize::MAX, None))
                    .0
            {
                continue;
            }
            priority_queue.push(AstarNode::new(
                successor.clone(),
                tentative_cost,
                tentative_cost + heuristic(&successor),
            ));
            cost_and_parent.insert(
                successor.clone(),
                (tentative_cost, Some(current.node.clone())),
            );
        }
    }

    None
}

#[derive(Clone, PartialEq, Eq)]
struct AstarNode<N> {
    node: N,
    cost: usize,
    priority: usize,
}

impl<N> AstarNode<N> {
    fn new(node: N, cost: usize, priority: usize) -> Self {
        AstarNode {
            node,
            cost,
            priority,
        }
    }
}

impl<N: Eq> Ord for AstarNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<N: Eq> PartialOrd for AstarNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
