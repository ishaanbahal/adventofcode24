use std::{
    fs::{self},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise15_1/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug, Clone, Copy)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq)]
enum TILE {
    WALL,
    BLOCK,
    EMPTY,
    ROBOT,
}

fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();

    let data: Vec<String> = fs::read_to_string(fname)
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    let mut map: Vec<Vec<TILE>> = data[0]
        .clone()
        .split("\n")
        .map(|x| {
            x.split("")
                .filter(|x| !(x.is_empty() || *x == "\n"))
                .map(|x| match x.trim() {
                    "#" => TILE::WALL,
                    "." => TILE::EMPTY,
                    "O" => TILE::BLOCK,
                    "@" => TILE::ROBOT,
                    _ => panic!("Unknown tile: {}", x),
                })
                .collect()
        })
        .collect();
    let re = Regex::new(r"(\^|<|>|v){1}").unwrap();

    let moves: Vec<DIRECTION> = re
        .find_iter(data[1].as_str())
        .map(|x| match x.as_str() {
            "^" => DIRECTION::UP,
            "<" => DIRECTION::LEFT,
            ">" => DIRECTION::RIGHT,
            "v" => DIRECTION::DOWN,
            _ => panic!("Unknown direction"),
        })
        .collect();

    print_map(&map);

    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    manage_robot_moves(&moves, &mut map)
}

fn manage_robot_moves(moves: &Vec<DIRECTION>, map: &mut Vec<Vec<TILE>>) -> i64 {
    // Find robot starting position
    let mut robot_pos: (usize, usize) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TILE::ROBOT {
                robot_pos = (x, y);
                break;
            }
        }
    }
    for m in moves {
        robot_pos = move_robot(*m, map, robot_pos);
    }
    get_gps(map)
}

fn get_gps(map: &Vec<Vec<TILE>>) -> i64 {
    let mut total_gps = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == TILE::BLOCK {
                total_gps += (100 * (i as i64)) + (j as i64);
            }
        }
    }
    total_gps
}

fn get_next_position(m: DIRECTION, pos: (usize, usize)) -> (usize, usize) {
    match m {
        DIRECTION::UP => (pos.0 - 1, pos.1),
        DIRECTION::DOWN => (pos.0 + 1, pos.1),
        DIRECTION::LEFT => (pos.0, pos.1 - 1),
        DIRECTION::RIGHT => (pos.0, pos.1 + 1),
    }
}

fn move_robot(m: DIRECTION, map: &mut Vec<Vec<TILE>>, pos: (usize, usize)) -> (usize, usize) {
    let mut blocks_in_path: Vec<(usize, usize)> = Vec::new();
    let mut current_pos = pos.clone();
    loop {
        let next_pos = get_next_position(m, current_pos);
        if map[next_pos.0][next_pos.1] == TILE::WALL {
            return pos;
        }
        if map[next_pos.0][next_pos.1] == TILE::BLOCK {
            blocks_in_path.push(next_pos);
            current_pos = next_pos;
            continue;
        }
        if map[next_pos.0][next_pos.1] == TILE::EMPTY {
            break;
        }
    }
    // move robot one ahead
    map[pos.0][pos.1] = TILE::EMPTY;
    let next_robot_pos = get_next_position(m, pos);
    map[next_robot_pos.0][next_robot_pos.1] = TILE::ROBOT;

    for block in blocks_in_path {
        // move ahead by 1
        let next_block_pos = get_next_position(m, block);
        map[next_block_pos.0][next_block_pos.1] = TILE::BLOCK;
    }

    next_robot_pos
}

fn print_map(map: &Vec<Vec<TILE>>) {
    for row in map {
        for tile in row {
            print!(
                "{}",
                match tile {
                    TILE::WALL => "#",
                    TILE::BLOCK => "O",
                    TILE::EMPTY => ".",
                    TILE::ROBOT => "@",
                }
            );
        }
        println!();
    }
}
