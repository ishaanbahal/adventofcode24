use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise5_1/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let file = File::open(fname);
    let mut sequence_parse_start = false;
    let mut ruleset: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut page_sequences: Vec<Vec<i32>> = Vec::new();
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let l = line.unwrap();

                // if true, then parse for sequences
                if l.is_empty() {
                    sequence_parse_start = true;
                    continue;
                }
                if !sequence_parse_start {
                    let mut rule = l.split("|");
                    let lhs: i32 = rule.next().unwrap().parse().unwrap();
                    let rhs: i32 = rule.next().unwrap().parse().unwrap();
                    if ruleset.contains_key(&lhs) {
                        let seq = ruleset.get_mut(&lhs).unwrap();
                        seq.push(rhs);
                    } else {
                        ruleset.insert(lhs, vec![rhs]);
                    }
                } else {
                    let seq = l.split(",");
                    let mut sequence: Vec<i32> = Vec::new();
                    seq.for_each(|s| {
                        sequence.push(s.parse().unwrap());
                    });
                    page_sequences.push(sequence);
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    };
    return validate_sequences(page_sequences, ruleset);
}

fn is_page_valid(prev: Vec<i32>, page: i32, ruleset: &HashMap<i32, Vec<i32>>) -> bool {
    if !ruleset.contains_key(&page) {
        return true;
    }
    let rule = ruleset.get(&page).unwrap();
    for previous_page in prev {
        if rule.contains(&previous_page) {
            return false;
        }
    }
    return true;
}

fn validate_sequences(sequences: Vec<Vec<i32>>, ruleset: HashMap<i32, Vec<i32>>) -> i32 {
    let mut sequence_mid_total: i32 = 0;
    for seq in sequences {
        let mut is_valid_seq: bool = true;
        for page_index in 0..seq.len() {
            if page_index == 0 {
                continue;
            }
            let prev_seq: Vec<i32> = seq[0..page_index].to_vec();
            let page = seq[page_index];
            if !is_page_valid(prev_seq, page, &ruleset) {
                is_valid_seq = false;
                break;
            }
        }
        if is_valid_seq {
            let seq_mid_point: usize = (seq.len() - 1) / 2;
            sequence_mid_total += seq[seq_mid_point];
        }
    }
    sequence_mid_total
}
