use std::{
    fs::{self},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise14_1/testfile");
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
const X_MID: i32 = 50;
const Y_MID: i32 = 51;
const ITERS: i32 = 100;
fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();

    let mut quads: Vec<i32> = vec![0; 4];

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
        let robot = Robot::new(lv[0], lv[1], lv[2], lv[3]);
        let final_x = (robot.px + (ITERS * robot.vx)).rem_euclid(X_MAX);
        let final_y = (robot.py + (ITERS * robot.vy)).rem_euclid(Y_MAX);

        let quad = get_quad_index(final_x, final_y);
        match quad {
            Some(index) => {
                quads[index] += 1;
            }
            None => {}
        }
    }

    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    (quads[0] as i64) * (quads[1] as i64) * (quads[2] as i64) * (quads[3] as i64)
}

fn get_quad_index(px: i32, py: i32) -> Option<usize> {
    if px < X_MID {
        if py > Y_MID {
            return Some(2);
        }
        if py < Y_MID {
            return Some(0);
        }
    } else if px > X_MID {
        if py > Y_MID {
            return Some(3);
        }
        if py < Y_MID {
            return Some(1);
        }
    }
    None
}
