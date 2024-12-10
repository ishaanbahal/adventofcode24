use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise10_1/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let file = File::open(fname);
    let mut data: Vec<Vec<i8>> = Vec::new();
    let mut trail_start: Vec<(usize, usize)> = Vec::new(); // i, j == (row, col)
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d").unwrap();
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let mut row: Vec<i8> = Vec::new();
                        for item in re.find_iter(&l) {
                            let num = item.as_str().parse().unwrap();
                            row.push(num);
                            if num == 0 {
                                trail_start.push((data.len(), row.len() - 1));
                            }
                        }
                        data.push(row);
                    }
                    Err(e) => panic!("Error reading line: {}", e),
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    }
    get_trail_scores(&data, &trail_start)
}

fn navigate_trail(data: &Vec<Vec<i8>>, start_i: usize, start_j: usize) -> usize {
    let mut trail_id = 1;
    let mut points: Vec<(usize, usize)> = vec![(start_i, start_j)];
    let rows = data.len();
    let cols = data[0].len();
    while points.len() != 0 {
        if trail_id == 10 {
            break;
        }
        let mut new_points: Vec<(usize, usize)> = Vec::new();
        for (i, j) in points {
            if j as i32 - 1 >= 0 && data[i][j - 1] == trail_id {
                new_points.push((i, j - 1));
            }
            if j + 1 < cols && data[i][j + 1] == trail_id {
                new_points.push((i, j + 1));
            }
            if i as i32 - 1 >= 0 && data[i - 1][j] == trail_id {
                new_points.push((i - 1, j));
            }
            if i + 1 < rows && data[i + 1][j] == trail_id {
                new_points.push((i + 1, j));
            }
        }
        points = new_points;
        trail_id += 1;
    }
    HashSet::<String>::from_iter(
        points
            .iter()
            .map(|(i, j)| format!("{}_{}", i, j))
            .into_iter(),
    )
    .len()
}

fn get_trail_scores(data: &Vec<Vec<i8>>, trail_start: &Vec<(usize, usize)>) -> i32 {
    let mut score = 0;
    for (start_i, start_j) in trail_start {
        let trail_score = navigate_trail(data, *start_i, *start_j);
        score += trail_score;
    }
    score as i32
}
