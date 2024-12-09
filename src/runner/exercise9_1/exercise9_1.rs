use std::fs::{self};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise9_1/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i64 {
    let data = fs::read_to_string(fname).unwrap();
    let re = Regex::new(r"\d").unwrap();
    let mut parsed_data: Vec<i32> = Vec::new();
    let mut is_free_space = false;
    let mut id: i32 = 0;
    re.find_iter(data.as_str()).for_each(|x| {
        let num = x.as_str().parse::<i32>().unwrap();
        if !is_free_space {
            is_free_space = true;
            for _ in 0..num {
                parsed_data.push(id);
            }
            id += 1;
        } else {
            is_free_space = false;
            for _ in 0..num {
                parsed_data.push(-1);
            }
        }
    });
    checksum(&defrag(&parsed_data))
}

fn defrag(data: &Vec<i32>) -> Vec<i32> {
    let mut defragged_storage: Vec<i32> = Vec::new();
    let mut flattened_data: Vec<i32> = Vec::new();
    data.iter().for_each(|num| {
        if *num != -1 {
            flattened_data.push(num.clone());
        }
    });
    let mut j = flattened_data.len() - 1;
    data.iter().for_each(|num| {
        if *num == -1 {
            defragged_storage.push(flattened_data[j]);
            j -= 1;
        } else {
            defragged_storage.push(num.clone());
        }
    });
    defragged_storage[0..flattened_data.len()].to_vec()
}

fn checksum(data: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    data.iter().enumerate().for_each(|(index, num)| {
        sum += index as i64 * (*num) as i64;
    });
    sum
}

fn print_storage_state(data: &Vec<i32>) {
    data.iter().for_each(|num| {
        if *num == -1 {
            print!(".");
            return;
        }
        print!("{}", num);
    });
    print!("\n");
}
