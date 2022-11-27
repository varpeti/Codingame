use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};

macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l; r!(l);
    let mut i = p!(l, " "); let (w, h, xx, yy) = p!(i, u32, u32, usize, usize);
    let mut xs = vec!(0); let mut i = p!(l, " "); for _ in 0..xx {xs.push(p!(i, u32))}
    let mut ys = vec!(0); let mut i = p!(l, " "); for _ in 0..yy {ys.push(p!(i, u32))}
    
    xs.push(w);
    ys.push(h);
    let mut count = 0;
    for x1 in xs.iter() { for x2 in xs.iter() {
        if x1 >= x2 {continue}
        let dx = x2 - x1;
        for y1 in ys.iter() { for y2 in ys.iter() {
            if y1 >= y2 {continue}
            let dy = y2 - y1;
            if dx == dy {count += 1;}
        }}
    }}
    println!("{}", count);
}
