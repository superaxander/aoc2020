use std::io;
use crate::common;
use std::ops::Index;

pub fn main(do_b: bool) -> io::Result<i32> {
    let mut max = -1;
    let mut seats: Vec<i32> = (0..(127*8)).collect();
    let lines = common::read_lines("inputs/5.txt")?;
    for line in lines {
        let string = line?;
        let mut row_upper = 127;
        let mut row_lower = 0;
        let mut column_upper = 7;
        let mut column_lower = 0;
        for c in string.chars() {
            match c { 
                'B' => {
                    row_lower = row_upper - (row_upper - row_lower) / 2;
                }
                'F' => {
                    row_upper = row_lower + (row_upper - row_lower) / 2;
                }
                'R' => {
                    column_lower = column_upper - (column_upper - column_lower) / 2;
                }
                'L' => {
                    column_upper = column_lower + (column_upper - column_lower) / 2;
                }
                _ => panic!()
            }
        }
        let seat = row * 8 + column;
        debug!("{} results in {}:{} ({})", string, row_lower, column_lower, seat);
        if do_b {
            seats.remove(seats.iter().position(|&x| x == seat).expect(""));
        }
        if seat > max {
            max = seat;
        }
    }
    if do_b {
        debug!("Remaining seats {:?}", seats);
        for seat in 1..seats.len()-1 {
            if seats[seat-1] + 1 != seats[seat] && seats[seat+1] - 1 != seats[seat] {
                return Ok(seats[seat])
            }
        }
        Ok(-1)   
    } else {
        Ok(max)
    }
} 
