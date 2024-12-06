use std::fs::{self};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise6_2/testfile");
        println!("Total: {}", safe_count);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn _get_direction(dir: String) -> Direction {
    match dir.as_str() {
        "^" => Direction::UP,
        "v" => Direction::DOWN,
        "<" => Direction::LEFT,
        ">" => Direction::RIGHT,
        _ => Direction::UP,
    }
}

fn _get_next_rotation(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn parse_input(fname: &str) -> i32 {
    let data = fs::read_to_string(fname).unwrap();
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut row_index = 0;
    let mut col_index;
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;

    let rows = data.split("\n");
    for row in rows {
        col_index = 0;
        let cols = row.split("");
        let mut row_data: Vec<String> = Vec::new();
        for col in cols {
            if col.is_empty() {
                continue;
            }
            if col == "^" || col == "v" || col == "<" || col == ">" {
                start_x = col_index as i32;
                start_y = row_index as i32;
            }
            row_data.push(col.to_string());
            col_index += 1;
        }
        if row_data.len() == 0 {
            continue;
        }
        matrix.push(row_data);
        row_index += 1;
    }
    return find_obstacles(&mut matrix, start_x, start_y);
}

fn is_exit(x: i32, y: i32, total_rows: usize, total_cols: usize) -> bool {
    if x < 0 || y < 0 || x > (total_cols - 1) as i32 || y > (total_rows - 1) as i32 {
        return true;
    }
    return false;
}

fn get_dir_str(direction: &Direction) -> &str {
    match direction {
        Direction::UP => return "^",
        Direction::DOWN => return "v",
        Direction::LEFT => return "<",
        Direction::RIGHT => return ">",
    }
}

fn is_same_direction(data: &str, direction: &Direction) -> bool {
    let dir_str: &str = get_dir_str(direction);
    data.contains(dir_str)
}

fn is_looping(
    start_x: i32,
    start_y: i32,
    x: i32,
    y: i32,
    matrix: &Vec<Vec<String>>,
    start_direction: &Direction,
) -> bool {
    let mut matrix = matrix.clone();
    matrix[y as usize][x as usize] = "#".to_string();

    let total_rows = matrix.len();
    let total_cols = matrix[0].len();
    let mut x = start_x;
    let mut y = start_y;
    let mut direction = start_direction.clone();
    let mut first_iter = true;
    loop {
        loop {
            //println!("{:?}", matrix);
            if is_exit(x, y, total_rows, total_cols) {
                return false;
            }
            if matrix[y as usize][x as usize] != "."
                && is_same_direction(&matrix[y as usize][x as usize], &direction)
                && !first_iter
            {
                return true;
            }
            // Checking re-encounter
            if matrix[y as usize][x as usize] != "#" {
                matrix[y as usize][x as usize] =
                    matrix[y as usize][x as usize].to_string() + get_dir_str(&direction);
            }

            first_iter = false;
            let (peek_x, peek_y) = get_next_coordinate(x, y, &direction);
            if !is_exit(peek_x, peek_y, total_rows, total_cols)
                && matrix[peek_y as usize][peek_x as usize] == "#"
            {
                break;
            } else {
                x = peek_x;
                y = peek_y;
            }
        }
        direction = _get_next_rotation(&direction);
    }
}

fn find_obstacles(matrix: &mut Vec<Vec<String>>, x: i32, y: i32) -> i32 {
    let start_x = x;
    let start_y = y;
    let direction = _get_direction(matrix[y as usize][x as usize].clone());
    let total_rows = matrix.len();
    let total_cols = matrix[0].len();
    let mut obstacle_counter = 0;
    for i in 0..total_rows {
        for j in 0..total_cols {
            if matrix[i][j] == "#" || matrix[i][j] == "^" {
                continue;
            }
            if is_looping(start_x, start_y, j as i32, i as i32, matrix, &direction) {
                obstacle_counter += 1;
            }
        }
    }
    return obstacle_counter;
}

fn get_next_coordinate(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::UP => {
            return (x, y - 1);
        }
        Direction::DOWN => {
            return (x, y + 1);
        }
        Direction::LEFT => {
            return (x - 1, y);
        }
        Direction::RIGHT => {
            return (x + 1, y);
        }
    }
}
