use std::{io, cmp::Ordering};

macro_rules! rs {($l:expr) => {let mut s = String::new();io::stdin().read_line(&mut s).unwrap();$l = s.split("\n");};}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l; rs!(l); let mut s = p!(l, " ");
    let (nb_floors, width, nb_rounds, exit_floor, exit_pos, _nb_total_clones, _nb_additional_elevators, nb_elevators) = p!(s, 
         u8,        u8,    u8,        u8,         i8,       u8,               u8,                       usize);
    let mut elevators = vec!(0; nb_elevators);
    for _ in 0..nb_elevators {
        rs!(l); s = p!(l, " ");
        let (floor, pos) = p!(s, usize, i8);
        elevators[floor] = pos;
    }
    elevators.push(exit_pos);
    let mut floor_ok: i8 = -1;
    eprintln!("{} {} {} {} {} {:?}", nb_floors, width, nb_rounds, exit_floor, exit_pos, elevators);
    loop {
        rs!(l); s = p!(l, " ");
        let (clone_floor, clone_pos, direction) = p!(s, i8, i8, String);
        eprintln!("{} {} {}", clone_floor, clone_pos, direction);

        if floor_ok > clone_floor || clone_pos<0 {
            println!("WAIT");
            continue;
        }

        match (clone_pos.cmp(&elevators[(clone_floor as usize)]), direction.as_str()) {
            (Ordering::Less, "LEFT") | (Ordering::Greater, "RIGHT") => println!("BLOCK"),
            (_, _) => println!("WAIT"),
        };
        floor_ok += 1;
        eprintln!("Floor #{} done!", floor_ok);
    }
}
