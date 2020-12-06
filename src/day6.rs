use std::collections::HashSet;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut count = 0;
    let mut set: HashSet<char> = HashSet::new();
    let lines = common::read_lines("inputs/6.txt")?;
    let mut unset = true;
    for line in lines {
        let string = line?;

        if string.is_empty() {
            count += set.len();
            set.clear();
            unset = true;
        } else {
            let mut personal_set = HashSet::new();
            string.chars().for_each(|c| {
                personal_set.insert(c);
                //debug!("Adding {}, count is now: {}", c, personal_set.len())
            });
            if !do_b || unset {
                unset = false;
                set.extend(personal_set);
                debug!("New set: {:?}", set)
            } else {
                set.retain(|c| personal_set.contains(c));
                debug!("Remaining set: {:?}", set)
            }
        }
    }

    count += set.len();

    Ok(count)
}
