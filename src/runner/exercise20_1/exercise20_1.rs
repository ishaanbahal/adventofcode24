use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise20_1/testfile");
        println!("Total: {}", count);
    }
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

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
    solve(&data, trail_start, trail_end)
}

fn solve(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut original_path: Vec<(usize, usize)> = vec![];
    dfs(grid, start, end, HashSet::new(), &mut original_path);
    let mut total = 0;
    for (idx, location) in original_path.iter().enumerate() {
        let cheat_exits = get_cheats(grid, *location, &original_path);
        cheat_exits.iter().for_each(|c| {
            for i in idx..original_path.len() {
                if original_path[i] == *c {
                    let diff = i - idx - 2;
                    if diff >= 100 {
                        total += 1;
                    }
                }
            }
        });
    }
    total
}

fn get_cheats(
    grid: &Vec<Vec<u8>>,
    location: (usize, usize),
    original_path: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 && grid[location.0][location.1 - 1] == 1 {
        let step = find_connection_point(
            grid,
            (location.0, location.1 - 1),
            Direction::LEFT,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] == 1 {
        let step = find_connection_point(
            grid,
            (location.0, location.1 + 1),
            Direction::RIGHT,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] == 1 {
        let step = find_connection_point(
            grid,
            (location.0 - 1, location.1),
            Direction::UP,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] == 1 {
        let step = find_connection_point(
            grid,
            (location.0 + 1, location.1),
            Direction::DOWN,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    steps
}

fn find_connection_point(
    grid: &Vec<Vec<u8>>,
    location: (usize, usize),
    direction: Direction,
    path: &Vec<(usize, usize)>,
) -> (usize, usize) {
    let mut current = location;
    loop {
        match direction {
            Direction::LEFT => {
                if current.1 == 0 {
                    break;
                }
                current = (current.0, current.1 - 1);
            }
            Direction::RIGHT => {
                if current.1 == grid[0].len() - 1 {
                    break;
                }
                current = (current.0, current.1 + 1);
            }
            Direction::UP => {
                if current.0 == 0 {
                    break;
                }
                current = (current.0 - 1, current.1);
            }
            Direction::DOWN => {
                if current.0 == grid.len() - 1 {
                    break;
                }
                current = (current.0 + 1, current.1);
            }
        }
        if grid[current.0][current.1] == 1 {
            break;
        }
        if path.contains(&current) {
            return current;
        }
    }
    (usize::MAX, usize::MAX)
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
