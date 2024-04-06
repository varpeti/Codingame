use std::collections::HashMap;
use itertools::Itertools;


use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};
macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}
macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}


fn main() {
    // Gathering data
    let mut l; r!(l); 
    let nbtiles = p!(l, usize);
    let mut char2score = HashMap::<char, u32>::new();
    for _ in 0..nbtiles {
        let mut i = p!(l, " ");
        let c = p!(i, char);
        let score = p!(i, u32);
        char2score.insert(c, score);
    }
    eprintln!("{:?}", char2score);

    let mut xy = p!(l, " ");
    let xx = p!(xy, usize);
    let yy = p!(xy, usize);

    let mut board = vec![vec![Node::new(); xx]; yy];
    for y in 0..yy {
        for (x, c) in p!(l).chars().enumerate() {
            match c {
               'l' => board[y][x].node_score_multiplier = 2,
               'L' => board[y][x].node_score_multiplier = 3,
               'w' => board[y][x].word_score_multiplier = 2,
               'W' => board[y][x].word_score_multiplier = 3,
               _ => (),
            }
        }
    }
    for y in 0..yy {
        for (x, c) in p!(l).chars().enumerate() {
            board[y][x].letter = c;
        }
    }
    let mut bonus50 = 0; // If it is reaches 7 we applie a bonus 50 point
    for y in 0..yy {
        for (x, c) in p!(l).chars().enumerate() {
            let node = &board[y][x];
            if node.letter != c {
                board[y][x].is_new = true;
                bonus50 += 1;
            }
            board[y][x].letter = c;
        }
    }
    //eprintln!("{:#?}", board);

    // Solve
    // It searches for all words from top left to bottom right, and stores only if it contains a new letter.
    let mut new_words = Vec::<Word>::new();
    for y in 0..yy {
        for x in 0..xx {
            if board[y][x].letter == '.' {continue} // Early skip
            // Only check Down and Right
            for direction in [Direction::Down, Direction::Right] {
                // If the previous position is not '.' or the board's edge then we skip
                // it is required to not include sub-words.
                if let Some((py, px)) = match direction {
                    Direction::Down => Direction::Up.change(y, x, yy, xx),
                    Direction::Right => Direction::Left.change(y, x, yy, xx),
                    _ => unreachable!(),
                } {
                    if board[py][px].letter != '.' {continue}
                }
                // New word, also reset the y, x positon
                let mut word = Word::new();
                let mut ny = y;
                let mut nx = x;
                // It is a do while like loop: we break if the word ends
                // either '.' is the next letter or we reached the edge of the board
                loop {
                    let node = &board[ny][nx];
                    if node.letter == '.' {break}
                    if node.is_new { // Only the new letter has multipiers
                        word.score += char2score[&node.letter]*node.node_score_multiplier;
                        word.multiplier *= node.word_score_multiplier;
                        word.is_new = true;
                    } else {
                        word.score += char2score[&node.letter];
                    }
                    word.letters.push(node.letter);
                    // Getting the next letter's position, if available
                    if let Some((nny, nnx)) = direction.change(ny, nx, yy, xx) {
                        nx = nnx;
                        ny = nny;
                    } else {
                        break;
                    }
                }
                if word.letters.len()<2 {continue} // 2 letter word is the min
                eprintln!("{:?}", word);
                if !word.is_new {continue} // Do not store old words
                new_words.push(word);
            }
        }
    }
    // Output
    new_words.sort_by(|a, b| a.letters.cmp(&b.letters) ); // Sorting by lexicographical order
    let mut total = 0;
    for word in new_words.into_iter() {
        let score = word.score*word.multiplier;
        println!("{} {}", word.letters.iter().join(""), score);
        total += score;
    }
    if bonus50 >= 7 {
        println!("Bonus 50");
        total += 50;
    }
    println!("Total {}", total);
}


#[derive(Debug, Clone)]
struct Node {
    node_score_multiplier: u32,
    word_score_multiplier: u32,
    letter: char,
    is_new: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            node_score_multiplier: 1,
            word_score_multiplier: 1,
            letter: '.',
            is_new: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Right, Down,
    Left, Up,
}

impl Direction {
    /// Returs the next letter positon, given the direction, (y,x) positon and upper bounds (yy,xx) of the board.
    /// Returs `None` if out of bounds.
    fn change(&self, y: usize, x: usize, yy: usize, xx: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Right => if x+1<xx {Some((y, x+1))} else {None}, 
            Direction::Down => if y+1<yy {Some((y+1, x))} else {None}, 
            Direction::Left => if x>0 {Some((y, x-1))} else {None},
            Direction::Up => if y>0 {Some((y-1, x))} else {None},
        }
    }
}

#[derive(Debug, Clone)]
struct Word {
    score: u32,
    multiplier: u32,
    is_new: bool,
    letters: Vec::<char>,
}

impl Word {
    fn new() -> Self {
        Word{
        score: 0,
        multiplier: 1,
        is_new: false,
        letters: Vec::new(),
        }
    }
}
