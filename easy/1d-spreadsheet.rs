use std::{io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

enum Value {
    Number(i32),
    Reference(usize),
    Nil,
}

enum Operation {
    Value, Add, Sub, Mult,
}

struct Cell{
    operation: Operation,
    a: Value,
    b: Value,
    value: std::cell::Cell<Option<i32>>,
}

fn parse_arg(a: &String) -> Value {
    if a == "_" { Value::Nil}
    else if a.contains("$") { Value::Reference(a[1..].parse::<usize>().unwrap())}
    else {Value::Number(a.parse::<i32>().unwrap())}
}

impl Cell {
    fn new(operation: &String, a: &String, b: &String) -> Cell
    {
        let pa = parse_arg(&a);
        let pb = parse_arg(&b);
        let op = match operation.as_str() {
            "VALUE" => { Operation::Value },
            "ADD" =>   { Operation::Add   },
            "SUB" =>   { Operation::Sub   },
            "MULT" =>  { Operation::Mult  },
            _ => panic!()
        };
        Cell{ operation: op, a: pa, b: pb, value: std::cell::Cell::new(Option::None)}
    }

    fn get_value(&self, cells: &Vec<Cell>) -> i32 {
        let val = match self.operation {
            Operation::Value => { self.a.get_value(cells)                                     },
            Operation::Add =>   { self.a.get_value(cells) + self.b.get_value(cells)},
            Operation::Sub =>   { self.a.get_value(cells) - self.b.get_value(cells)},
            Operation::Mult =>  { self.a.get_value(cells) * self.b.get_value(cells)},
        };
        self.value.set(Some(val));
        val
    }
}

impl Value {
    fn get_value(&self, cells: &Vec<Cell>) -> i32
    {
        match *self {
            Value::Number(num) => num,
            Value::Reference(i) => {cells[i].get_value(cells)},
            Value::Nil => panic!(),
        }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut cells = Vec::<Cell>::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let operation = inputs[0].trim().to_string();
        let arg_1 = inputs[1].trim().to_string();
        let arg_2 = inputs[2].trim().to_string();

        let cell = Cell::new(&operation, &arg_1, &arg_2);
        cells.push(cell);
    }

    //let (mut hit, mut mis) = (0, 0);
    for c in cells.iter() {
        println!("{:?}", match c.value.get() {
            Some(val) => {/*hit+=1;*/ val},
            None => {/*mis+=1;*/ c.get_value(&cells)},
        });
    }
    //eprintln!("hit: {}, mis: {}", hit, mis)
}

