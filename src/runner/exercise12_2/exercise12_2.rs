use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise12_2/testfile");
        println!("Total: {}", count);
    }
}

fn parse_input(fname: &str) -> i32 {
    let parse_start = SystemTime::now();
    let file = File::open(fname);
    let mut data: Vec<Vec<char>> = Vec::new();
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let re = Regex::new(r"[A-Za-z]{1}").unwrap();
            for line in reader.lines() {
                let mut row: Vec<char> = Vec::new();
                match line {
                    Ok(l) => {
                        re.find_iter(&l).for_each(|x| {
                            row.push(x.as_str().chars().nth(0).unwrap());
                        });
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
    find_island_neighbours(&data)
}

struct Node {
    i: usize,
    j: usize,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl Node {
    fn new(i: usize, j: usize) -> Node {
        Node {
            i,
            j,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

fn insert_to_map(map: &mut HashMap<usize, usize>, item: usize) {
    if map.contains_key(&item) {
        let prev_count = map.get(&item).unwrap();
        map.insert(item, 1 + prev_count);
    } else {
        map.insert(item, 1);
    }
}
fn get_perimeter_for_node(data: &Vec<Vec<char>>, all_nodes: &HashSet<(usize, usize)>) -> i32 {
    let mut total_perimeter = 0;
    let mut visited_nodes: HashMap<(usize, usize), Node> = HashMap::new();
    for item in all_nodes {
        let i = item.0;
        let j = item.1;
        let mut node = Node::new(i, j);
        if !all_nodes.contains(&(i - 1, j)) {
            if visited_nodes.contains_key(&(i, j - 1)) {
                let temp_node = visited_nodes.get_mut(&(i, j - 1)).unwrap();
                if temp_node.left {
                    node.left = true;
                }
            }
            if visited_nodes.contains_key(&(i, j + 1)) {
                let temp_node = visited_nodes.get_mut(&(i, j + 1)).unwrap();
                if temp_node.right {
                    node.right = true;
                }
            }
            if (node.left || node.right) {
                total_perimeter += 1;
            }
        }
        if !all_nodes.contains(&(i + 1, j)) {
            if (node.left || node.right) {
                total_perimeter += 1;
            }
        }

        if !all_nodes.contains(&(i, j - 1)) {
            if visited_nodes.contains_key(&(i - 1, j)) {
                let temp_node = visited_nodes.get_mut(&(i - 1, j)).unwrap();
                if temp_node.up {
                    node.up = true;
                }
            }
            if visited_nodes.contains_key(&(i + 1, j)) {
                let temp_node = visited_nodes.get_mut(&(i + 1, j)).unwrap();
                if temp_node.down {
                    node.down = true;
                }
            }
            if node.up || node.down {
                total_perimeter += 1;
            }
        }
        if !all_nodes.contains(&(i, j + 1)) {
            if node.up || node.down {
                total_perimeter += 1;
            }
        }
        visited_nodes.insert((i, j), node);
    }
    total_perimeter as i32
}

fn level_order_traversal(
    data: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    ch: char,
) -> (i32, HashSet<(usize, usize)>) {
    let mut next_nodes: Vec<(usize, usize)> = vec![(i, j)];
    let perimeter: i32;
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    while next_nodes.len() != 0 {
        let (i, j) = next_nodes.pop().unwrap();
        if visited_nodes.contains(&(i, j)) {
            continue;
        }
        if i > 0 && !visited_nodes.contains(&(i - 1, j)) && data[i - 1][j] == ch {
            next_nodes.push((i - 1, j))
        }
        if i < data.len() - 1 && !visited_nodes.contains(&(i + 1, j)) && data[i + 1][j] == ch {
            next_nodes.push((i + 1, j))
        }
        if j > 0 && !visited_nodes.contains(&(i, j - 1)) && data[i][j - 1] == ch {
            next_nodes.push((i, j - 1))
        }
        if j < data[0].len() - 1 && !visited_nodes.contains(&(i, j + 1)) && data[i][j + 1] == ch {
            next_nodes.push((i, j + 1))
        }
        visited_nodes.insert((i, j));
    }

    perimeter = get_perimeter_for_node(data, &visited_nodes);
    println!("Sides of {} are {}", ch, perimeter);
    (perimeter, visited_nodes)
}

fn find_island_neighbours(data: &Vec<Vec<char>>) -> i32 {
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut total_cost = 0;
    let mut current_ch: char = ' ';
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if visited_nodes.contains(&(i, j)) {
                continue;
            }
            if data[i][j] != current_ch {
                current_ch = data[i][j];
            }
            // Level order traversal
            let (perimeter, nodes) = level_order_traversal(data, i, j, current_ch);
            total_cost += perimeter * nodes.len() as i32;
            visited_nodes.extend(nodes);
        }
    }
    total_cost
}
