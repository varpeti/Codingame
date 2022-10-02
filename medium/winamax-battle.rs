use std::{io::Read, fs::File, os::unix::prelude::FromRawFd, cmp::Ordering, collections::VecDeque};

macro_rules! parse_input { ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum  Card { N2, N3, N4, N5, N6, N7, N8, N9, N10, J, Q, K, A}

fn parse_card(card: &str) -> Option<Card> {
    let mut c = card.chars();
    c.next_back();
    match c.as_str() {
       "2" => Some(Card::N2),
       "3" => Some(Card::N3),
       "4" => Some(Card::N4),
       "5" => Some(Card::N5),
       "6" => Some(Card::N6),
       "7" => Some(Card::N7),
       "8" => Some(Card::N8),
       "9" => Some(Card::N9),
       "10" => Some(Card::N10),
       "J" => Some(Card::J),
       "Q" => Some(Card::Q),
       "K" => Some(Card::K),
       "A" => Some(Card::A),
        _ => None
    }
}

fn solve(mut p1: VecDeque::<Card>, mut p2: VecDeque::<Card>) -> String {
    let mut rounds = 0;
    let mut t1 = VecDeque::new();
    let mut t2 = VecDeque::new();
    loop {
        if p1.is_empty() { return format!("2 {}", rounds); }
        if p2.is_empty() { return format!("1 {}", rounds); }
        let c1 = p1.pop_front().unwrap(); // unwrap is OK because we check emptiness before
        let c2 = p2.pop_front().unwrap();
        eprint!("{}: {:?}-{:?}: ", rounds, c1, c2);
        let cmp = c1.cmp(&c2);
        t1.push_back(c1); 
        t2.push_back(c2);
        match cmp {
            Ordering::Less => {p2.append(&mut t1); p2.append(&mut t2); eprintln!("P2 win!");rounds += 1;},
            Ordering::Greater => {p1.append(&mut t1); p1.append(&mut t2); eprintln!("P1 win!");rounds += 1;},
            Ordering::Equal => { // War
                eprintln!("WAR");
                if p1.len() < 4 || p2.len() < 4 {break;}
                for _ in 0..3 {
                    t1.push_back(p1.pop_front().unwrap()); // unwrap is OK because we check len before
                    t2.push_back(p2.pop_front().unwrap());
                }
            },
        }
        //eprintln!("{:?} {:?}", p1, p2);
    }
    "PAT".to_string()
}

fn main() {
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.split("\n");
    
    let mut p1 = VecDeque::new();
    let mut p1_num_of_card = parse_input!(lines.next().unwrap(), i32);
    while p1_num_of_card>0 {
        if let Some(card) = parse_card(lines.next().unwrap()) { p1.push_back(card) }
        p1_num_of_card -= 1;
    }
    let mut p2 = VecDeque::new();
    for line in lines.skip(1) {
        if let Some(card) = parse_card(line) { p2.push_back(card) }
    }
    //eprintln!("{:?} {:?}", p1, p2);
    println!("{}", solve(p1, p2));
}
