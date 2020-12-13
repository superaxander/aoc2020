use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<u64> {
    let lines: Vec<String> = common::read_lines("inputs/13.txt")?
        .map(|x| x.unwrap())
        .collect();

    if do_b {
        let busses: Vec<(u64, u64)> = lines[1]
            .split(",")
            .enumerate()
            .filter(|(_, x)| *x != "x")
            .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
            .collect();
        let first = busses.first().unwrap();
        let (_, first_id) = first;
        let mut current_bus = 1;
        let mut step_size = *first_id;
        let mut t = 0;
        while current_bus < busses.len() {
            t += step_size;

            let (idx, id) = busses[current_bus];
            if (t + idx) % id == 0 {
                debug!("Found at {} + {}", t, idx);
                current_bus += 1;
                step_size *= id;
                continue;
            }
        }
        Ok(t)
    } else {
        let start = lines[0].parse::<u64>().unwrap();
        let busses = lines[1]
            .split(",")
            .filter(|x| *x != "x")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for t in start..u64::MAX {
            for id in &busses {
                if t % id == 0 {
                    return Ok(id * (t - start));
                }
            }
        }
        Ok(0) // Fail
    }
}
