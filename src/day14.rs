use std::collections::HashMap;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<u64> {
    let lines: Vec<String> = common::read_lines("inputs/14.txt")?
        .map(|x| x.unwrap())
        .collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;
    let mut mask: String = "".to_string();

    for line in lines {
        if line.starts_with("mask = ") {
            if do_b {
                mask = String::from(&line[7..]);
            } else {
                and_mask = line[7..]
                    .chars()
                    .map(|x| match x {
                        '0' => 0u64,
                        _ => 1u64,
                    })
                    .fold(0, |n, x| (n << 1) | x);
                debug!("and: {:b}", and_mask);
                or_mask = line[7..]
                    .chars()
                    .map(|x| match x {
                        '1' => 1u64,
                        _ => 0u64,
                    })
                    .fold(0, |n, x| (n << 1) | x);
                debug!("or: {:b}", or_mask);
            }
        } else {
            let open_idx = line.chars().position(|x| x == '[').unwrap();
            let close_idx = line.chars().position(|x| x == ']').unwrap();
            let address = line[(open_idx + 1)..close_idx].parse::<u64>().unwrap();
            let num = line[close_idx + 4..].parse::<u64>().unwrap();
            // debug!("{} interpreted as mem[{}] = {}", line, address, num);
            if do_b {
                let mut addresses: Vec<u64> = Vec::new();
                addresses.push(0);
                // debug!("mask: {}", mask);
                mask.chars().enumerate().for_each(|(i, c)| match c {
                    '0' => {
                        for a in &mut addresses {
                            *a = (*a << 1) | ((address & (1 << (35 - i))) >> (35 - i));
                        }
                    }
                    '1' => {
                        for a in &mut addresses {
                            *a = (*a << 1) | 1;
                        }
                    }
                    _ => {
                        // 'X'
                        let mut to_be_added: Vec<u64> = Vec::new();
                        for a in &mut addresses {
                            *a = *a << 1;
                            to_be_added.push(*a | 1);
                        }
                        for a in to_be_added {
                            addresses.push(a);
                        }
                    }
                });
                // debug!("Address {} became: {:?}", address, addresses);
                for a in addresses {
                    memory.insert(a, num);
                }
            } else {
                memory.insert(address, (num & and_mask) | or_mask);
            }
        }
    }
    Ok(memory.values().sum())
}
