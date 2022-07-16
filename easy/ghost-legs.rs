use std::{io};

fn get_next_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim_matches('\n').to_string()
}

fn main() {
    let wh = get_next_line();
    let h = wh.split(" ").collect::<Vec<_>>()[1].trim().parse::<usize>().unwrap();
    
    let orig_t = get_next_line().chars().enumerate().step_by(3).map(|x| x.1).collect::<Vec<char>>();
    let mut t = orig_t.to_vec();
    for _ in 1..(h-1) {
        let l = get_next_line().chars().enumerate().skip(1).step_by(3).map(|x| x.1).collect::<Vec<char>>();
        for (i, c) in l.iter().enumerate() { if *c == '-' { t.swap(i, i+1) } }
    }
    let b = get_next_line().chars().enumerate().step_by(3).enumerate().map(|x| (x.1.1, t[x.0])).collect::<Vec<(char, char)>>();
    
    for c in orig_t.iter()
    {
        let k = b.iter().find(|x| x.1 == *c).unwrap();
        println!("{}{}", k.1, k.0);
    }
}
