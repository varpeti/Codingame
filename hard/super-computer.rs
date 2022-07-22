use std::{fs::File, io::Read, os::unix::prelude::FromRawFd};
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
fn interval_from_start_duration_to_end_start(start: usize, duration: usize) -> (usize, usize) {
    (start + duration, start)
}

fn main() {
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();
    let mut data = input
        .trim()
        .split("\n")
        .skip(1)
        .map(|line| line.split(" ").take(2))
        .map(|mut t| {
            interval_from_start_duration_to_end_start(
                parse_input!(t.next().unwrap(), usize),
                parse_input!(t.next().unwrap(), usize),
            )
        })
        .collect::<Vec<_>>();

    // It's called the Activity Selection Problem...
    data.sort();
    let mut selected = 0;
    let mut counter = 1;
    for cur in 1..data.len() {
        if data[cur].1 >= data[selected].0 {
            counter += 1;
            selected = cur;
        }
    }
    println!("{}", counter);
}
