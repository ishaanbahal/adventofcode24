use std::{
    collections::HashSet,
    fs::{self},
    io::{self, Read},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise14_2/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(px: i32, py: i32, vx: i32, vy: i32) -> Robot {
        Robot { px, py, vx, vy }
    }
}

const X_MAX: i32 = 101;
const Y_MAX: i32 = 103;
fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();

    let mut robots: Vec<Robot> = vec![];

    let data: Vec<String> = fs::read_to_string(fname)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let re = Regex::new(r"(-){0,1}\d+").unwrap();
    for item in data {
        let lv: Vec<i32> = re
            .find_iter(&item)
            .map(|x| x.as_str().parse().unwrap())
            .collect();
        if lv.len() < 4 {
            continue;
        }
        robots.push(Robot::new(lv[0], lv[1], lv[2], lv[3]));
    }
    let mut iter = 0;
    loop {
        iter += 1;
        let was_printed = print_iter(&robots, iter);
        if was_printed {
            let mut buffer = [0u8; 1];
            match io::stdin().read_exact(&mut buffer) {
                Ok(_) => {
                    println!("\x1B[2J");
                    if buffer[0] == b'q' {
                        break;
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            }
        }
    }

    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    0
}

fn print_iter(robots: &Vec<Robot>, iter: i32) -> bool {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    for robot in robots {
        let final_x = (robot.px + (iter * robot.vx)).rem_euclid(X_MAX);
        let final_y = (robot.py + (iter * robot.vy)).rem_euclid(Y_MAX);
        if points.contains(&(final_x, final_y)) {
            return false;
        }
        points.insert((final_x, final_y));
    }
    println!("Iteration: {}", iter);
    for y in 0..Y_MAX {
        for x in 0..X_MAX {
            if points.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
    true
}
