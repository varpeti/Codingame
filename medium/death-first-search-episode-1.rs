use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

//use std::{fs::File, io::Read, os::unix::prelude::FromRawFd};
/*
macro_rules! r { // Reads all
    ($l:expr) => {
        let mut input = String::new();
        let mut stdin = unsafe { File::from_raw_fd(0) };
        stdin.read_to_string(&mut input).unwrap();
        $l = input.split("\n");
    };
}*/

use std::io;
macro_rules! rs {
    // Reads a single line
    ($l:expr) => {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        $l = s.split("\n"); //Dirty way to turn into an iter...
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
    let mut nums = p!(l, " ");
    let (n_nodes, n_links, n_gateways) = p!(nums, usize, usize, usize);
    eprintln!("{} {} {}", n_nodes, n_links, n_gateways);
    let mut links = vec![vec![]; n_nodes];
    for _ in 0..n_links {
        let mut l;
        rs!(l);
        let mut link = p!(l, " ");
        let (a, b) = p!(link, usize, usize);
        links[a].push(b);
        links[b].push(a);
    }
    eprintln!("{:?}", links);
    let mut gateways = vec![];
    for _ in 0..n_gateways {
        let mut l;
        rs!(l);
        gateways.push(p!(l, usize));
    }
    eprintln!("{:?}", gateways);
    loop {
        let mut l;
        rs!(l);
        let bobnet_pos = p!(l, usize);
        if let Some(path) = astar(
            &bobnet_pos,
            |id| links[*id].iter().map(|id| (*id, 1_usize)).collect(),
            |_| 0, // We don't need heuristic
            |id| gateways.contains(id),
        ) {
            eprintln!("{:?}", path);
            let len = path.len();
            let a = path[len - 1];
            let b = path[len - 2];
            links[a].retain(|x| *x != b);
            links[b].retain(|x| *x != a);
            println!("{} {}", a, b);
        }
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
