use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::SystemTime,
};

use crate::modrunner::modrunner::ModRunner;

pub struct Worker {}

impl ModRunner for Worker {
    fn run(&self) {
        let count = parse_input("runner/exercise17_2/testfile");
        println!("Total: {}", count);
    }
}

#[derive(Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,

    output: Vec<i8>,
    program: Vec<i8>,
}

impl Computer {
    fn new() -> Computer {
        return Computer {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
            program: vec![],
        };
    }

    fn combo_opd(&mut self, val: i8) -> usize {
        match val {
            0 => return 0,
            1 => return 1,
            2 => return 2,
            3 => return 3,
            4 => return self.reg_a,
            5 => return self.reg_b,
            6 => return self.reg_c,
            _ => panic!("Invalid register {}", val),
        }
    }

    fn adv(&mut self, val: i8) -> usize {
        let den = 2usize.pow(self.combo_opd(val) as u32);
        self.reg_a = self.reg_a / den;
        return 2;
    }

    fn bxl(&mut self, val: i8) -> usize {
        self.reg_b = self.reg_b ^ (val as usize);
        return 2;
    }

    fn bst(&mut self, val: i8) -> usize {
        let opd = self.combo_opd(val);
        self.reg_b = opd.rem_euclid(8);
        return 2;
    }
    fn jnz(&mut self, val: i8, idx: usize) -> usize {
        if self.reg_a != 0 {
            return val as usize;
        }
        return idx + 2;
    }

    fn bxc(&mut self) -> usize {
        self.reg_b = self.reg_c ^ self.reg_b;
        return 2;
    }
    fn out(&mut self, val: i8) -> usize {
        let opd = self.combo_opd(val) as i8;
        self.output.push(opd.rem_euclid(8));
        return 2;
    }
    fn bdv(&mut self, val: i8) -> usize {
        let den = 2usize.pow(self.combo_opd(val) as u32);
        self.reg_b = self.reg_a / den;
        return 2;
    }
    fn cdv(&mut self, val: i8) -> usize {
        let den = usize::pow(2, self.combo_opd(val) as u32);
        self.reg_c = self.reg_a / den;
        return 2;
    }
}

fn parse_input(fname: &str) -> i64 {
    let parse_start = SystemTime::now();
    let file = File::open(fname);
    let mut computer: Computer = Computer::new();
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        if l.is_empty() {
                            continue;
                        }
                        if l.contains("Register A: ") {
                            let reg_a: usize =
                                l.replace("Register A: ", "").trim().parse().unwrap();

                            computer.reg_a = reg_a;
                        }
                        if l.contains("Register B: ") {
                            let reg_b: usize = l.replace("Register B: ", "").parse().unwrap();
                            computer.reg_b = reg_b;
                        }
                        if l.contains("Register C: ") {
                            let reg_c: usize = l.replace("Register C: ", "").parse().unwrap();
                            computer.reg_c = reg_c;
                        }
                        if l.contains("Program: ") {
                            let program: Vec<i8> = l
                                .replace("Program: ", "")
                                .replace(" ", "")
                                .split(",")
                                .map(|x| x.parse().unwrap())
                                .collect();
                            computer.program = program;
                        }
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
    let mut idx = 0;
    let mut seed_a = 0;
    let pg_str = computer
        .program
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Program: {}", pg_str);
    loop {
        let mut c = computer.clone();
        c.reg_a = seed_a;
        //println!("Attempting seed: {}", seed_a);
        loop {
            if idx > c.program.len() - 1 {
                break;
            }
            let op = c.program[idx];
            match op {
                0 => {
                    idx += c.adv(c.program[idx + 1]);
                }
                1 => {
                    idx += c.bxl(c.program[idx + 1]);
                }
                2 => {
                    idx += c.bst(c.program[idx + 1]);
                }
                3 => {
                    idx = c.jnz(c.program[idx + 1], idx);
                }
                4 => {
                    idx += c.bxc();
                }
                5 => {
                    idx += c.out(c.program[idx + 1]);
                }
                6 => {
                    idx += c.bdv(c.program[idx + 1]);
                }
                7 => {
                    idx += c.cdv(c.program[idx + 1]);
                }
                _ => {
                    idx += 2;
                }
            }
        }
        let out = c
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        if out == pg_str {
            break;
        }
        println!(
            "Seed: {}, Program: {}, Length: {}, Output: {}, Length: {}",
            seed_a,
            pg_str,
            c.program.len(),
            out,
            c.output.len()
        );
        if c.program.len() - c.output.len() >= 2 {
            seed_a += 100000000000;
        } else if c.program.len() - c.output.len() == 1 {
            seed_a += 100000000;
        } else {
            seed_a += 1;
        }
        idx = 0;
    }
    seed_a as i64
}
