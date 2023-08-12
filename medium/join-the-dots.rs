use std::collections::HashMap;
use std::{io::Read,fs::File,os::unix::prelude::FromRawFd};
macro_rules! r{($l:expr)=>{let mut i=String::new();let mut s=unsafe{File::from_raw_fd(0)};s.read_to_string(&mut i).unwrap();$l=i.split("\n");};}
macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Horizontal,   // -
    Vertical,     // |
    DiagonalUrDl, // /
    DiagonalUlDr, // \
}

type Paper = Vec<Vec<char>>;
type Points = HashMap<char, Pos>;

fn get_char(paper: &Paper, pos: &Pos) -> char {
    paper[pos.y][pos.x]
}

fn get_next_char(c: char) -> char {
    match c {
        '9' => 'A',
        c => (c as u8 + 1) as char,
    }
}

fn get_direction(start: &Pos, end: &Pos) -> Direction {
   if start.y == end.y { return Direction::Vertical;}
   if start.x == end.x { return Direction::Horizontal;} 
   match ((start.x < end.x), (start.y < end.y)) {
        (false, true) | (true, false) => Direction::DiagonalUrDl, 
        (true, true) | (false, false) => Direction::DiagonalUlDr,
   }
}

// This function is unused, because the last validation "lies" about the size of the paper. I use the `get_paper2() -> (Paper, Points)` instead, which uses the provided height and width.
fn _get_paper() -> (Paper, Points) {
    let mut l; r!(l);
    let _ = p!(l);
    let mut points = Points::new();
    let paper = l.enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, c)| {
            if c != '.' { points.insert(c, Pos{x,y});}
            c
        }).collect::<Vec<_>>()).collect::<Vec<_>>();
    (paper, points)
}

fn get_paper2() -> (Paper, Points) {
    let mut l; r!(l); 
    let mut s = p!(l, " "); let (h, w) = p!(s, usize, usize);
    let mut points = Points::new();
    let mut paper = vec![vec!['.'; w]; h];
    for y in 0..h {
        let line = p!(l, String).as_bytes().to_owned();
        for x in 0..w {
            let c = line[x] as char;
            if c != '.' { points.insert(c, Pos{x,y});}
            paper[y][x] = c;
        }
    }

    (paper, points)
}

fn set_char(paper: &mut Paper, pos: &Pos, c: char) {
    paper[pos.y][pos.x] = c;
}

fn get_range(a: usize, b: usize) -> std::ops::Range<i32> {
    let dif = (b as i32) - (a as i32);
    dif.min(0)..dif.max(0)
}

fn set_horizontal(paper: &mut Paper, p: &Pos, p_next: &Pos) {
    for i in get_range(p.y, p_next.y) {
        let pp = Pos{x: p.x, y: ((p.y as i32) + i) as usize};
        let cc = get_char(&paper, &pp);
        let cc_next = match cc {
            '.' | '|' => '|',
            '-' | '+' => '+',
            '/' | '\\' | 'X' |'*' => '*',
            cc => cc, 
        };
        set_char(paper, &pp, cc_next);
    }
}

fn set_vertical(paper: &mut Paper, p: &Pos, p_next: &Pos) {
    for i in get_range(p.x, p_next.x) {
        let pp = Pos{x: ((p.x as i32) + i) as usize, y: p.y};
        let cc = get_char(&paper, &pp);
        let cc_next = match cc {
            '.' | '-' => '-',
            '|' | '+' => '+',
            '/' | '\\' | 'X' | '*' => '*',
            cc => cc, 
        };
        set_char(paper, &pp, cc_next);
    }
}

fn set_diagonal_ul_dr(paper: &mut Paper, p: &Pos, p_next: &Pos) {
    for i in get_range(p.y, p_next.y) {
        let pp = Pos{x: ((p.x as i32) + i) as usize,  y: ((p.y as i32) + i) as usize};
        let cc = get_char(&paper, &pp);
        let cc_next = match cc {
            '.' | '\\' => '\\',
            '/' | 'X' => 'X',
            '-' | '|' | '+' | '*' => '*',
            cc => cc, 
        };
        set_char(paper, &pp, cc_next);
    }
}

fn set_diagonal_ur_dl(paper: &mut Paper, p: &Pos, p_next: &Pos) {
    for i in get_range(p.y, p_next.y) {
        let pp = Pos{x: ((p.x as i32) - i) as usize,  y: ((p.y as i32) + i) as usize};
        let cc = get_char(&paper, &pp);
        let cc_next = match cc {
            '.' | '/' => '/',
            '\\' | 'X' => 'X',
            '-' | '|' | '+' | '*' => '*',
            cc => cc, 
        };
        set_char(paper, &pp, cc_next);
    }
}

fn clear_dots(paper: &mut Paper) {
    for line in paper.iter_mut() {
        for c in line.iter_mut() {
            if *c == '.' { *c = ' ';}
        }
    }
}

fn print_paper(paper: &Paper) {
    println!("{}", paper.iter().map(|line| {
        line.iter().collect::<String>().trim_end().to_owned()
    }).collect::<Vec<_>>().join("\n"));
}

fn main() {
    let (mut paper, points) = get_paper2();
    let mut c = '1';
    let mut p = points.get(&c).unwrap();
    loop {
        set_char(&mut paper, p, 'o');
        let c_next = get_next_char(c);
        if let Some(p_next) = points.get(&c_next) {
            let direction = get_direction(p, p_next); 
            match direction {
                Direction::Vertical => set_vertical(&mut paper, p, p_next),
                Direction::Horizontal => set_horizontal(&mut paper, p, p_next),
                Direction::DiagonalUrDl => set_diagonal_ur_dl(&mut paper, p, p_next),
                Direction::DiagonalUlDr => set_diagonal_ul_dr(&mut paper, p, p_next),
            }
            (c, p) = (c_next, p_next);
        } else { break; }
    }
    clear_dots(&mut paper);
    print_paper(&paper);
}
