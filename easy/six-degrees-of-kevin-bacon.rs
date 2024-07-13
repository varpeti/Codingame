use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::Read,
    os::unix::prelude::FromRawFd,
};
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

type Actor = String;
type Film = HashSet<Actor>;
type Films = Vec<Film>;
type ActorsNodes = Vec<(Actor, usize)>;

fn main() {
    let mut l;
    r!(l);
    let start = p!(l, String);
    let n = p!(l, usize);
    let mut films = Films::new();
    for _ in 0..n {
        let line = p!(l, String);
        let film: Film = line
            .split(": ")
            .skip(1)
            .next()
            .expect(": ")
            .split(", ")
            .map(|str| str.to_string())
            .collect();
        films.push(film);
    }
    println!(
        "{}",
        astar(
            &start,
            |actor| {
                let mut actors_nodes = ActorsNodes::new();
                for film in films.iter() {
                    if film.contains(actor) {
                        for actorb in film.iter() {
                            if *actorb == *actor {
                                continue;
                            }
                            actors_nodes.push((actorb.clone(), 1));
                        }
                    }
                }
                actors_nodes
            },
            |_| { 0 }, //Dijkstra
            |actor| { *actor == "Kevin Bacon".to_string() }
        )
        .expect("solution")
        .len()
            - 1
    )
}

// Jeah BFS would be cheaper and shorter than this "Dijkstra"...
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
