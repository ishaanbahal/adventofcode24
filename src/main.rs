mod modrunner;
mod runner;
use std::{env, process, time::SystemTime};

use modrunner::modrunner::{InvalidWorker, ModRunner};
use runner::{
    exercise10_1::exercise10_1, exercise10_2::exercise10_2, exercise11_1::exercise11_1,
    exercise11_2::exercise11_2, exercise12_1::exercise12_1, exercise12_2::exercise12_2,
    exercise13_1::exercise13_1, exercise13_2::exercise13_2, exercise14_1::exercise14_1,
    exercise14_2::exercise14_2, exercise15_1::exercise15_1, exercise15_2::exercise15_2,
    exercise16_1::exercise16_1, exercise16_2::exercise16_2, exercise17_1::exercise17_1,
    exercise17_2::exercise17_2, exercise18_1::exercise18_1, exercise18_2::exercise18_2,
    exercise19_1::exercise19_1, exercise19_2::exercise19_2, exercise1_1::exercise1_1,
    exercise1_2::exercise1_2, exercise20_1::exercise20_1, exercise20_2::exercise20_2,
    exercise2_1::exercise2_1, exercise2_2::exercise2_2, exercise3_1::exercise3_1,
    exercise3_2::exercise3_2, exercise4_1::exercise4_1, exercise4_2::exercise4_2,
    exercise5_1::exercise5_1, exercise5_2::exercise5_2, exercise6_1::exercise6_1,
    exercise6_2::exercise6_2, exercise7_1::exercise7_1, exercise7_2::exercise7_2,
    exercise8_1::exercise8_1, exercise8_2::exercise8_2, exercise9_1::exercise9_1,
    exercise9_2::exercise9_2,
};

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
        "2_1" => wrkr = &exercise2_1::Worker {},
        "2_2" => wrkr = &exercise2_2::Worker {},
        "3_1" => wrkr = &exercise3_1::Worker {},
        "3_2" => wrkr = &exercise3_2::Worker {},
        "4_1" => wrkr = &exercise4_1::Worker {},
        "4_2" => wrkr = &exercise4_2::Worker {},
        "5_1" => wrkr = &exercise5_1::Worker {},
        "5_2" => wrkr = &exercise5_2::Worker {},
        "6_1" => wrkr = &exercise6_1::Worker {},
        "6_2" => wrkr = &exercise6_2::Worker {},
        "7_1" => wrkr = &exercise7_1::Worker {},
        "7_2" => wrkr = &exercise7_2::Worker {},
        "8_1" => wrkr = &exercise8_1::Worker {},
        "8_2" => wrkr = &exercise8_2::Worker {},
        "9_1" => wrkr = &exercise9_1::Worker {},
        "9_2" => wrkr = &exercise9_2::Worker {},
        "10_1" => wrkr = &exercise10_1::Worker {},
        "10_2" => wrkr = &exercise10_2::Worker {},
        "11_1" => wrkr = &exercise11_1::Worker {},
        "11_2" => wrkr = &exercise11_2::Worker {},
        "12_1" => wrkr = &exercise12_1::Worker {},
        "12_2" => wrkr = &exercise12_2::Worker {},
        "13_1" => wrkr = &exercise13_1::Worker {},
        "13_2" => wrkr = &exercise13_2::Worker {},
        "14_1" => wrkr = &exercise14_1::Worker {},
        "14_2" => wrkr = &exercise14_2::Worker {},
        "15_1" => wrkr = &exercise15_1::Worker {},
        "15_2" => wrkr = &exercise15_2::Worker {},
        "16_1" => wrkr = &exercise16_1::Worker {},
        "16_2" => wrkr = &exercise16_2::Worker {},
        "17_1" => wrkr = &exercise17_1::Worker {},
        "17_2" => wrkr = &exercise17_2::Worker {},
        "18_1" => wrkr = &exercise18_1::Worker {},
        "18_2" => wrkr = &exercise18_2::Worker {},
        "19_1" => wrkr = &exercise19_1::Worker {},
        "19_2" => wrkr = &exercise19_2::Worker {},
        "20_1" => wrkr = &exercise20_1::Worker {},
        "20_2" => wrkr = &exercise20_2::Worker {},
        _ => {}
    }
    let now = SystemTime::now();
    wrkr.run();
    println!("Time elapsed: {} µs", now.elapsed().unwrap().as_micros());
}
