use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    os::unix::prelude::FromRawFd,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

fn main() {
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();

    let mut data = input.trim().split("\n");
    let length = parse_input!(data.next().unwrap(), usize);
    let mut data = data.map(|line| line.split(" ").take(2)).map(|mut t| {
        Interval::new(
            parse_input!(t.next().unwrap(), usize),
            parse_input!(t.next().unwrap(), usize),
        )
    });

    let mut root = Node::new(data.next().unwrap());
    let mut cols = HashMap::<(usize, usize), HashSet<(usize, usize)>>::new();

    for i in data {
        if let Some(j) = root.search_overlap(Interval {
            low: i.low,
            high: i.high,
        }) {
            match cols.get_mut(&(i.low, i.high)) {
                None => {
                    cols.insert((i.low, i.high), set![(j.low, j.high)]);
                }
                Some(h) => {
                    h.insert((j.low, j.high));
                }
            }
            match cols.get_mut(&(j.low, j.high)) {
                None => {
                    cols.insert((j.low, j.high), set![(i.low, i.high)]);
                }
                Some(h) => {
                    h.insert((i.low, i.high));
                }
            }
        }
        root.insert(i);
    }
    //eprintln!("{:#?}", cols);
    //eprintln!("{:#?}", root);

    //Todo better reducer (or better interval tree?)
    let mut removed = 0;
    loop {
        let max = *(cols.iter().map(|v| (v.1.len(), v.0)).max().unwrap().1);
        cols.remove(&max);
        for v in cols.iter_mut() {
            v.1.remove(&max);
        }
        removed += 1;
        for v in cols.clone().iter() {
            if v.1.len() < 2 {
                cols.remove(&v.0);
            }
        }
        //eprintln!("{:#?}", cols.len());
        if cols.is_empty() {
            break;
        }
    }

    println!("{}", (length - removed));
}

// Rewritten from GeekForGeeeks c++ implementation (https://www.geeksforgeeks.org/interval-tree/)

#[derive(Debug)]
struct Interval {
    low: usize,
    high: usize,
}
#[derive(Debug)]
struct Node {
    i: Interval,
    max: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(i: Interval) -> Node {
        let max = i.high;
        Node {
            i,
            max: max,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, i: Interval) -> &Node {
        if self.max < i.high {
            self.max = i.high;
        }

        if i.low < self.i.low {
            match &mut self.left {
                None => {
                    self.left = Some(Box::new(Node::new(i)));
                }
                Some(left) => {
                    left.insert(i);
                }
            }
        } else {
            match &mut self.right {
                None => {
                    self.right = Some(Box::new(Node::new(i)));
                }
                Some(right) => {
                    right.insert(i);
                }
            }
        }

        self
    }

    fn search_overlap(&self, i: Interval) -> Option<Interval> {
        if i.is_overlaping(&self.i) {
            return Some(Interval {
                low: self.i.low,
                high: self.i.high,
            });
        }

        if let Some(left) = &self.left {
            if left.max >= i.low {
                return left.search_overlap(i);
            }
        }

        if let Some(right) = &self.right {
            return right.search_overlap(i);
        }

        None
    }
}

impl Interval {
    fn new(start: usize, duration: usize) -> Interval {
        Interval {
            low: start,
            high: start + duration,
        }
    }
    fn is_overlaping(&self, other: &Interval) -> bool {
        self.low <= other.high && other.low <= self.high
    }
}
