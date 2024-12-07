use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise7_2/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i64 {
    let file = File::open(fname);
    let mut correct_total: i64 = 0;
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d+").unwrap();
            for line in reader.lines() {
                let l = line.unwrap();
                if l.is_empty() {
                    continue;
                }
                let mut parsed_input = re.find_iter(&l);
                let total = parsed_input
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap();
                let nums: Vec<i64> = parsed_input
                    .map(|x| x.as_str().parse::<i64>().unwrap())
                    .collect();
                //if check_eq_bfs(total, &nums) {
                //    correct_total += total;
                //}
                if check_eq_dfs(total, &nums, 1, nums[0]) {
                    correct_total += total;
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    }
    return correct_total;
}

fn concat_i64(a: i64, b: i64) -> i64 {
    let digits = (b as f64).log10().ceil() as u32;
    let c = (a * i64::pow(10, digits)) + b;
    return c;
}

fn check_eq_dfs(total: i64, nums: &Vec<i64>, index: usize, seed: i64) -> bool {
    if index == nums.len() {
        return seed == total;
    }
    let num = nums[index];
    let mul = seed * num;
    let add = seed + num;
    let cc = concat_i64(seed, num);
    let next_index = index + 1;
    if mul <= total && check_eq_dfs(total, &nums, next_index, mul) {
        return true;
    }
    if add <= total && check_eq_dfs(total, &nums, next_index, add) {
        return true;
    }
    if cc <= total && check_eq_dfs(total, &nums, next_index, cc) {
        return true;
    }
    return false;
}

fn check_eq_bfs(total: i64, nums: &Vec<i64>) -> bool {
    let mut results: Vec<i64> = vec![nums[0]];
    for index in 1..nums.len() {
        let mut level_results: Vec<i64> = vec![];
        while results.len() > 0 {
            let r = results.pop().unwrap();
            let mul = r * nums[index];
            let add = r + nums[index];
            let cc = concat_i64(r, nums[index]);
            if index == nums.len() - 1 && (mul == total || add == total || cc == total) {
                return true;
            }
            if mul <= total {
                level_results.push(mul);
            }
            if add <= total {
                level_results.push(add);
            }
            if cc <= total {
                level_results.push(cc);
            }
        }
        results = level_results;
    }
    return false;
}
