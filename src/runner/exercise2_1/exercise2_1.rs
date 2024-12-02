use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input_per_line("runner/exercise2_1/testfile");
        println!("Total safe: {}", safe_count);
    }
}

fn parse_input_per_line(fname: &str) -> i16 {
    let file = File::open(fname);
    let mut safe_counter: i16 = 0;
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d+").unwrap();
            for line in reader.lines() {
                let l = line.unwrap();
                let iter = re.find_iter(&l);
                let mut data: Vec<i32> = Vec::new();
                iter.for_each(|m| {
                    let num: i32 = m.as_str().parse().unwrap();
                    data.push(num);
                });
                if is_safe(data) {
                    safe_counter += 1;
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    };
    return safe_counter;
}

fn is_safe(data: Vec<i32>) -> bool {
    if data.len() == 1 {
        return true;
    }
    let inc: bool;
    if data[0] < data[1] {
        inc = true;
    } else if data[0] > data[1] {
        inc = false;
    } else {
        return false;
    }
    for i in 0..data.len() - 1 {
        let mut diff: i32 = data[i + 1] - data[i];
        if diff == 0 {
            return false;
        }
        if !inc {
            diff *= -1;
        }
        if diff > 3 || diff < 0 {
            return false;
        }
    }

    return true;
}
