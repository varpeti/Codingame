use std::io;
use regex::Regex;


fn main() {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    let t = input_line.trim_matches('\n');
    let re = Regex::new(r"^(\d*?)(.|sp|bS|sQ)$").unwrap();
    let mut ret = String::new();
    for w in t.split(" ")
    {
        if w == "nl" { ret += "\n"; continue; }
        let m = re.captures(w).unwrap();
        let num = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let c = match m.get(2).unwrap().as_str() {
            "sp" => {" "},
            "bS" => {"\\"},
            "sQ" => {"'"},
            "nl" => {"\n"},
            x => {x},
        };
        ret += &c.repeat(num);
    }
    println!("{}", ret);
}
