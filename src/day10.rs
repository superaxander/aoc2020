use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut nums: Vec<usize> = common::read_lines("inputs/10.txt")?
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();

    nums.sort_unstable_by_key(|x| -(*x as i32));
    if do_b {
        let mut table = Vec::<usize>::with_capacity(*nums.first().unwrap() as usize + 1); // TODO: Reduce size to 4
        for _ in 0..*nums.first().unwrap() + 1 {
            table.push(0);
        }
        table[0] = 1;
        nums.push(0);
        loop {
            let min = nums.pop().unwrap();
            let length = nums.len();
            if length == 0 {
                break;
            }
            let jump = nums[nums.len() - 1];
            table[jump] += table[min];
            if length == 1 {
                break;
            }
            match jump - min {
                1 => {
                    let jump = nums[nums.len() - 2];
                    match jump - min {
                        2 => {
                            table[jump] += table[min];
                            if length == 2 {
                                continue;
                            }
                            let jump = nums[length - 3];
                            if jump - min == 3 {
                                table[jump] += table[min];
                            }
                        }
                        3 => {
                            table[jump] += table[min];
                        }
                        _ => {
                            // Jump too far
                            continue;
                        }
                    }
                }
                2 => {
                    let jump = nums[length - 2];
                    if jump - min == 3 {
                        table[jump] += table[min];
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
                1 => jolt_1 += 1,
                3 => jolt_3 += 1,
                _ => panic!("Not expecting jump of {}?!", diff),
            }
            current = min;
        }

        Ok(jolt_1 * jolt_3)
    }
}
