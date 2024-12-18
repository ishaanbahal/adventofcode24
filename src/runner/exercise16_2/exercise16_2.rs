use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise16_2/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PathType {
    ROAD,
    WALL,
    END,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
struct Location {
    y: usize,
    x: usize,
    cost: i64,
    direction: Direction,
    current_path: Vec<(usize, usize)>,
}

impl Location {
    fn new(
        y: usize,
        x: usize,
        current_cost: i64,
        direction: Direction,
        current_path: Vec<(usize, usize)>,
    ) -> Location {
        Location {
            y,
            x,
            cost: current_cost,
            direction,
            current_path,
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
    let mut up_path = location.current_path.clone();
    let mut down_path = location.current_path.clone();
    let mut right_path = location.current_path.clone();
    let mut left_path = location.current_path.clone();

    up_path.push((location.y - 1, location.x));
    down_path.push((location.y + 1, location.x));
    left_path.push((location.y, location.x - 1));
    right_path.push((location.y, location.x + 1));
    match location.direction {
        Direction::UP => {
            return vec![
                Location::new(
                    location.y - 1,
                    location.x,
                    location.cost + 1,
                    Direction::UP,
                    up_path,
                ),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1001,
                    Direction::LEFT,
                    left_path,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1001,
                    Direction::RIGHT,
                    right_path,
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
                    down_path,
                ),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1001,
                    Direction::LEFT,
                    left_path,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1001,
                    Direction::RIGHT,
                    right_path,
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
                    up_path,
                ),
                Location::new(
                    location.y + 1,
                    location.x,
                    location.cost + 1001,
                    Direction::DOWN,
                    down_path,
                ),
                Location::new(
                    location.y,
                    location.x - 1,
                    location.cost + 1,
                    Direction::LEFT,
                    left_path,
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
                    up_path,
                ),
                Location::new(
                    location.y + 1,
                    location.x,
                    location.cost + 1001,
                    Direction::DOWN,
                    down_path,
                ),
                Location::new(
                    location.y,
                    location.x + 1,
                    location.cost + 1,
                    Direction::RIGHT,
                    right_path,
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
    let start_location = Location::new(
        start_point.0,
        start_point.1,
        0,
        start_direction,
        vec![start_point],
    );
    let mut to_visit: Vec<Location> = vec![start_location];
    let max_y = layout.len();
    let max_x = layout[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut final_cost = 0;
    while !to_visit.is_empty() {
        let location = to_visit.remove(0);
        if layout[location.y][location.x] == PathType::END {
            final_cost = location.cost;
            break;
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
    find_all_paths(layout, start_point, end_point, start_direction, final_cost)
}

fn find_all_paths(
    layout: &Vec<Vec<PathType>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
    start_direction: Direction,
    total_cost: i64,
) -> i64 {
    let start_location = Location::new(
        start_point.0,
        start_point.1,
        0,
        start_direction,
        vec![start_point],
    );
    let mut to_visit: Vec<Location> = vec![start_location];
    let max_y = layout.len();
    let max_x = layout[0].len();
    let mut grid: Vec<Vec<usize>> = vec![vec![usize::MAX; max_x]; max_y];
    let mut paths: Vec<Vec<(usize, usize)>> = vec![];
    println!("Total cost {total_cost}");
    while !to_visit.is_empty() {
        let location = to_visit.remove(0);
        //print_layout(layout, &location);
        //sleep(Duration::from_millis(100));
        let next_locations = get_next_locations(location.clone());
        //println!("Location: {:?}", location.cost);
        if grid[location.y][location.x] >= location.current_path.len() {
            grid[location.y][location.x] = location.current_path.len();
        } else {
            continue;
        }
        for l in next_locations {
            if location.current_path.contains(&(l.y, l.x)) {
                continue;
            }
            if l.y <= max_y - 1 && l.x <= max_x - 1 {
                if layout[l.y][l.x] == PathType::END && location.cost == total_cost - 1 {
                    paths.push(l.current_path.clone());
                    continue;
                }
                if layout[l.y][l.x] != PathType::WALL && location.cost < total_cost {
                    to_visit.push(l);
                }
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
    let mut all_points: HashSet<(usize, usize)> = HashSet::new();
    for path in paths {
        for l in path {
            all_points.insert(l);
        }
    }
    all_points.len() as i64
}

fn print_layout(layout: &Vec<Vec<PathType>>, location: &Location) {
    print!("\x1B[2J");
    for (i, row) in layout.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if i == location.y && j == location.x {
                print!("O");
                continue;
            }
            match item {
                PathType::ROAD => {
                    print!("_");
                }
                PathType::WALL => {
                    print!("|");
                }
                PathType::END => {
                    print!("E");
                }
            }
        }
        println!();
    }
}
