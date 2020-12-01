#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod day1;

fn main() {
    pretty_env_logger::init();
    match day1::main() {
        Ok((solution_a, solution_b)) => {
            info!("Solution day 1a: {}", solution_a);
            info!("Solution day 1b: {}", solution_b);
        }
        Err(err) => {
            error!("Error occurred running day 1: {}", err)
        }
    }
}
