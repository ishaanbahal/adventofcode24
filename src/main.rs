mod modrunner;
mod runner;
use std::{env, process};

use modrunner::modrunner::{InvalidWorker, ModRunner};
use runner::{exercise1_1::exercise1_1, exercise1_2::exercise1_2};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide exercise path like\ncargo run 1_1");
        process::exit(1)
    }
    let exercise_code: &str = &args[1];
    let ex_parts: Vec<&str> = exercise_code.split("_").collect();
    if ex_parts.len() < 2 {
        println!("Please provide exercise path like\ncargo run 1_1");
        process::exit(1)
    }
    let mut wrkr: &dyn ModRunner = &InvalidWorker {};
    match exercise_code {
        "1_1" => wrkr = &exercise1_1::Worker {},
        "1_2" => wrkr = &exercise1_2::Worker {},
        _ => {}
    }
    return wrkr.run();
}
