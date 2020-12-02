use core::convert::AsRef;
use core::result::Result::Ok;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
