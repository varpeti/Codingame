use std::{fs::File, io::Read, os::unix::prelude::FromRawFd, collections::HashMap};
macro_rules! r { // Reads all
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
    let mut l; r!(l);
    let n = p!(l, usize);
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let word = p!(l);
        if word.len() <= 7 {
            words.push(word);
        }
    }
    let letters = p!(l);

    let mut max_word = "";
    let mut max_score = 0;
    for word in words.iter() {
        let mut letters = letters.clone().chars().collect::<Vec<char>>();
        let mut score = 0;
        for cw in word.chars() {
            let mut id = letters.len();
            for (i, &cl) in letters.iter().enumerate() {
                if cw == cl {
                    id = i;
                    break;
                }
            }
            if id != letters.len() {
                letters.swap_remove(id);
                score += get_score(cw);
            } else {
                score = 0;
                break;
            }
        }

        if score != 0 && max_score < score {
            max_score = score;
            max_word = word;
        }
    }
    println!("{}", max_word);
}

fn get_score(c: char) -> usize {
    match c {
        'e' |'a' |'i' |'o' |'n' |'r' |'t' |'l' |'s' |'u' => 1,
        'd' |'g' => 2,
        'b' |'c' |'m' |'p' => 3,
        'f' |'h' |'v' |'w' |'y' => 4,
        'k' => 5,
        'j' |'x' => 8,
        'q' |'z' => 10,
        _ => panic!("Letter not found: {}",c),
    }
}
