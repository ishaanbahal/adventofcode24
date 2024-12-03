use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let total = parse_input("runner/exercise3_2/testfile");
        println!("Total: {}", total);
    }
}

fn parse_input(fname: &str) -> i64 {
    let mut data = fs::read_to_string(fname).unwrap();
    data = "do()".to_string() + &data;
    let mut total: i64 = 0;
    let mut enabled = true;
    data = data.replace("\n", "");

    let re_command = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let re_formulae = Regex::new(r"\d{1,3}").unwrap();
    for seq in re_command.find_iter(&data) {
        let sequence = seq.as_str();
        match sequence {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if !enabled {
                    continue;
                }
                let mut nums = re_formulae.find_iter(sequence);
                let left_num: i32 = nums.next().unwrap().as_str().parse().unwrap();
                let right_num: i32 = nums.next().unwrap().as_str().parse().unwrap();
                total += i64::from(left_num * right_num);
            }
        }
    }
    return total;
}
