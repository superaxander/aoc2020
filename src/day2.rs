use std::io;

use crate::common;

pub fn main() -> io::Result<(i32, i32)> {
    let mut valid = 0;
    let mut valid2 = 0;

    let lines = common::read_lines("inputs/2.txt")?;
    for line in lines {
        let string = line?;
        let whitespace = string.split_whitespace();
        let split = whitespace.collect::<Vec<&str>>();
        assert_eq!(split.len(), 3);
        let range_split = split[0].split("-").collect::<Vec<&str>>();
        assert_eq!(range_split.len(), 2);
        if let Ok(begin) = range_split[0].parse::<i32>() {
            if let Ok(end) = range_split[1].parse::<i32>() {
                let char = split[1].chars().nth(0).expect("Aaaaah");
                let mut count = 0;
                let mut index = 1;
                let mut validated = false;
                for c in split[2].chars() {
                    if c == char {
                        if index == begin || index == end {
                            if !validated {
                                valid2 += 1;
                                validated = true;
                            } else {
                                valid2 -= 1;
                            }
                        }
                        count += 1;
                    }
                    index += 1;
                }

                if count >= begin && count <= end {
                    valid += 1;
                }
            }
        }
    }
    return Ok((valid, valid2));
}