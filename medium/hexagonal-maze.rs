use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use std::{fs::File, io::Read, os::unix::prelude::FromRawFd};
macro_rules! r {
    ($l:expr) => {
        let mut i = String::new();
        let mut s = unsafe { File::from_raw_fd(0) };
        s.read_to_string(&mut i).unwrap();
        $l = i.split("\n");
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
    let mut wh = p!(l, " ");
    let (w, h) = p!(wh, usize, usize);
    eprintln!("h{}, w{}", h, w);
    let mut map: Vec<Vec<char>> = vec![vec![' '; w]; h];
    let mut start = Cell::new(0, 0);
    let mut end = Cell::new(0, 0);
    for y in 0..h {
        for (x, c) in p!(l).chars().enumerate() {
            map[y][x] = c;
            match c {
                'S' => start = Cell::new(y, x),
                'E' => end = Cell::new(y, x),
                _ => (),
            }
        }
    }
    eprintln!("s: {:?}, e: {:?}", start, end);
    //_test_get_neigh_cell();

    let path = astar(
        &start,
        |cell| {
            cell.get_all_neighs(h, w)
                .into_iter()
                .filter(|successor| map[successor.y][successor.x] != '#')
                .map(|successor| (successor.clone(), 1_usize))
                .collect()
        },
        |_cell| 0, //TODO heuristic
        |cell| *cell == end,
    )
    .expect("Path not found!");

    for cell in path.iter() {
        if map[cell.y][cell.x] == '_' {
            map[cell.y][cell.x] = '.';
        }
    }

    for y in 0..h {
        for x in 0..w {
            print!("{}", map[y][x]);
        }
        println!();
    }
}

enum Neigh {
    UR,
    RR,
    DR,
    DL,
    LL,
    UL,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Cell {
    y: usize,
    x: usize,
}

impl Cell {
    fn new(y: usize, x: usize) -> Cell {
        Cell { y, x }
    }

    fn get_neigh_cell(&self, neigh: Neigh, h: usize, w: usize) -> Cell {
        match (neigh, self.y % 2 == 0) {
            (Neigh::UR, false) => Cell::new(
                (self.y as i32 - 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 1).rem_euclid(w as i32) as usize,
            ),
            (Neigh::UR, true) => Cell::new(
                (self.y as i32 - 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 0).rem_euclid(w as i32) as usize,
            ),
            (Neigh::RR, _) => Cell::new(
                (self.y as i32 + 0).rem_euclid(h as i32) as usize,
                (self.x as i32 + 1).rem_euclid(w as i32) as usize,
            ),
            (Neigh::DR, false) => Cell::new(
                (self.y as i32 + 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 1).rem_euclid(w as i32) as usize,
            ),
            (Neigh::DR, true) => Cell::new(
                (self.y as i32 + 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 0).rem_euclid(w as i32) as usize,
            ),
            (Neigh::DL, false) => Cell::new(
                (self.y as i32 + 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 0).rem_euclid(w as i32) as usize,
            ),
            (Neigh::DL, true) => Cell::new(
                (self.y as i32 + 1).rem_euclid(h as i32) as usize,
                (self.x as i32 - 1).rem_euclid(w as i32) as usize,
            ),
            (Neigh::LL, _) => Cell::new(
                (self.y as i32 + 0).rem_euclid(h as i32) as usize,
                (self.x as i32 - 1).rem_euclid(w as i32) as usize,
            ),
            (Neigh::UL, false) => Cell::new(
                (self.y as i32 - 1).rem_euclid(h as i32) as usize,
                (self.x as i32 + 0).rem_euclid(w as i32) as usize,
            ),
            (Neigh::UL, true) => Cell::new(
                (self.y as i32 - 1).rem_euclid(h as i32) as usize,
                (self.x as i32 - 1).rem_euclid(w as i32) as usize,
            ),
        }
    }

    fn get_all_neighs(&self, h: usize, w: usize) -> [Cell; 6] {
        use Neigh as N;
        [N::UR, N::RR, N::DR, N::DL, N::LL, N::UL].map(|neigh| self.get_neigh_cell(neigh, h, w))
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
    let mut priority_queue = BinaryHeap::<Node<N>>::new();
    let mut cost_and_parent = HashMap::<N, (usize, Option<N>)>::new();
    let mut visited = HashSet::<N>::new();

    priority_queue.push(Node::new(start_node.clone(), 0, heuristic(start_node)));
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
            priority_queue.push(Node::new(
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
struct Node<N> {
    node: N,
    cost: usize,
    priority: usize,
}

impl<N> Node<N> {
    fn new(node: N, cost: usize, priority: usize) -> Self {
        Node {
            node,
            cost,
            priority,
        }
    }
}

impl<N: Eq> Ord for Node<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<N: Eq> PartialOrd for Node<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn _test_get_neigh_cell() {
    let (h, w) = (4, 4);
    let cell = Cell::new(0, 1); //B

    assert_eq!(cell.get_neigh_cell(Neigh::UR, h, w), Cell { y: 3, x: 1 });
    assert_eq!(cell.get_neigh_cell(Neigh::RR, h, w), Cell { y: 0, x: 2 });
    assert_eq!(cell.get_neigh_cell(Neigh::DR, h, w), Cell { y: 1, x: 1 });
    assert_eq!(cell.get_neigh_cell(Neigh::DL, h, w), Cell { y: 1, x: 0 });
    assert_eq!(cell.get_neigh_cell(Neigh::LL, h, w), Cell { y: 0, x: 0 });
    assert_eq!(cell.get_neigh_cell(Neigh::UL, h, w), Cell { y: 3, x: 0 });

    let cell = Cell::new(1, 3); // H
    assert_eq!(cell.get_neigh_cell(Neigh::UR, h, w), Cell { y: 0, x: 0 });
    assert_eq!(cell.get_neigh_cell(Neigh::RR, h, w), Cell { y: 1, x: 0 });
    assert_eq!(cell.get_neigh_cell(Neigh::DR, h, w), Cell { y: 2, x: 0 });
    assert_eq!(cell.get_neigh_cell(Neigh::DL, h, w), Cell { y: 2, x: 3 });
    assert_eq!(cell.get_neigh_cell(Neigh::LL, h, w), Cell { y: 1, x: 2 });
    assert_eq!(cell.get_neigh_cell(Neigh::UL, h, w), Cell { y: 0, x: 3 });
}
