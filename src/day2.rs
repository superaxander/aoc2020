use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i32> {
    let mut valid = 0;

    let lines = common::read_lines("inputs/2.txt")?;
    for line in lines {
        let string = line?;
        let whitespace = string.split_whitespace();
        let split = whitespace.collect::<Vec<&str>>();
        assert_eq!(split.len(), 3);
        let range_split = split[0].split("-").collect::<Vec<&str>>();
        assert_eq!(range_split.len(), 2);
        if let Ok(begin) = range_split[0].parse::<usize>() {
            if let Ok(end) = range_split[1].parse::<usize>() {
                let char = split[1].chars().nth(0).expect("Aaaaah");

                if do_b {
                    let size = split[2].len();
                    if begin < size {
                        if split[2].chars().nth(begin) == Some(char) {
                            if end >= size || split[2].chars().nth(end) != Some(char) {
                                valid += 1;
                            }
                        } else if end < size && split[2].chars().nth(end) == Some(char) {
                            valid += 1;
                        }
                    }
                } else {
                    let mut count = 0;
                    for c in split[2].chars() {
                        if c == char {
                            count += 1;
                        }
                    }

                    if count >= begin && count <= end {
                        valid += 1;
                    }
                }
            }
        }
    }
    return Ok(valid);
}