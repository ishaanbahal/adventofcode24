use std::fs::{self};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise4_2/testfile");
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
            if matrix[i][j] == "A" {
                if is_xmas(&matrix, j as i32, i as i32, row_width, col_height) {
                    total += 1;
                };
            }
        }
    }
    return total;
}

fn is_xmas(matrix: &Vec<Vec<&str>>, x: i32, y: i32, width: usize, height: usize) -> bool {
    if x - 1 < 0 || (x + 1) as usize >= width || y - 1 < 0 || (y + 1) as usize >= height {
        return false;
    }

    let mut is_diag1_correct: bool = false;
    let mut is_diag2_correct: bool = false;
    let diag1: String;
    let diag2: String;
    diag1 = matrix[(y - 1) as usize][(x - 1) as usize].to_string()
        + matrix[y as usize][x as usize]
        + matrix[(y + 1) as usize][(x + 1) as usize];

    diag2 = matrix[(y - 1) as usize][(x + 1) as usize].to_string()
        + matrix[y as usize][x as usize]
        + matrix[(y + 1) as usize][(x - 1) as usize];

    if diag1 == "SAM" || diag1 == "MAS" {
        is_diag1_correct = true;
    }
    if diag2 == "SAM" || diag2 == "MAS" {
        is_diag2_correct = true;
    }
    return is_diag2_correct && is_diag1_correct;
}
