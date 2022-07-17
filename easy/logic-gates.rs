use std::{collections::HashMap, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn next_input() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

struct Signal {
    name: String,
    signal: Vec<bool>,
}

impl Signal {
    fn new_iput_singal() -> Signal {
        let input = next_input();
        let inputs = input.split(" ").collect::<Vec<_>>();
        Signal {
            name: inputs[0].trim().to_string(),
            signal: inputs[1]
                .trim()
                .chars()
                .map(|c| match c {
                    '_' => false,
                    '-' => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
        }
    }
    fn new_output_signal(signals: &HashMap<String, Signal>) -> Signal {
        let input = next_input();
        let inputs = input.split(" ").collect::<Vec<_>>();
        let output_name = inputs[0].trim().to_string();
        let gate = inputs[1].trim();
        let a_input_name = inputs[2].trim();
        let b_input_name = inputs[3].trim();

        let mut output_signal = Vec::<bool>::new();
        let a_sig = &signals.get(a_input_name).unwrap().signal;
        let b_sig = &signals.get(b_input_name).unwrap().signal;
        for (i, a) in a_sig.iter().enumerate() {
            let a = *a;
            let b = b_sig[i];
            output_signal.push(match gate {
                "AND" => a & b,
                "OR" => a | b,
                "XOR" => a != b,
                "NAND" => !(a & b),
                "NOR" => !(a | b),
                "NXOR" => a == b,
                _ => unreachable!(),
            });
        }
        Signal {
            name: output_name,
            signal: output_signal,
        }
    }
    fn print(&self) -> String {
        format!(
            "{} {}",
            self.name,
            self.signal
                .iter()
                .map(|c| match c {
                    true => '-',
                    false => '_',
                })
                .collect::<String>()
        )
    }
}

fn main() {
    let n = parse_input!(next_input(), u8);
    let m = parse_input!(next_input(), u8);
    let mut signals = HashMap::<String, Signal>::new();
    for _ in 0..n as usize {
        let signal = Signal::new_iput_singal();
        signals.insert(signal.name.clone(), signal);
    }
    for _ in 0..m as usize {
        let signal = Signal::new_output_signal(&signals);
        println!("{}", signal.print());
    }
}
