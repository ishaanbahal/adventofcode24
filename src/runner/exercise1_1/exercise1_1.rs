use regex::Regex;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let (left_list, right_list) = get_heaps("runner/exercise1_1/testfile");
        let distance = total_distance(&mut left_list.clone(), &mut right_list.clone());
        println!("Total distance: {}", distance);
    }
}

fn get_heaps(fname: &str) -> (BinaryHeap<Reverse<i64>>, BinaryHeap<Reverse<i64>>) {
    let file = File::open(fname);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d+").unwrap();
            let mut left_list: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            let mut right_list: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            for line in reader.lines() {
                let l = line.unwrap();
                let mut iter = re.find_iter(&l);
                let left_match = iter.next().unwrap();
                let left_num: i64 = left_match.as_str().parse().unwrap();
                let right_match = iter.next().unwrap();
                let right_num: i64 = right_match.as_str().parse().unwrap();
                left_list.push(Reverse(left_num));
                right_list.push(Reverse(right_num));
            }
            return (left_list, right_list);
        }
        Err(e) => panic!("Error opening file: {}", e),
    };
}

fn total_distance(
    left_list: &mut BinaryHeap<Reverse<i64>>,
    right_list: &mut BinaryHeap<Reverse<i64>>,
) -> i64 {
    let mut total_distance: i64 = 0;
    loop {
        if left_list.len() == 0 || right_list.len() == 0 {
            break;
        }
        let left = left_list.pop().unwrap().0;
        let right = right_list.pop().unwrap().0;
        total_distance += (left - right).abs();
    }
    return total_distance;
}
