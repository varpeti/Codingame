use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};
macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}
macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

struct Pos {
    x: u8,
    y: u8,
}

fn main() {
    let mut l; r!(l);
    let n = p!(l, u8);
    let mut aircrafts = Vec::<Pos>::new();
    let mut turret = Pos{x: 0, y: 0};
    for y in 0..n {
        for (x, c) in p!(l).chars().enumerate() {
            let x = x as u8;
            match c {
                '>' | '<' => aircrafts.push(Pos{x, y}), 
                '^' => turret = Pos{x, y},
                _ => (), 
            }
        }
    }
    let mut timings = [false; 40];
    let mut max = 0;
    for aircraft in aircrafts.iter() {
        let timing = ((turret.x.max(aircraft.x)-turret.x.min(aircraft.x))-(turret.y-aircraft.y)-1) as usize;
        max = max.max(timing);
        timings[timing] = true;
    }
    for t in 0..=max {
        println!("{}",match timings[t] {
           true => "SHOOT",
           false => "WAIT", 
        });
    }
}
