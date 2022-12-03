use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};

macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn char_2_card(c: char) -> Option<u8> {
    match c {
        'A' => Some(1),
        '2'..='9' => { Some(c as u8 - 48) }
        'T' | 'J' | 'Q' | 'K' => Some(10),
        _ => None,
    }
} 

fn main() {
    let mut deck = [4; 13];
    let mut l; r!(l);
    for observation in p!(l, ".") {
        dbg!(observation);
        let cards = observation.chars().map(char_2_card).collect::<Vec<_>>();
        if cards.contains(&None) {continue;} // Got distracted
        for card in cards { deck[card.unwrap() as usize - 1] -= 1; dbg!(card.unwrap()); } // unwrap is safe, because cards cannot contain None at this point
    }
    let bust_treshold = p!(l, usize) - 1;
    let passing_cards = deck.iter().take(bust_treshold).sum::<u32>();
    let all_remaining_card = deck.iter().sum::<u32>();
    dbg!(passing_cards, all_remaining_card);
    
    println!("{:.0}%", (passing_cards as f64 / all_remaining_card as f64)*100.);
}
