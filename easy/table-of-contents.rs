use std::io;

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
    let length_of_line = p!(l, usize);
    let n = p!(l, usize);
    let mut num_of_ident = vec![0; n];
    let mut prev_ident = 0;
    for _ in 0..n {
        let line = p!(l, '>').collect::<Vec<_>>();
        let ident = line.len() - 1;
        if prev_ident < ident {
            num_of_ident[ident] = 1;
        } else {
            num_of_ident[ident] += 1;
        }
        prev_ident = ident;
        let mut line = line.last().expect("Space expected").split(' ');
        let title = line.next().expect("title");
        let page_number = line.next().expect("page number");
        let ident_str = " ".repeat(4 * ident);
        let num_of_ident_str = format!("{}", num_of_ident[ident]);
        let num_of_dots = length_of_line
            - (ident_str.len() + num_of_ident_str.len() + 1 + title.len() + page_number.len());
        let dots = ".".repeat(num_of_dots);
        println!(
            "{}{} {}{}{}",
            ident_str, num_of_ident_str, title, dots, page_number,
        )
    }
}
