#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use common::{Day, Runnable};
use std::time::Instant;

mod day1;
mod day2;
mod common;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    pretty_env_logger::init();
    let now = Instant::now();
    Day::Combined(day1::main).run("day 1");
    Day::Separated(day2::main).run("day 2");
    Day::SeparatedLong(day3::main).run("day 3");
    Day::Separated(day4::main).run("day 4");
    Day::Separated(day5::main).run("day 5");
    Day::SeparatedUsize(day6::main).run("day 6");
    Day::SeparatedUsize(day7::main).run("day 7");
    Day::Separated(day8::main).run("day 8");
    info!("All days together took {:#?}", now.elapsed());
}

