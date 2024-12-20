use std::{collections::HashSet, fs};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise19_1/testfile");
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
    let mut counter: i64 = 0;
    for design in designs {
        if is_valid(design.chars().collect(), patterns, max_pattern_size) {
            counter += 1;
        }
    }
    counter
}

fn is_valid(design: Vec<char>, patterns: &HashSet<String>, max_pattern_size: usize) -> bool {
    for i in 0..design.len() {
        if i > max_pattern_size - 1 {
            return false;
        }
        let p: String = design[0..i + 1].into_iter().collect();
        if patterns.contains(&p) {
            let d: String = design[i + 1..].into_iter().collect();
            if d.len() < max_pattern_size && patterns.contains(&d) {
                return true;
            }
            if is_valid(design[i + 1..].to_vec(), patterns, max_pattern_size) {
                return true;
            }
        }
    }
    false
}
