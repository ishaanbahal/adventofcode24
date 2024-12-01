use std::process;

pub trait ModRunner {
    fn run(&self);
}

pub struct InvalidWorker {}

impl ModRunner for InvalidWorker {
    fn run(&self) {
        println!("Invalid Worker");
        process::exit(1);
    }
}
