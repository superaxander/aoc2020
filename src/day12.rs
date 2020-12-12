use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i32> {
    let lines = common::read_lines("inputs/12.txt")?;
    let mut way_x = 10;
    let mut way_y = -1;
    let mut x = 0;
    let mut y = 0;
    let mut facing = 0; // 0: east, 1: south 2: west 3:north
    for line in lines {
        let string = line?;
        let num: i32 = string[1..].parse().unwrap();
        if do_b {
            match string.chars().nth(0).unwrap() {
                'N' => way_y -= num,
                'S' => way_y += num,
                'E' => way_x += num,
                'W' => way_x -= num,
                'L' => match num {
                    90 => {
                        let temp = way_x;
                        way_x = way_y;
                        way_y = -temp;
                    }
                    180 => {
                        way_x = -way_x;
                        way_y = -way_y;
                    }
                    270 => {
                        let temp = way_x;
                        way_x = -way_y;
                        way_y = temp;
                    }
                    _ => {}
                },
                'R' => match num {
                    270 => {
                        let temp = way_x;
                        way_x = way_y;
                        way_y = -temp;
                    }
                    180 => {
                        way_x = -way_x;
                        way_y = -way_y;
                    }
                    90 => {
                        let temp = way_x;
                        way_x = -way_y;
                        way_y = temp;
                    }
                    _ => {}
                },
                // f
                _ => {
                    x += num * way_x;
                    y += num * way_y;
                }
            }
        } else {
            match string.chars().nth(0).unwrap() {
                'N' => y -= num,
                'S' => y += num,
                'E' => x += num,
                'W' => x -= num,
                'L' => facing = (4 + facing - (num / 90)) % 4,
                'R' => facing = (facing + (num / 90)) % 4,
                // f
                _ => match facing {
                    0 => x += num,
                    1 => y += num,
                    2 => x -= num,
                    _ => y -= num, // 3
                },
            }
        }
    }

    Ok(x.abs() + y.abs())
}
