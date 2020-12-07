use std::io;
use regex::Regex;

use crate::common;
use std::collections::HashMap;

pub fn main(do_b: bool) -> io::Result<i32> {
    let re = Regex::new(r"(\d+) (\w+ \w+ \w+)(?:(?:, )|.)").unwrap();

    let mut contained_in: HashMap<&str, Vec<&str>> = HashMap::new();
    
    let lines = common::read_lines("inputs/7.txt")?;
    for line in lines {
        let string = line?;
        //debug!("{}", split[0]);
        let split: Vec<String> = string.split( " contain ").map(String::from).collect();
        find_bags(&re, &mut contained_in, &split[0], &split[1]);
    }
    
    Ok(0)
}

fn find_bags<'a>(re: &Regex, contained_in: &'a mut HashMap<&'a str, Vec<&'a str>>, current_bag: &'a String, other_bags: &'a String) {
    for caps in re.captures_iter(other_bags.as_str()) {
        debug!("{:?}", caps);
        let count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let bag = caps.get(1).unwrap().as_str();

        contained_in.entry(bag.clone()).or_insert(Vec::new()).push(current_bag.as_str());
    }
}
