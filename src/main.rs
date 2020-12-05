#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use common::{Day, Runnable};

mod day1;
mod day2;
mod common;
mod day3;
mod day4;
mod day5;

fn main() {
    pretty_env_logger::init();
    Day::Combined(day1::main).run("day 1");
    Day::Separated(day2::main).run("day 2");
    Day::SeparatedLong(day3::main).run("day 3");
    Day::Separated(day4::main).run("day 4");
    Day::Separated(day5::main).run("day 5");
}

