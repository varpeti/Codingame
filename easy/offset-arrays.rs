use std::collections::HashMap;
use regex::Regex;

use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};

macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

#[derive(Debug)]
struct MyArray {
    offset: i32,
    data: Vec<i32>,
}

impl MyArray {
    fn new(offset: i32,  data: Vec<i32>) -> Self {
        MyArray{offset,data}
    }

    fn get(&self, n: i32) -> i32 {
        self.data.get((self.offset + n) as usize).unwrap().to_owned()
    }
}

fn main() {
    let mut l; r!(l);
    let n = p!(l, usize);
    let mut arrays = HashMap::<String,MyArray>::new();
    let re = Regex::new(r"^(\w+)\[(-?\d+)\.\.(-?\d+)\]$").unwrap();
    for _ in 0..n {
        let mut s = p!(l, " = ");
        let m = re.captures(p!(s)).unwrap();
        s = p!(s, " ");
        arrays.insert(
            m[1].to_string(), 
            MyArray::new(
                -m[2].parse::<i32>().unwrap(),
                s.map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>(),
            ),
        );
    }
    eprintln!("{:?}", arrays);
    let mut num = 0;
    let re_tokenize = Regex::new(r"\w+|\[|\]|-?\d+").unwrap();
    for token in re_tokenize.find_iter(p!(l)).map(|s| s.as_str()).collect::<Vec<_>>().iter().rev() {
        eprint!("{} ", token);
        let c: char = token.chars().next().unwrap();
        if c.is_alphabetic() {
            let e = arrays.get(&token.to_string()).unwrap();
            num = e.get(num);
        }
        else if let Ok(n) = token.parse::<i32>() {
            num = n;
        }
        eprintln!("{}", num);
    }

    println!("{}", num);
}
