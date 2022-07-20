use std::{io::{Read}, fs::File, os::unix::prelude::FromRawFd};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    
    let mut input = String::new();
    let mut stdin = unsafe { File::from_raw_fd(0) };
    stdin.read_to_string(&mut input).unwrap();
    let lines = input.split("\n").collect::<Vec<_>>();

    let inputs = lines[0].split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], u32);
    let mut c = parse_input!(inputs[1], u32);
    //let n = parse_input!(inputs[2], usize);
    let mut queue = Vec::<u32>::new();
    for i in 1..lines.len()-1 {queue.push(parse_input!(lines[i], u32));}

    let mut sequences = Vec::<Vec<u32>>::new(); // Save all possible sequences til it repeates itself (so we trade memory for time gain)
    let mut sequence = Vec::<u32>::new();
    let mut offset: Option::<usize> = None; // Number of init sequences before repeated sequences.
    let mut overflow = false;
    let mut i: usize = 0; // Where are we at atm in the queue
    loop {
        let one_seat_for_one_person = i;
        let mut ride_group: u32 = 0;
        loop {
            ride_group += queue[i];
            i+=1; if i==queue.len() { i=0; overflow=true; }
            if one_seat_for_one_person == i || ride_group + queue[i] > l { break; }
        }
        sequence.push(ride_group);
        if overflow {
            overflow = false;
            offset = sequences.iter().position(|s| *s == sequence);
            if offset != None { break; } // Found a repeate
            sequences.push(sequence.clone());
            sequence.clear();
        }
    }
    let offset = offset.unwrap();
    eprintln!("{:?}, {:?}", sequences, offset);

    //Before the repeate
    let mut init: u32 = 0;
    for si in 0..offset {
        init += sequences[si].iter().sum::<u32>();
        c -= sequences[si].len() as u32;
    }
    
    //Repeate
    let mut repeate: u64 = 0;
    let times = c/sequences.iter().skip(offset).map(|s| s.len() as u32).sum::<u32>();
    

    //After the repeate (if last does not go a full cycle)
    let mut remaining = c as usize % sequences.iter().skip(offset).map(|s| s.len()).sum::<usize>();
    eprintln!("{:?}, {:?}", c, remaining);
    let mut end: u32 = 0;

    

    for si in offset..sequences.len() {

        repeate += sequences[si].iter().sum::<u32>() as u64; //Repeated

        //Remaining
        if remaining >= sequences[si].len() {
            end += sequences[si].iter().sum::<u32>();
            remaining -= sequences[si].len(); 
        } else {
            for i in 0..remaining {
                end += sequences[si][i];
            }
            remaining = 0;
        }
    }

    eprintln!("{}, {} * {}, {}", init, repeate, times, end);
    println!("{}", init as u64 + (repeate * times as u64) + end as u64);
}

