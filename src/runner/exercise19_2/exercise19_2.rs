use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise19_2/testfile");
        println!("Total: {}", count);
    }
}

fn parse_input(fname: &str) -> i64 {
    let data: Vec<String> = fs::read_to_string(fname)
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
    let patterns: HashSet<String> = data[0]
        .replace(" ", "")
        .split(",")
        .map(|x| x.to_string())
        .collect();
    let max_pattern_size = patterns.iter().map(|x| x.len()).max().unwrap();

    let designs: Vec<String> = data[1].split("\n").map(|x| x.to_string()).collect();
    is_valid_design(&designs, &patterns, max_pattern_size)
}

fn is_valid_design(
    designs: &Vec<String>,
    patterns: &HashSet<String>,
    max_pattern_size: usize,
) -> i64 {
    let mut cache: HashMap<String, i64> = HashMap::new();
    let mut counter: i64 = 0;
    for design in designs {
        if design.is_empty() {
            continue;
        }
        let count = possible_patterns(
            design.chars().collect(),
            patterns,
            max_pattern_size,
            &mut cache,
        ) as i64;
        counter += count;
    }
    counter
}

fn possible_patterns(
    design: Vec<char>,
    patterns: &HashSet<String>,
    max_pattern_size: usize,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    let mut count = 0;
    let design_str: String = design.iter().collect();
    if cache.contains_key(&design_str) {
        return *cache.get(&design_str).unwrap();
    }
    for i in 0..design.len() {
        if i > max_pattern_size - 1 {
            break;
        }
        let p: String = design[0..i + 1].into_iter().collect();
        if patterns.contains(&p) {
            let d: String = design[i + 1..].into_iter().collect();
            if d.is_empty() {
                continue;
            }
            if d.len() < max_pattern_size && patterns.contains(&d) {
                count += 1;
            }
            if cache.contains_key(&d) {
                count += *cache.get(&d).unwrap();
                continue;
            }
            let c = possible_patterns(design[i + 1..].to_vec(), patterns, max_pattern_size, cache);
            count += c;
        }
    }
    cache.insert(design_str, count);
    count
}
