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
        let count = parse_input("runner/exercise16_1/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PathType {
    ROAD,
    WALL,
    END,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Location {
    y: usize,
    x: usize,
    cost: i64,
    direction: Direction,
}

impl Location {
    fn new(y: usize, x: usize, current_cost: i64, direction: Direction) -> Location {
        Location {
            y,
            x,
            cost: current_cost,
            direction,
        }
    }
}

fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();
    let file = File::open(fname);
    let mut data: Vec<Vec<PathType>> = Vec::new();
    let mut trail_start: (usize, usize) = (0, 0); // i, j == (row, col)
    let mut trail_end: (usize, usize) = (0, 0);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let mut row: Vec<PathType> = Vec::new();
                        for c in l.chars() {
                            match c {
                                '.' => row.push(PathType::ROAD),
                                '#' => row.push(PathType::WALL),
                                'E' => {
                                    row.push(PathType::END);
                                    trail_end = (data.len(), row.len() - 1);
                                }
                                'S' => {
                                    row.push(PathType::ROAD);
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
    astar_search_end(&data, trail_start, trail_end, Direction::RIGHT)
}

fn get_distance_from_end(current_point: (usize, usize), end_point: (usize, usize)) -> i64 {
    return (end_point.0 as i64 - current_point.0 as i64).abs()
        + (end_point.1 as i64 - current_point.1 as i64).abs();
}

fn get_next_locations(location: Location) -> Vec<Location> {
    match location.direction {
        Direction::UP => {
            return vec![
                Location::new(location.y - 1, location.x, location.cost + 1, Direction::UP),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1001,
                    Direction::LEFT,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1001,
                    Direction::RIGHT,
                ),
            ]
        }
        Direction::DOWN => {
            return vec![
                Location::new(
                    location.y + 1,
                    location.x,
                    location.cost + 1,
                    Direction::DOWN,
                ),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1001,
                    Direction::LEFT,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1001,
                    Direction::RIGHT,
                ),
            ];
        }
        Direction::LEFT => {
            return vec![
                Location::new(
                    location.y - 1,
                    location.x,
                    location.cost + 1001,
                    Direction::UP,
                ),
                Location::new(
                    location.y + 1,
                    location.x,
                    location.cost + 1001,
                    Direction::DOWN,
                ),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1,
                    Direction::LEFT,
                ),
            ];
        }
        Direction::RIGHT => {
            return vec![
                Location::new(
                    location.y - 1,
                    location.x,
                    location.cost + 1001,
                    Direction::UP,
                ),
                Location::new(
                    location.y + 1,
                    location.x,
                    location.cost + 1001,
                    Direction::DOWN,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1,
                    Direction::RIGHT,
                ),
            ];
        }
    }
}

fn astar_search_end(
    layout: &Vec<Vec<PathType>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
    start_direction: Direction,
) -> i64 {
    let start_location = Location::new(start_point.0, start_point.1, 0, start_direction);
    let mut to_visit: Vec<Location> = vec![start_location];
    let max_y = layout.len();
    let max_x = layout[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !to_visit.is_empty() {
        let location = to_visit.remove(0);
        if layout[location.y][location.x] == PathType::END {
            return location.cost;
        }
        visited.insert((location.y, location.x));
        let next_locations = get_next_locations(location);
        for l in next_locations {
            if l.y <= max_y - 1
                && l.x <= max_x - 1
                && layout[l.y][l.x] != PathType::WALL
                && !visited.contains(&(l.y, l.x))
            {
                to_visit.push(l);
            }
        }
        to_visit.sort_by(|x, y| {
            let hx = get_distance_from_end((x.y, x.x), end_point);
            let hy = get_distance_from_end((y.y, y.x), end_point);

            let diff = (x.cost + hx as i64) - (y.cost + hy as i64);
            if diff == 0 {
                return std::cmp::Ordering::Equal;
            }
            if diff < 0 {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
    }
    0
}
