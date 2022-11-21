use std::{io::Read, fs::File, os::unix::prelude::FromRawFd, collections::{HashMap, BTreeMap}};

macro_rules! r { // Reads all
    ($l:expr) => {
        let mut input = String::new();
        let mut stdin = unsafe { File::from_raw_fd(0) };
        stdin.read_to_string(&mut input).unwrap();
        $l = input.split("\n");
    };
}

macro_rules! p {
    ($x:expr) => ($x.next().unwrap().trim());
    ($x:expr, $t:ident) => (p!($x).parse::<$t>().unwrap());
    ($x:expr, $s:expr) => (p!($x).split($s));
}

const MAX_TIME: u64 = 6 * 60 * 1000;
const DIST: f64 = 13.0 * 1000.0 * 60.0 * 60.0;

fn main() {
    let mut l; r!(l);
    let n = p!(l, usize);
    let mut stack = HashMap::<String, u64>::new();
    let mut badies = BTreeMap::<String, u32>::new();
    for _ in 0..n {
        let mut k = p!(l, " ");
        let plate = p!(k, String);
        let radarname = p!(k);
        let timestamp = p!(k, u64);
        match radarname {
            "A21-42" => {stack.insert(plate, timestamp);},
            "A21-55" => {
                if let Some(start_time) = stack.remove(&plate) {
                    let time = timestamp - start_time;
                    if time < MAX_TIME 
                    {
                        let speed = (DIST / time as f64).floor() as u32;
                        badies.insert(plate, speed);
                    }
                }
            },
            _ =>(),
        };
    }
    for badie in badies.iter() {
        println!("{} {}", badie.0, badie.1);
    }
}
