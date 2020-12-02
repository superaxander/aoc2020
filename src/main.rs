#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io;
use std::time::Instant;

mod day1;
mod day2;
mod common;

fn main() {
    pretty_env_logger::init();
    run_day("day 1", day1::main);
    run_day("day 2", day2::main);
}

fn run_day(name: &str, func: fn() -> io::Result<(i32, i32)>) {
    let now = Instant::now();
    let result = func();
    info!("took {:#?}", now.elapsed());
    match result {
        Ok((solution_a, solution_b)) => {
            info!("Solution {}a: {}", name, solution_a);
            info!("Solution {}b: {}", name, solution_b);
        }
        Err(err) => {
            error!("Error occurred running {}: {}", name, err)
        }
    }
}
