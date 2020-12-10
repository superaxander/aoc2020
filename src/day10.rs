use std::io;
// use std::time::Instant;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut nums: Vec<usize> = common::read_lines("inputs/10.txt")?
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();
    // let start = Instant::now();
    if do_b {
        nums.sort_unstable();
        let len = nums.len();
        let max = *nums.last().unwrap() + 1;
        let mut table: Vec<usize> = vec![0; max];
        table[0] = 1;
        let mut last = 0;
        for idx in 0..len {
            match len - idx {
                1 => {
                    // No need to check always possible
                    let jump = nums[idx];
                    table[jump] += table[last];
                }
                2 => {
                    // No need to check always possible
                    let jump = nums[idx];
                    table[jump] += table[last];
                    let jump_2 = nums[idx + 1];
                    table[jump_2] += (jump_2 - last <= 3) as usize * table[last];
                }
                3 => {
                    let jump = nums[idx];
                    table[jump] += table[last];
                    let jump_2 = nums[idx + 1];
                    let jump_3 = nums[idx + 2];
                    table[jump_2] += (jump_2 - last <= 3) as usize * table[last];
                    table[jump_3] += (jump_3 - last <= 3) as usize * table[last];
                }
                _ => {
                    let jump_1 = nums[idx];
                    let jump_2 = nums[idx + 1];
                    let jump_3 = nums[idx + 2];
                    table[jump_1] += table[last];
                    table[jump_2] += (jump_2 - last <= 3) as usize * table[last];
                    table[jump_3] += (jump_3 - last <= 3) as usize * table[last];
                }
            }
            last = nums[idx];
            debug!("{}", idx);
        }
        // info!("{:#?}", start.elapsed());
        debug!("{:?}", table);
        Ok(*table.last().unwrap())
    } else {
        nums.sort_unstable_by_key(|x| -(*x as i32));
        let mut jolt_1 = 0;
        let mut jolt_3 = 1;
        let mut current = 0;
        while !nums.is_empty() {
            let min = nums.pop().unwrap();
            let diff = min - current;
            match diff {
                1 => jolt_1 += 1,
                _ => jolt_3 += 1, // 3
                                  //_ => panic!("Not expecting jump of {}?!", diff),
            }
            current = min;
        }

        Ok(jolt_1 * jolt_3)
    }
}
