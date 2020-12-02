#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use common::{Day, Runnable};

mod day1;
mod day2;
mod common;

fn main() {
    pretty_env_logger::init();
    Day::Combined(day1::main).run("day 1");
    Day::Separated(day2::main).run("day 2");
}

