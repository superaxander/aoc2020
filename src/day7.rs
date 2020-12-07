use std::collections::{HashMap, HashSet};
use std::io;

use regex::Regex;

use crate::common;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag {
    color: String,
    count: usize,
}

impl Bag {
    fn new(color: String, count: usize) -> Bag {
        return Bag { color, count };
    }
}


pub fn main(do_b: bool) -> io::Result<usize> {
    let re = Regex::new(r"(\d+) (\w+ \w+) \w+(?:(?:, )|.)").unwrap();

    let mut contained_in: HashMap<String, HashSet<String>> = HashMap::new();
    let mut reverse_map: HashMap<String, HashSet<Bag>> = HashMap::new();

    let lines = common::read_lines("inputs/7.txt")?;
    for line in lines {
        let string = line?;
        //debug!("{}", split[0]);
        let split: Vec<String> = string.split(" bags contain ").map(String::from).collect();
        for caps in re.captures_iter(split[1].as_str()) {
            debug!("{:?}", caps);
            let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let bag = String::from(caps.get(2).unwrap().as_str());
            
            if do_b {
                reverse_map.entry(split[0].clone()).or_insert(HashSet::new()).insert(Bag::new(bag.clone(), count));
            } else {
                contained_in.entry(bag.clone()).or_insert(HashSet::new()).insert(split[0].clone());
            }
        };
    }
    if do_b {
        let set: Vec<Bag> = reverse_map.iter().filter(|(bag, _)| **bag == "shiny gold").flat_map(|(_, contents)| contents).cloned().collect();
        let count: usize = set.iter().map(|bag| bag.count).sum::<usize>() + do_count(&mut reverse_map, &set, 1);
        Ok(count)
    } else {
        let mut set: HashSet<String> = HashSet::new();
        set.extend(contained_in.get("shiny gold").unwrap().iter().cloned());
        debug!("Gold contained directly in {:?}", set);
        let mut length = set.len();
        loop {
            let cloned = set.clone();
            let empty = HashSet::new();
            set.extend(cloned.iter().flat_map(|x| contained_in.get(x).unwrap_or_else(|| &empty).iter().cloned()));
            if length == set.len() { break; }
            length = set.len();
        }
        Ok(length)
    }
}

fn do_count(reverse_map: &mut HashMap<String, HashSet<Bag>>, set: &Vec<Bag>, multiplier: usize) -> usize {
    let mut count: usize = 0;
    debug!("{:?}, {}", set, multiplier);

    for bag in set {
        let set2: Vec<Bag> = reverse_map.iter().filter(|(bag2, _)| **bag2 == bag.color).flat_map(|(_, contents)| contents).cloned().collect();
        count += multiplier * bag.count * set2.iter().map(|bag2| bag2.count).sum::<usize>();
        count += do_count(reverse_map, &set2, multiplier * bag.count);
    }

    return count;
}

