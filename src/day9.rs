use std::collections::{HashSet, LinkedList};
use std::io::Error;
use std::{cmp, io};

use crate::common;

const PREAMBLE_SIZE: usize = 25;

pub fn main() -> io::Result<(usize, usize)> {
    return match main_a() {
        Ok(res_a) => match main_b(res_a) {
            Ok(res_b) => Ok((res_a, res_b)),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    };
}

pub fn main_a() -> io::Result<usize> {
    let lines = common::read_lines("inputs/9.txt")?;
    let mut nums: [usize; PREAMBLE_SIZE] = [0; PREAMBLE_SIZE];
    let mut idx = 0;
    for line in lines {
        let string = line?;
        let num = string.parse::<usize>().unwrap();
        let mut found = false;
        if idx >= PREAMBLE_SIZE {
            'outer: for i in 0..PREAMBLE_SIZE {
                let search_num = (num as i64 - nums[i] as i64);
                if search_num < 0 {
                    continue;
                }
                for j in i..PREAMBLE_SIZE {
                    if nums[j] == search_num as usize {
                        // debug!(
                        //     "Found: {}({})+{}({})={}({})",
                        //     nums[i], i, search_num, j, num, idx
                        // );
                        found = true;
                        break 'outer;
                    }
                }
            }
            if !found {
                debug!("{:?}", nums);
                return Ok(num);
            }
        }
        nums[idx % PREAMBLE_SIZE] = num;
        idx += 1;
    }
    Ok(0)
}

pub fn main_b(res_a: usize) -> io::Result<usize> {
    let lines = common::read_lines("inputs/9.txt")?;
    let mut nums: LinkedList<usize> = LinkedList::new();
    let mut sum = 0;
    for line in lines {
        let string = line?;
        let num = string.parse::<usize>().unwrap(); // Even though I reload it's still plenty fast
                                                    // debug!("{:?} + {}", nums, num);
        if sum + num == res_a {
            return Ok(nums.iter().max().unwrap() + nums.iter().min().unwrap());
        }
        while sum > 0 && sum + num > res_a {
            sum -= nums.pop_front().unwrap();
            if sum > 0 && sum + num == res_a {
                return Ok(nums.iter().max().unwrap() + nums.iter().min().unwrap());
            }
        }
        nums.push_back(num);
        sum += num;
    }
    Ok(0)
}
