use std::fs::{self};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise4_1/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let data = fs::read_to_string(fname).unwrap();
    let mut matrix: Vec<Vec<&str>> = Vec::new();
    let rows = data.split("\n");
    for row in rows {
        let cols = row.split("");
        let mut row_data: Vec<&str> = Vec::new();
        for col in cols {
            if col == "" {
                continue;
            }
            row_data.push(col);
        }
        if row_data.len() == 0 {
            continue;
        }
        matrix.push(row_data);
    }
    return find_next_x(&matrix);
}

fn find_next_x(matrix: &Vec<Vec<&str>>) -> i32 {
    let row_width = matrix[0].len();
    let col_height = matrix.len();
    let mut total = 0;
    for i in 0..col_height {
        for j in 0..row_width {
            if matrix[i][j] == "X" {
                total +=
                    find_words_for_x(&matrix, j as i32, i as i32, row_width, col_height) as i32;
            }
        }
    }
    return total;
}

fn find_words_for_x(matrix: &Vec<Vec<&str>>, x: i32, y: i32, width: usize, height: usize) -> usize {
    let mut total = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let neighbour = neighbour(&matrix, x, y, j, i, width, height);
            if neighbour == "MAS" {
                total += 1
            }
        }
    }
    return total;
}

fn neighbour(
    matrix: &Vec<Vec<&str>>,
    x: i32,
    y: i32,
    x_neg: i32,
    y_neg: i32,
    width: usize,
    height: usize,
) -> String {
    let mut result: String = String::new();
    for index in 1..4 {
        let i: i32;
        i = y + (y_neg * index);
        let j: i32;
        j = x + (x_neg * index);

        if i < 0 || i >= height as i32 || j < 0 || j >= width as i32 {
            break;
        }
        result = format!("{}{}", result, matrix[i as usize][j as usize]);
    }
    return result.to_string();
}
