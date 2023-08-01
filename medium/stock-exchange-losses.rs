use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};

macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l; r!(l);
    let _ = p!(l);
    let mut max = 0;
    let mut min = 0;
    for i in p!(l, " ") {
        let c = i.parse::<i32>().unwrap();
        if max<c {max = c};
        if min>(c-max) {min = c-max};
    }
    println!("{}", min);
}
