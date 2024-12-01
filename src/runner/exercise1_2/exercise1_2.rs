use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let (left_list, right_map) = get_heaps("runner/exercise1_2/testfile");
        let distance = total_distance(left_list, right_map);
        println!("Total distance: {}", distance);
    }
}

fn get_heaps(fname: &str) -> (Vec<i64>, HashMap<i64, i64>) {
    let file = File::open(fname);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d+").unwrap();
            let mut left_list: Vec<i64> = Vec::new();
            let mut right_map: HashMap<i64, i64> = HashMap::new();
            for line in reader.lines() {
                let l = line.unwrap();
                let mut iter = re.find_iter(&l);
                let left_match = iter.next().unwrap();
                let left_num: i64 = left_match.as_str().parse().unwrap();
                left_list.push(left_num);
                let right_match = iter.next().unwrap();
                let right_num: i64 = right_match.as_str().parse().unwrap();
                if right_map.contains_key(&right_num) {
                    let count = right_map.get(&right_num).unwrap();
                    right_map.insert(right_num, count + 1);
                } else {
                    right_map.insert(right_num, 1);
                }
            }
            return (left_list, right_map);
        }
        Err(e) => panic!("Error opening file: {}", e),
    };
}

fn total_distance(left_list: Vec<i64>, right_map: HashMap<i64, i64>) -> i64 {
    let mut total_distance: i64 = 0;
    for left_num in left_list {
        if right_map.contains_key(&left_num) {
            let count = right_map.get(&left_num).unwrap();
            total_distance += left_num * count;
        }
    }
    return total_distance;
}
