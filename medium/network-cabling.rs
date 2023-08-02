use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};

macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l; r!(l);
    let n = p!(l, usize);
    let mut max_x = i64::MIN;
    let mut min_x = i64::MAX;
    let mut ys = vec!();
    for _ in 0..n {
        let mut xy = p!(l, " ");
        let (x, y) = p!(xy, i64, i64);
        max_x = x.max(max_x);
        min_x = x.min(min_x);
        ys.push(y);
    }
    ys.sort_unstable();
    let median_y = ys.get(n/2).unwrap();
    println!("{}", ys.iter().fold(max_x-min_x, |acc, y| acc+(y-median_y).abs()));
