use std::{
    collections::HashMap,
    fs::{self},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise11_2/testfile");
        println!("Total: {}", count);
    }
}

fn parse_input(fname: &str) -> i64 {
    let data: String = fs::read_to_string(fname).unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let stones: Vec<String> = re
        .find_iter(&data)
        .map(|x| x.as_str().to_string())
        .collect();
    calculate_blink_stones(&stones, 75)
}

fn insert_to_map(map: &mut HashMap<String, i64>, item: String, count: i64) {
    if map.contains_key(&item) {
        let prev_count = map.get(&item).unwrap();
        map.insert(item, count + prev_count);
    } else {
        map.insert(item, count);
    }
}

fn calculate_blink_stones(data: &Vec<String>, times: usize) -> i64 {
    let mut map: HashMap<String, i64> = HashMap::new();
    for item in data {
        insert_to_map(&mut map, item.clone(), 1);
    }
    for _ in 0..times {
        let mut local_map: HashMap<String, i64> = HashMap::new();
        for (item, count) in map {
            if item == "0" {
                insert_to_map(&mut local_map, "1".to_string(), count);
            } else if item.len() % 2 == 0 {
                let half = item.len() / 2;
                let mut s1 = (&item[..half]).trim_start_matches('0').to_string();
                let mut s2 = (&item[half..]).trim_start_matches('0').to_string();
                if s1 == "" {
                    s1 = "0".to_string();
                }
                if s2 == "" {
                    s2 = "0".to_string();
                }
                insert_to_map(&mut local_map, s1, count);
                insert_to_map(&mut local_map, s2, count);
            } else {
                let num: i64 = item.parse().unwrap();
                insert_to_map(&mut local_map, (num * 2024).to_string(), count);
            }
        }
        map = local_map;
    }
    let mut total: i64 = 0;
    for (_, count) in map {
        total += count as i64;
    }
    total
}
