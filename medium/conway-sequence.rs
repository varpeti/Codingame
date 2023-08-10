use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};
macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}
macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn next(seq: &Vec::<u32>) -> Vec::<u32>{
    let mut count: u32 = 1;
    let mut seq = seq.iter();
    let mut last: &u32 = seq.next().unwrap();
    let mut next = Vec::<u32>::new();
    for i in seq {
        if last != i {
            next.push(count);
            next.push(*last);
            count = 1;
            last = i;
            continue;
        }
        count+=1;
    }
    next.push(count);
    next.push(*last);
    next
}

fn main() {
    let mut l; r!(l);
    let mut seq = vec![p!(l, u32)];
    let l = p!(l, u32);

    for _ in 1..l {
        eprintln!("{:?}", seq);
        seq = next(&seq);
    }
    println!("{}", seq.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}
