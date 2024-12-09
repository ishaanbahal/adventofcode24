use std::fs::{self};

use regex::Regex;

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let safe_count = parse_input("runner/exercise9_2/testfile");
        println!("Total: {}", safe_count);
    }
}

fn parse_input(fname: &str) -> i64 {
    let data = fs::read_to_string(fname).unwrap();
    let re = Regex::new(r"\d").unwrap();
    let mut parsed_data: Vec<(i32, i32)> = Vec::new();
    let mut is_free_space = false;
    let mut id: i32 = 0;
    re.find_iter(data.as_str()).for_each(|x| {
        let num = x.as_str().parse::<i32>().unwrap();
        if !is_free_space {
            is_free_space = true;
            parsed_data.push((id, num));
            id += 1;
        } else {
            is_free_space = false;
            parsed_data.push((-1, num));
        }
    });

    checksum(&flatten_storage(defrag(&mut parsed_data, id)))
}

fn defrag(data: &mut Vec<(i32, i32)>, max_id: i32) -> &Vec<(i32, i32)> {
    let mut max_id = max_id;
    loop {
        let mut index = data.len() - 1;
        while data[index].0 == -1 || data[index].0 > max_id {
            index -= 1;
        }
        if index <= 0 {
            break;
        }
        if max_id <= 0 {
            break;
        }
        let (t_num, t_times) = data[index];
        let mut j = 0;
        loop {
            if j >= index {
                break;
            }
            if data[j].0 == -1 {
                let mut is_swapped = false;
                if data[j].1 >= t_times {
                    let free_space = data[j];
                    data[index] = (free_space.0, t_times);
                    data[j] = (t_num, t_times);

                    if free_space.1 - t_times > 0 {
                        data.insert(j + 1, (-1, free_space.1 - t_times));
                    }
                    is_swapped = true;
                }
                if is_swapped {
                    break;
                }
            }
            j += 1;
        }
        max_id -= 1;
    }
    data
}

fn space_compaction(data: &mut Vec<(i32, i32)>) {
    let mut i = 0;
    loop {
        if i >= data.len() {
            break;
        }
        if data[i].0 == -1 {
            let mut j = i + 1;
            loop {
                if j >= data.len() {
                    break;
                }
                if data[j].0 != -1 {
                    break;
                } else {
                    let empty_storage = data.remove(j);
                    data[i].1 += empty_storage.1;
                    j -= 1;
                }
                j += 1
            }
        }
        i += 1;
    }
}

fn checksum(data: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    data.iter().enumerate().for_each(|(index, num)| {
        if *num != -1 {
            sum += index as i64 * (*num) as i64;
        }
    });
    sum
}

fn flatten_storage(data: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut flat_data: Vec<i32> = Vec::new();
    for (num, times) in data {
        for _ in 0..*times {
            flat_data.push(num.clone());
        }
    }
    flat_data
}

fn print_storage_state(data: &Vec<(i32, i32)>) {
    data.iter().for_each(|(num, times)| {
        for _ in 0..*times {
            if *num == -1 {
                print!(".");
            } else {
                print!("{}", num);
            }
        }
    });
    print!("\n");
}
