use std::{
    fs::{self},
    io::{self, Read},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise15_2/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    BLOCK_OPEN,
    BLOCK_CLOSE,
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

    let map: Vec<Vec<TILE>> = data[0]
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

    let mut wide_map = widen_map(&map);

    println!(
        "Parse time: {:?} µs",
        parse_start.elapsed().unwrap().as_micros()
    );
    print_map(&wide_map);
    manage_robot_moves(&moves, &mut wide_map)
}

fn widen_map(map: &Vec<Vec<TILE>>) -> Vec<Vec<TILE>> {
    let mut wide_map: Vec<Vec<TILE>> = Vec::new();
    for row in map {
        let mut wide_row: Vec<TILE> = Vec::new();
        for tile in row {
            match tile {
                TILE::WALL => {
                    wide_row.push(TILE::WALL);
                    wide_row.push(TILE::WALL);
                }
                TILE::BLOCK => {
                    wide_row.push(TILE::BLOCK_OPEN);
                    wide_row.push(TILE::BLOCK_CLOSE);
                }
                TILE::EMPTY => {
                    wide_row.push(TILE::EMPTY);
                    wide_row.push(TILE::EMPTY);
                }
                TILE::ROBOT => {
                    wide_row.push(TILE::ROBOT);
                    wide_row.push(TILE::EMPTY);
                }
                _ => {}
            }
        }
        wide_map.push(wide_row);
    }
    wide_map
}

fn manage_robot_moves(moves: &Vec<DIRECTION>, map: &mut Vec<Vec<TILE>>) -> i64 {
    let mut robot_pos: (usize, usize) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TILE::ROBOT {
                robot_pos = (y, x);
                break;
            }
        }
    }
    //print_map(map);
    for m in moves {
        robot_pos = move_robot(*m, map, robot_pos);
        println!("Direction: {:?}", m);
        print_map(map);
        let mut buffer = [0u8; 1];
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => {
                println!("\x1B[2J");
                if buffer[0] == b'q' {
                    break;
                }
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }
    get_gps(map)
}

fn get_gps(map: &Vec<Vec<TILE>>) -> i64 {
    let mut total_gps = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == TILE::BLOCK_OPEN {
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

fn get_next_positions(
    m: DIRECTION,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Vec<(usize, usize)> {
    match m {
        DIRECTION::UP => {
            let mut positions: Vec<(usize, usize)> = Vec::new();
            for i in start_pos.1..end_pos.1 + 1 {
                positions.push((start_pos.0 - 1, i));
            }
            positions
        }
        DIRECTION::DOWN => {
            let mut positions: Vec<(usize, usize)> = Vec::new();
            for i in start_pos.1..end_pos.1 + 1 {
                positions.push((start_pos.0 + 1, i));
            }
            positions
        }
        DIRECTION::LEFT => vec![(start_pos.0, start_pos.1 - 1)],
        DIRECTION::RIGHT => vec![(start_pos.0, end_pos.1 + 1)],
    }
}

fn move_robot(m: DIRECTION, map: &mut Vec<Vec<TILE>>, pos: (usize, usize)) -> (usize, usize) {
    let mut start_pos = pos.clone();
    let mut end_pos = pos.clone();

    let mut blocks_in_path: Vec<(usize, usize, TILE)> = Vec::new();
    loop {
        let next_positions = get_next_positions(m, start_pos, end_pos);
        println!(
            "Start: {:?}, End: {:?}. Direction: {:?}, Next positions: {:?}",
            start_pos, end_pos, m, next_positions
        );
        let mut clean_break: bool = true;
        for next_pos in next_positions {
            if map[next_pos.0][next_pos.1] == TILE::WALL {
                return pos;
            }
            if map[next_pos.0][next_pos.1] == TILE::BLOCK_OPEN {
                clean_break = false;
                blocks_in_path.push((next_pos.0, next_pos.1, TILE::BLOCK_OPEN));
                blocks_in_path.push((next_pos.0, next_pos.1 + 1, TILE::BLOCK_CLOSE));
                if start_pos.1 >= next_pos.1 {
                    start_pos = (next_pos.0, next_pos.1);
                } else {
                    start_pos = (next_pos.0, start_pos.1);
                }
                if end_pos.1 <= next_pos.1 {
                    end_pos = (next_pos.0, next_pos.1 + 1);
                } else {
                    end_pos = (next_pos.0, end_pos.1);
                }
            }
            if map[next_pos.0][next_pos.1] == TILE::BLOCK_CLOSE {
                clean_break = false;
                blocks_in_path.push((next_pos.0, next_pos.1 - 1, TILE::BLOCK_OPEN));
                blocks_in_path.push((next_pos.0, next_pos.1, TILE::BLOCK_CLOSE));
                if start_pos.1 >= next_pos.1 - 1 {
                    start_pos = (next_pos.0, next_pos.1 - 1);
                } else {
                    start_pos = (next_pos.0, start_pos.1);
                }

                if end_pos.1 <= next_pos.1 {
                    end_pos = (next_pos.0, next_pos.1);
                } else {
                    end_pos = (next_pos.0, end_pos.1);
                }
            }
            if m == DIRECTION::LEFT {
                start_pos = (next_pos.0, next_pos.1 - 1);
                end_pos = (next_pos.0, next_pos.1);
            }
            if m == DIRECTION::RIGHT {
                start_pos = (next_pos.0, next_pos.1);
                end_pos = (next_pos.0, next_pos.1 + 1);
            }
        }
        if clean_break {
            break;
        }
    }
    for block in &blocks_in_path {
        // mark all blocks as empty
        map[block.0][block.1] = TILE::EMPTY;
    }
    map[pos.0][pos.1] = TILE::EMPTY;
    let next_robot_pos = get_next_position(m, pos);
    map[next_robot_pos.0][next_robot_pos.1] = TILE::ROBOT;

    for block in blocks_in_path {
        let next_block_pos = get_next_position(m, (block.0, block.1));
        map[next_block_pos.0][next_block_pos.1] = block.2;
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
                    TILE::BLOCK_OPEN => "[",
                    TILE::BLOCK_CLOSE => "]",
                }
            );
        }
        println!();
    }
}
