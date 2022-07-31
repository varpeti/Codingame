use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

#[derive(Debug)]
struct Pos {
    x: i16,
    y: i16,
}

fn main() {
    // Input
    let mut buf = BufReader::new(stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap().to_string();

    let inputs = read_line()
        .split(" ")
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i16>>();
    let w = inputs[0];
    let h = inputs[1];
    let d = (inputs[4] - inputs[3]) as f32 / (inputs[3] - inputs[2]) as f32; // Delta
    eprintln!("{}", d);
    let mut asteroids = HashMap::<char, [Option<Pos>; 2]>::new();
    for y in 0..h {
        let line = read_line();
        let mut p = line
            .split(" ")
            .map(|p| p.trim().chars())
            .collect::<Vec<_>>();
        for i in 0..p.len() {
            for x in 0..w {
                if let Some(c) = p[i].next() {
                    if !c.is_ascii_uppercase() {
                        continue;
                    }
                    match asteroids.get_mut(&c) {
                        None => {
                            let mut ver = [None, None];
                            ver[i] = Some(Pos { x, y });
                            asteroids.insert(c, ver);
                        }
                        Some(v) => v[i] = Some(Pos { x, y }),
                    };
                }
            }
        }
    }
    // Calculate
    let mut canvas = vec![vec!['.'; w as usize]; h as usize];
    for (name, ver) in asteroids {
        let pas = Pos {
            x: ver[0].as_ref().unwrap().x,
            y: ver[0].as_ref().unwrap().y,
        };
        let cur = Pos {
            x: ver[1].as_ref().unwrap().x,
            y: ver[1].as_ref().unwrap().y,
        };
        let fut = Pos {
            x: ((cur.x - pas.x) as f32 * d).floor() as i16 + cur.x,
            y: ((cur.y - pas.y) as f32 * d).floor() as i16 + cur.y,
        };
        eprintln!("{}, {:?}, {:?}, {:?}", name, pas, cur, fut);
        if fut.y >= h || fut.y < 0 || fut.x >= w || fut.x < 0 {
            continue;
        }
        if canvas[fut.y as usize][fut.x as usize] == '.'
            || canvas[fut.y as usize][fut.x as usize] > name
        {
            canvas[fut.y as usize][fut.x as usize] = name;
        }
    }
    // Draw
    for line in canvas {
        println!("{}", line.iter().collect::<String>());
    }
}
