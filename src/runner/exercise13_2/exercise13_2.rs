use std::{
    fs::{self},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise13_1/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug)]
struct Config {
    a1: i64,
    a2: i64,
    b1: i64,
    b2: i64,
    c1: i64,
    c2: i64,
}

impl Config {
    fn new() -> Config {
        Config {
            a1: 0,
            a2: 0,
            b1: 0,
            b2: 0,
            c1: 0,
            c2: 0,
        }
    }
}

fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();

    let mut total_count = 0;
    let data: Vec<String> = fs::read_to_string(fname)
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    let re = Regex::new(r"\d+").unwrap();
    for item in data {
        let formula: Vec<String> = item.split("\n").map(|x| x.to_string()).collect();
        let mut config = Config::new();
        let mut a = re.find_iter(formula[0].as_str());
        (config.a1, config.a2) = (
            a.nth(0).unwrap().as_str().parse().unwrap(),
            a.nth(0).unwrap().as_str().parse().unwrap(),
        );

        let mut b = re.find_iter(formula[1].as_str());
        (config.b1, config.b2) = (
            b.nth(0).unwrap().as_str().parse().unwrap(),
            b.nth(0).unwrap().as_str().parse().unwrap(),
        );

        let mut c = re.find_iter(formula[2].as_str());
        (config.c1, config.c2) = (
            c.nth(0).unwrap().as_str().parse().unwrap(),
            c.nth(0).unwrap().as_str().parse().unwrap(),
        );
        config.c1 = 10000000000000 + config.c1;
        config.c2 = 10000000000000 + config.c2;
        let (t1, t2) = get_factors(&config);
        if (config.a1 * t1 + config.b1 * t2 == config.c1)
            && (config.a2 * t1 + config.b2 * t2 == config.c2)
        {
            total_count += (3 * t1) + t2;
        }
    }

    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    total_count
}

fn get_factors(config: &Config) -> (i64, i64) {
    let t2 = ((config.a2 * config.c1) - (config.a1 * config.c2))
        / ((config.a2 * config.b1) - (config.a1 * config.b2));
    let t1 = (config.c1 - (config.b1 * t2)) / config.a1;
    (t1, t2)
}
