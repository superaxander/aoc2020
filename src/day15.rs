use std::collections::HashMap;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut history: HashMap<usize, usize> = common::read_lines("inputs/15.txt")?
        .map(|x| x.unwrap())
        .nth(0)
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(i, x)| (x.parse::<usize>().unwrap(), i))
        .collect();

    debug!("{:?}", history);
    let mut last = *history.iter().max_by_key(|(_, i)| *i).unwrap().0;
    history.remove(&last);
    let max;
    if do_b {
        max = 30000000;
    } else {
        max = 2020;
    }

    for i in (history.len() + 1)..max {
        debug!("Last {}: {}", i, last);
        if history.contains_key(&last) {
            let new = i - history.get(&last).unwrap() - 1;
            debug!("New: {} - {} - 1", i, history.get(&last).unwrap());
            history.insert(last, i - 1);
            last = new;
        } else {
            history.insert(last, i - 1);
            last = 0;
        }
    }
    Ok(last)
}
