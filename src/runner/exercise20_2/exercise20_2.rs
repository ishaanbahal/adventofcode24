use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise20_2/testfile");
        println!("Total: {}", count);
    }
}

const CHEAT_DIST: usize = 20;

fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();
    let file = File::open(fname);
    let mut data: Vec<Vec<u8>> = Vec::new();
    let mut trail_start: (usize, usize) = (0, 0); // i, j == (row, col)
    let mut trail_end: (usize, usize) = (0, 0);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let mut row: Vec<u8> = Vec::new();
                        for c in l.chars() {
                            match c {
                                '.' => row.push(0),
                                '#' => row.push(1),
                                'E' => {
                                    row.push(0);
                                    trail_end = (data.len(), row.len() - 1);
                                }
                                'S' => {
                                    row.push(0);
                                    trail_start = (data.len(), row.len() - 1);
                                }
                                _ => panic!("Invalid character in input {}", c),
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
    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    solve(&data, trail_start, trail_end, CHEAT_DIST)
}

fn solve(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize), distance: usize) -> i64 {
    let mut original_path: Vec<(usize, usize)> = vec![];
    dfs(grid, start, end, HashSet::new(), &mut original_path);
    let path_len = original_path.len() - 1;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let path_map: HashMap<(usize, usize), usize> = original_path
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect();
    let mut counter = 0;
    for (idx, location) in original_path.iter().enumerate() {
        visited.insert(*location);
        let cheats = get_all_cheats_for_location(&path_map, *location, &visited, distance);
        for (l, md) in cheats {
            let dist = idx as i64 + md + (path_len - *(path_map.get(&l).unwrap())) as i64;
            let diff = path_len as i64 - dist;
            if diff > 0 {
                if diff >= 100 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn get_all_cheats_for_location(
    path: &HashMap<(usize, usize), usize>,
    start: (usize, usize),
    visited: &HashSet<(usize, usize)>,
    cheat_dist: usize,
) -> Vec<((usize, usize), i64)> {
    let mut cheats: Vec<((usize, usize), i64)> = vec![];
    for k in 1..cheat_dist + 1 {
        for j in 0..k {
            let p1 = (start.0 + j, start.1 + j - k);
            if path.contains_key(&p1) && !visited.contains(&p1) {
                cheats.push((p1, get_manhattan_distance(start, p1)));
            }
            let p2 = (start.0 + j - k, start.1 + j);
            if path.contains_key(&p2) && !visited.contains(&p2) {
                cheats.push((p2, get_manhattan_distance(start, p2)));
            }
            let p3 = (start.0 - j, start.1 - j + k);
            if path.contains_key(&p3) && !visited.contains(&p3) {
                cheats.push((p3, get_manhattan_distance(start, p3)));
            }
            let p4 = (start.0 - j + k, start.1 - j);
            if path.contains_key(&p4) && !visited.contains(&p4) {
                cheats.push((p4, get_manhattan_distance(start, p4)));
            }
        }
    }
    cheats
}

fn get_manhattan_distance(start: (usize, usize), end: (usize, usize)) -> i64 {
    (end.0 as i64 - start.0 as i64).abs() + (end.1 as i64 - start.1 as i64).abs()
}

fn get_next(grid: &Vec<Vec<u8>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 && grid[location.0][location.1 - 1] != 1 {
        steps.push((location.0, location.1 - 1));
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] != 1 {
        steps.push((location.0, location.1 + 1));
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] != 1 {
        steps.push((location.0 - 1, location.1));
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] != 1 {
        steps.push((location.0 + 1, location.1));
    }
    steps
}

fn dfs(
    grid: &Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    visited: HashSet<(usize, usize)>,
    path: &mut Vec<(usize, usize)>,
) -> i64 {
    let mut count = 0;
    let mut visited = visited;
    if start.0 == end.0 && start.1 == end.1 {
        path.push(end);
        return visited.len() as i64;
    }
    if grid[start.0][start.1] == 1 {
        return 0;
    }
    path.push(start);
    visited.insert(start);
    let next_steps = get_next(grid, start);
    for l in next_steps {
        if !visited.contains(&l) {
            let mut step_path = vec![];
            dfs(grid, l, end, visited.clone(), &mut step_path);
            if step_path.contains(&end) {
                path.extend(step_path);
                count += path.len() as i64;
                return count - 1;
            }
        }
    }
    0
}
