use std::{io::{self}, time::SystemTime};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn next_input() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

fn main() {
    
    let now = SystemTime::now();
    
    let input = next_input();
    let inputs = input.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], u32);
    let mut c = parse_input!(inputs[1], u32);
    let n = parse_input!(inputs[2], usize);
    let mut queue = Vec::<u32>::new();
    for _ in 0..n { queue.push(parse_input!(next_input(), u32));}

    eprintln!("Read input: {:?}", now.elapsed().unwrap().as_micros());

    let now = SystemTime::now();

    let mut sequences = Vec::<Vec<u32>>::new(); // Save all possible sequences til it repeates itself (so we trade memory for time gain)
    let mut sequence = Vec::<u32>::new();
    let mut overflow = false;
    let mut i: usize = 0; // Where are we at atm in the queue
    loop {
        let one_seat_for_one_person = i;
        let mut ride_group: u32 = 0;
        loop {
            ride_group += queue[i];
            i+=1; if i==queue.len() { i=0; overflow=true; }
            if one_seat_for_one_person == i { break; } // Occures when there are less people than the capacity of the ride
            if ride_group + queue[i] > l { break; } // Next group cannot fit
        }
        sequence.push(ride_group);
        if overflow {
            overflow = false;
            sequences.push(sequence.clone());
            sequence.clear();
            if sequences.len() > 1 && sequences[sequences.len()-1] == sequences[sequences.len()-2] { break; } // Found a repeate
        }
    }
    //eprintln!("{:?}", sequences);

    eprintln!("Sequence hunt: {:?}", now.elapsed().unwrap().as_micros());

    let now = SystemTime::now();

    //Before the repeate
    let mut init: u32 = 0;
    for si in 0..sequences.len()-2 {
        init += sequences[si].iter().sum::<u32>();
        c -= sequences[si].len() as u32;
    }
    
    //Repeate
    let rseq = sequences.last().unwrap();
    let repeate: u64 = 
        (rseq.iter().sum::<u32>() as u64) *
        ((c as u64)/(rseq.len() as u64));
    
    //After the repeate (if last does not go a full cycle)
    let remaining = c as usize % rseq.len();
    let mut end: u32 = 0;
    for i in 0..(remaining) {
        end += rseq[i];
    }

    eprintln!("Cacl: {:?}", now.elapsed().unwrap().as_micros());

    //eprintln!("{} {} {}", init, repeate, end);
    println!("{}", init as u64 + repeate + end as u64);
}

