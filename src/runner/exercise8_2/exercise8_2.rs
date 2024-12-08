use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise8_2/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let file = File::open(fname);
    let mut parsed_data: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut all_antinodes: HashSet<(usize, usize)> = HashSet::new();
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"[a-zA-Z0-9]{1}").unwrap();
            let mut i: usize = 0;
            let mut total_cols = 0;
            for line in reader.lines() {
                let l = line.unwrap();
                if l.is_empty() {
                    continue;
                }
                total_cols = l.chars().count();
                for (j, ch) in l.chars().enumerate() {
                    if !re.is_match(&ch.to_string()) {
                        continue;
                    }
                    match parsed_data.get(&ch) {
                        Some(data) => {
                            let mut data = data.clone();
                            data.push((j, i));
                            parsed_data.insert(ch.clone(), data);
                        }
                        None => {
                            parsed_data.insert(ch.clone(), vec![(j, i)]);
                        }
                    }
                }
                i += 1;
            }
            let total_rows = i as i32;

            for (_, ls) in &parsed_data {
                if ls.len() < 2 {
                    continue;
                }
                for (i, p1) in ls.iter().enumerate() {
                    for j in (i + 1)..ls.len() {
                        let p2 = &ls[j];
                        match find_antinode(p1.clone(), p2.clone(), total_rows, total_cols as i32) {
                            Some(antinodes) => {
                                for node in antinodes {
                                    all_antinodes.insert(node);
                                }
                            }
                            None => {}
                        }
                    }
                }
                for node in ls {
                    all_antinodes.insert(node.clone());
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    }
    all_antinodes.len() as i32
}

fn find_antinode(
    a: (usize, usize),
    b: (usize, usize),
    total_rows: i32,
    total_cols: i32,
) -> Option<Vec<(usize, usize)>> {
    let x_diff = a.0 as i32 - b.0 as i32;
    let y_diff = a.1 as i32 - b.1 as i32;
    let mut points: Vec<(usize, usize)> = vec![];

    let mut lc = 1;
    loop {
        let p1 = (a.0 as i32 + lc * x_diff, a.1 as i32 + lc * y_diff);
        if p1.0 < 0 || p1.1 < 0 || p1.0 >= total_cols || p1.1 >= total_rows {
            break;
        }
        points.push((p1.0 as usize, p1.1 as usize));
        lc += 1;
    }

    lc = 1;

    loop {
        let p2 = (b.0 as i32 - lc * x_diff, b.1 as i32 - lc * y_diff);
        if p2.0 < 0 || p2.1 < 0 || p2.0 >= total_cols || p2.1 >= total_rows {
            break;
        }
        points.push((p2.0 as usize, p2.1 as usize));
        lc += 1;
    }

    if points.len() != 0 {
        return Some(points);
    }
    None
}
