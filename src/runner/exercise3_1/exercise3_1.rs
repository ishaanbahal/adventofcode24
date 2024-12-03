use std::fs::{self};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise3_1/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let mut data = fs::read_to_string(fname).unwrap();
    let mut total: i32 = 0;
    data = data.replace("\n", "");
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_formulae = Regex::new(r"\d{1,3}").unwrap();
    for formulae in re.find_iter(&data) {
        let f = formulae.as_str();
        let mut nums = re_formulae.find_iter(f);
        let left_num: i32 = nums.next().unwrap().as_str().parse().unwrap();
        let right_num: i32 = nums.next().unwrap().as_str().parse().unwrap();
        total += left_num * right_num;
    }
    return total;
}
