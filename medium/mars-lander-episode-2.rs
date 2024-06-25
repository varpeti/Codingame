use std::io;
macro_rules! rs {
    // Reads a single line
    ($l:expr) => {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        $l = s.split("\n"); //Dirty way to turn into an iter...
    };
}

macro_rules! p{
    ($x:expr)=>($x.next().unwrap().trim());
    ($x:expr,$($t:ty),*)=>{($(p!($x).parse::<$t>().unwrap()),*)};
    ($x:expr,$s:expr)=>(p!($x).split($s));
}

const GRAVITY: f64 = -3.711;
const MAX_X_SPEED: isize = 20;
const MAX_Y_SPEED: isize = 40;
const SAFETY_CATCH: isize = 10;
const SAFE_DISTANCE: isize = 50;

type Land = (isize, isize, isize);

fn main() {
    let mut l;
    rs!(l);
    let surface_n = p!(l, usize);
    let (mut px, mut py) = (0, 0);
    let mut land: Land = (0, 0, 0);
    for _ in 0..surface_n {
        let mut l;
        rs!(l);
        let mut point = p!(l, " ");
        let (x, y) = p!(point, isize, isize);
        if y == py {
            land = (px, x, y);
        }
        (px, py) = (x, y);
    }
    eprintln!("{:?}", land);

    loop {
        let mut l;
        rs!(l);
        let mut data = p!(l, " ");
        let (x, y, x_speed, y_speed, _fuel, mut rotation, mut power) =
            p!(data, isize, isize, isize, isize, isize, isize, isize);

        if is_over_landing_ground(&land, x) {
            eprintln!("Above landing ground");
            if is_able_to_land(&land, y) {
                eprintln!("Able to land");
                if is_speed_within_limit(x_speed, y_speed) {
                    eprintln!("Speed OK");
                    rotation = 0;
                    power = 3;
                } else {
                    eprintln!("Speed NOK");
                    rotation = 0;
                    power = 4;
                }
            } else if is_speed_within_limit(x_speed, y_speed) {
                eprintln!("Unable to land & Speed OK");
                rotation = 0;
                power = 0;
            } else {
                eprintln!("Unable to land & Speed NOK");
                rotation = calculate_angle_swap(x_speed, y_speed);
                power = 4;
            }
        } else {
            eprintln!("Not above landing ground");
            if is_wrong_direction(&land, x, x_speed) || is_too_fast(x_speed) {
                eprintln!("Wrong dir | Too fast");
                rotation = calculate_angle_swap(x_speed, y_speed);
                power = 4;
            } else if is_too_slow(x_speed) {
                eprintln!("Too Slow");
                rotation = calculate_angle_x(&land, x);
                power = 4;
            } else {
                eprintln!("OK?");
                rotation = 0;
                power = calculate_thrust(y_speed);
            }
        }
        println!("{} {}", rotation, power);
    }
}

fn is_over_landing_ground(land: &Land, x: isize) -> bool {
    x >= land.0 && x <= land.1
}

fn is_able_to_land(land: &Land, y: isize) -> bool {
    y < land.2 + SAFE_DISTANCE
}

fn is_speed_within_limit(x_speed: isize, y_speed: isize) -> bool {
    x_speed.abs() <= (MAX_X_SPEED - SAFETY_CATCH) && y_speed.abs() <= (MAX_Y_SPEED - SAFETY_CATCH)
}

fn is_wrong_direction(land: &Land, x: isize, x_speed: isize) -> bool {
    (x < land.0 && x_speed < 0) || (x > land.1 && x_speed > 0)
}

fn is_too_fast(y_speed: isize) -> bool {
    y_speed.abs() > (MAX_X_SPEED * 4)
}

fn is_too_slow(y_speed: isize) -> bool {
    y_speed.abs() < (MAX_X_SPEED * 2)
}

fn rad_2_deg(rad: f64) -> isize {
    (rad * (180.0 / std::f64::consts::PI)).round() as isize
}

fn calculate_angle_swap(x_speed: isize, y_speed: isize) -> isize {
    let speed = ((x_speed * x_speed + y_speed * y_speed) as f64).sqrt();
    let angle = x_speed as f64 / speed;
    rad_2_deg(angle)
}

fn calculate_angle_x(land: &Land, x: isize) -> isize {
    if x < land.0 {
        -rad_2_deg((-GRAVITY / 4.0).acos())
    } else if x > land.1 {
        rad_2_deg((-GRAVITY / 4.0).acos())
    } else {
        0
    }
}

fn calculate_thrust(y_speed: isize) -> isize {
    match y_speed >= 0 {
        true => 3,
        false => 4,
    }
}
