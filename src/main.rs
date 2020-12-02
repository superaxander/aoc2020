#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use std::time::Instant;
use std::io;

mod day1;
mod day2;

fn main() {
    pretty_env_logger::init();
    //run_day(day1::main);
    run_day(day2::main)
}

fn run_day(func: fn() -> io::Result<(i32,i32)>) {
    let now = Instant::now();
    let result = func();
    info!("took {:#?}", now.elapsed());
    match result {
        Ok((solution_a, solution_b)) => {
            info!("Solution day 1a: {}", solution_a);
            info!("Solution day 1b: {}", solution_b);
        }
        Err(err) => {
            error!("Error occurred running day 1: {}", err)
        }
    }
}
