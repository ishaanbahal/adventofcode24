use std::{
    cmp::{Ordering, Reverse},
    fs::File,
    i64,
    io::{BufRead, BufReader},
    thread::sleep,
    time::{Duration, SystemTime},
};

use priority_queue::PriorityQueue;
use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise18_1/testfile");
        println!("Total: {}", count);
    }
}

const GRID_X: usize = 71;
const GRID_Y: usize = 71;
const MAX_STEPS: usize = 1024;
fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();
    let file = File::open(fname);
    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_X]; GRID_Y];
    let mut read_counter = 0;
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"\d+").unwrap();
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        read_counter += 1;
                        let coordinates: Vec<&str> = re.find_iter(&l).map(|m| m.as_str()).collect();
                        let x: usize = coordinates[0].parse().unwrap();
                        let y: usize = coordinates[1].parse().unwrap();
                        grid[y][x] = 1;
                        if read_counter == MAX_STEPS {
                            println!("Max steps reached");
                            break;
                        }
                    }
                    Err(e) => panic!("Error reading line: {}", e),
                }
            }
        }
        Err(e) => panic!("Error opening file: {}", e),
    }
    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    shortest_path(&grid, (0, 0))
}

fn get_next_steps(location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 {
        steps.push((location.0, location.1 - 1));
    }
    if location.1 < GRID_X - 1 {
        steps.push((location.0, location.1 + 1));
    }
    if location.0 > 0 {
        steps.push((location.0 - 1, location.1));
    }
    if location.0 < GRID_Y - 1 {
        steps.push((location.0 + 1, location.1));
    }
    steps
}

fn get_displacement(location: (usize, usize)) -> i64 {
    return (((GRID_X as i64 - location.1 as i64).pow(2)
        + (GRID_Y as i64 - location.0 as i64).pow(2)) as f64)
        .sqrt() as i64
        + 1;
}

fn shortest_path(grid: &Vec<Vec<usize>>, start: (usize, usize)) -> i64 {
    let mut costs: Vec<Vec<i64>> = vec![vec![-1; GRID_X]; GRID_Y];
    let mut to_visit = PriorityQueue::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; GRID_X]; GRID_Y];
    costs[start.0][start.1] = 0;
    to_visit.push(start, Reverse(0));
    while !to_visit.is_empty() {
        let current = to_visit.pop().unwrap().0;
        if current == (GRID_Y - 1, GRID_X - 1) {
            return costs[current.0][current.1];
        }
        visited[current.0][current.1] = true;
        let next_steps = get_next_steps(current);
        for step in next_steps {
            if step.0 <= GRID_Y - 1
                && step.1 <= GRID_X - 1
                && grid[step.0][step.1] != 1
                && !visited[step.0][step.1]
            {
                costs[step.0][step.1] = costs[current.0][current.1] + 1;
                let hx = costs[step.0][step.1] + get_displacement(step);
                to_visit.push(step, Reverse(hx));
            }
        }
    }
    0
}
