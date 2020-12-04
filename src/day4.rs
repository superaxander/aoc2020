use std::collections::{HashMap, HashSet};
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i32> {
    struct B {
        f: fn(&String) -> bool,
    }

    impl B {
        fn new(f: fn(&String) -> bool) -> B {
            B { f: f }
        }
    }

    impl Clone for B {
        fn clone(&self) -> Self {
            return B::new(self.f);
        }
    }

    let required_set: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().map(String::from).collect();


    let required_map: HashMap<String, B> = [("byr", B::new(|byr: &String| -> bool {
        let num = byr.parse::<i32>();
        if let Ok(year) = num {
            return year >= 1920 && year <= 2002;
        }
        return false;
    })), ("iyr", B::new(|iyr: &String| -> bool {
        let num = iyr.parse::<i32>();
        if let Ok(year) = num {
            return year >= 2010 && year <= 2020;
        }
        return false;
    })), ("eyr", B::new(|eyr: &String| -> bool {
        let num = eyr.parse::<i32>();
        if let Ok(year) = num {
            return year >= 2020 && year <= 2030;
        }
        return false;
    })), ("hgt", B::new(|hgt: &String| -> bool {
        let num = hgt[0..hgt.len() - 2].parse::<i32>();
        if let Ok(height) = num {
            return match &hgt[hgt.len() - 2..hgt.len()] {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                _ => false
            };
        }
        return false;
    })), ("hcl", B::new(|hcl: &String| -> bool {
        return hcl.len() == 7 && hcl.chars().nth(0).expect("") == '#' && hcl[1..6].chars().all(|c| {
            return match c {
                '0'..='9' | 'a'..='f' => true,
                _ => false
            };
        });
    })), ("ecl", B::new(|ecl: &String| -> bool {
        return match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        };
    })), ("pid", B::new(|pid: &String| -> bool {
        pid.len() == 9 && pid.chars().all(|c| {
            return match c {
                '0'..='9' | 'a'..='f' => true,
                _ => false
            };
        })
    }))].iter().cloned().map(|(a, b)| (String::from(a.clone()), b)).collect();


    let mut valid = 0;
    let lines = common::read_lines("inputs/4.txt")?;
    let mut set: HashSet<String> = HashSet::new();
    for line in lines {
        let string = line?;
        let whitespace: Vec<String> = string.split_whitespace().map(String::from).collect();
        if whitespace.len() == 0 {
            if set.intersection(&required_set).count() >= required_set.len() {
                debug!("Accepted {:?}", set);
                valid += 1;
            } else {
                debug!("Rejected {:?}", set);
            }
            set.clear()
        } else {
            for field in whitespace {
                let field_split: Vec<String> = field.split(":").map(String::from).collect();
                let name = field_split.get(0).expect("").clone();
                let value = field_split.get(1).expect("").clone();

                if do_b {
                    if required_map.contains_key(name.as_str()) {
                        if (required_map[name.as_str()].f)(&value) {
                            debug!("Accepted {}: {}", name, value);
                            set.insert(name);
                        } else {
                            debug!("Rejected {}: {}", name, value);
                        }
                    } else if name.as_str() != "cid" {
                        debug!("Unknown {}: {}", name, value);
                    }
                } else {
                    set.insert(name);
                }
            }
        }
    }

    if set.intersection(&required_set).count() >= required_set.len() {
        valid += 1;
    }

    Ok(valid)
}