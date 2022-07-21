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

fn main() {
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();

    let mut data = input.trim().split("\n");
    let length = parse_input!(data.next().unwrap(), usize);

    let mut all = HashMap::<usize, Vec<usize>>::new();
    for (id, line) in data.enumerate() {
        let d = line
            .split(" ")
            .map(|n| parse_input!(n, usize))
            .collect::<Vec<_>>();
        for i in d[0]..(d[0] + d[1]) {
            // Too slow :( I should use an itervallTree
            match all.get_mut(&i) {
                None => {
                    all.insert(i, vec![id]);
                }
                Some(v) => {
                    v.push(id);
                }
            };
        }
    }

    //TODO Find a way to collect cols directly
    let mut cols = HashMap::<usize, HashSet<&usize>>::new();
    for v in all.iter().filter(|v| v.1.len() > 1) {
        for id in v.1 {
            match cols.get_mut(id) {
                None => {
                    cols.insert(*id, v.1.iter().collect());
                }
                Some(h) => {
                    for i in v.1 {
                        h.insert(i);
                    }
                }
            };
        }
    }

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
        eprintln!("{:?}", cols);
        if cols.is_empty() {
            break;
        }
    }

    println!("{}", (length - removed));
}
