use std::fs::{self};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise6_1/testfile");
        println!("Total: {}", safe_count);
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn _get_direction(dir: &str) -> Direction {
    match dir {
        "^" => Direction::UP,
        "v" => Direction::DOWN,
        "<" => Direction::LEFT,
        ">" => Direction::RIGHT,
        _ => Direction::UP,
    }
}

fn _get_next_rotation(direction: Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

fn parse_input(fname: &str) -> i32 {
    let data = fs::read_to_string(fname).unwrap();
    let mut matrix: Vec<Vec<&str>> = Vec::new();
    let mut row_index = 0;
    let mut col_index;
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;

    let rows = data.split("\n");
    for row in rows {
        col_index = 0;
        let cols = row.split("");
        let mut row_data: Vec<&str> = Vec::new();
        for col in cols {
            if col.is_empty() {
                continue;
            }
            if col == "^" || col == "v" || col == "<" || col == ">" {
                start_x = col_index as i32;
                start_y = row_index as i32;
            }
            row_data.push(col);
            col_index += 1;
        }
        if row_data.len() == 0 {
            continue;
        }
        matrix.push(row_data);
        row_index += 1;
    }
    return follow_path_until_block(&mut matrix, start_x, start_y);
}

fn follow_path_until_block(matrix: &mut Vec<Vec<&str>>, x: i32, y: i32) -> i32 {
    let mut visited_counter: i32 = 0;
    let mut x = x;
    let mut y = y;
    let mut direction = _get_direction(matrix[y as usize][x as usize]);
    let mut direction_change_count = 0;
    let total_rows = matrix.len() - 1;
    let total_cols = matrix[0].len() - 1;
    loop {
        loop {
            if matrix[y as usize][x as usize] != "x" {
                visited_counter += 1;
                matrix[y as usize][x as usize] = "x";
            }
            if x < 0 || y < 0 || x >= total_cols as i32 || y >= total_rows as i32 {
                for row in matrix {
                    println!("{}", row.join(""));
                }
                println!("Exit at: x: {}, y: {}", x + 1, y + 1);
                println!("Direction changes: {}", direction_change_count);
                return visited_counter;
            }
            let mut peek_x = x.clone();
            let mut peek_y = y.clone();
            match direction {
                Direction::UP => {
                    peek_y -= 1;
                }
                Direction::DOWN => {
                    peek_y += 1;
                }
                Direction::LEFT => {
                    peek_x -= 1;
                }
                Direction::RIGHT => {
                    peek_x += 1;
                }
            }
            if matrix[peek_y as usize][peek_x as usize] == "#" {
                //println!("Hit a wall at x: {}, y: {}", peek_x, peek_y);
                break;
            } else {
                x = peek_x;
                y = peek_y;
            }
        }
        direction = _get_next_rotation(direction);
        direction_change_count += 1;
    }
}
