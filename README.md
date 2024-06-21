# Codingame

My [Codingame](https://www.codingame.com) solutions

## Parsing Macros for Rust (🦀)

### Normal

```rust
use std::{fs::File, io::Read, os::unix::prelude::FromRawFd};
macro_rules! r { // Reads all
    ($l:expr) => {
        let mut input = String::new();
        let mut stdin = unsafe { File::from_raw_fd(0) };
        stdin.read_to_string(&mut input).unwrap();
        $l = input.split("\n");
    };
}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

fn main() {
    let mut l; r!(l);
```

### Loop

```rust
use std::io;
macro_rules! rs { // Reads a single line
    ($l:expr) => {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        $l = s.split("\n"); //Dirty way to turn into an iter...
    };
}

```
