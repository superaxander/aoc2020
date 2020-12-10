use std::collections::LinkedList;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut nums: Vec<usize> = common::read_lines("inputs/10.txt")?
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();

    nums.sort_unstable_by_key(|x| -(*x as i32));
    if do_b {
        let mut table = Vec::<usize>::with_capacity(*nums.first().unwrap() as usize + 1);
        for _ in 0..*nums.first().unwrap() + 1 {
            table.push(0);
        }
        table[0] = 1;
        nums.push(0);
        while !nums.is_empty() {
            let min = nums.pop().unwrap();
            if nums.is_empty() {
                break;
            }
            let jump_1 = nums[nums.len() - 1];
            table[jump_1] += table[min];
            match jump_1 - min {
                1 => {
                    if nums.len() == 1 {
                        continue;
                    }
                    let jump_2 = nums[nums.len() - 2];
                    match jump_2 - min {
                        2 => {
                            table[jump_2] += table[min];
                            if nums.len() == 2 {
                                continue;
                            }
                            let jump_3 = nums[nums.len() - 3];
                            if jump_3 - min == 3 {
                                table[jump_3] += table[min];
                            }
                        }
                        3 => {
                            table[jump_2] += table[min];
                        }
                        _ => {
                            // Jump too far
                            continue;
                        }
                    }
                }
                2 => {
                    if nums.len() == 1 {
                        continue;
                    }
                    let jump_2 = nums[nums.len() - 2];
                    if jump_2 - min == 3 {
                        table[jump_2] += table[min];
                    }
                }
                3 => {}
                _ => panic!(),
            }
        }
        debug!("{:?}", table);
        Ok(*table.last().unwrap())
    } else {
        let mut jolt_1 = 0;
        let mut jolt_3 = 1;
        let mut current = 0;
        while !nums.is_empty() {
            let min = nums.pop().unwrap();
            let diff = min - current;
            match diff {
                0 => panic!("Duplicates????"),
                1 => jolt_1 += 1,
                2 => panic!("Jump of 2?"),
                3 => jolt_3 += 1,
                _ => panic!("Bigger jump of {}?!", diff),
            }
            current = min;
        }

        Ok(jolt_1 * jolt_3)
    }
}
