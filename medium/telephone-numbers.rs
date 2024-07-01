use std::{fs::File, io::Read, os::unix::prelude::FromRawFd};
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

fn main() {
    let mut l;
    r!(l);
    let n = p!(l, usize);
    //                  rootId, childrens
    let mut data = vec![(10_u8, Vec::<usize>::new())];
    for _ in 0..n {
        let num = p!(l, String);
        let mut current = 0; // root
        for a in num.chars() {
            let a = (a as u8) - 48; // ASCII number -> u8 number
            match data[current].1.iter().find(|id| data[**id as usize].0 == a) {
                None => {
                    let len = data.len();
                    data[current].1.push(len);
                    data.push((a, vec![]));
                    current = len;
                }
                Some(id) => current = *id,
            }
        }
    }
    println!("{}", data.len() - 1);
}
