use std::collections::HashSet;
use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i32> {
    struct B {
        f: fn(&String) -> bool,
    }

    impl B {
        fn new(f: fn(&String) -> bool) -> B {
            B { f }
        }
    }

    impl Clone for B {
        fn clone(&self) -> Self {
            return B::new(self.f);
        }
    }

    let required_set: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let required_map: [B; 7] = [
        B::new(|byr: &String| -> bool {
            let num = byr.parse::<i32>();
            if let Ok(year) = num {
                return year >= 1920 && year <= 2002;
            }
            return false;
        }),
        B::new(|iyr: &String| -> bool {
            let num = iyr.parse::<i32>();
            if let Ok(year) = num {
                return year >= 2010 && year <= 2020;
            }
            return false;
        }),
        B::new(|eyr: &String| -> bool {
            let num = eyr.parse::<i32>();
            if let Ok(year) = num {
                return year >= 2020 && year <= 2030;
            }
            return false;
        }),
        B::new(|hgt: &String| -> bool {
            let num = hgt[0..hgt.len() - 2].parse::<i32>();
            if let Ok(height) = num {
                return match &hgt[hgt.len() - 2..hgt.len()] {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                };
            }
            return false;
        }),
        B::new(|hcl: &String| -> bool {
            return hcl.len() == 7
                && hcl.chars().nth(0).expect("") == '#'
                && hcl[1..6].chars().all(|c| {
                    return match c {
                        '0'..='9' | 'a'..='f' => true,
                        _ => false,
                    };
                });
        }),
        B::new(|ecl: &String| -> bool {
            return match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };
        }),
        B::new(|pid: &String| -> bool {
            pid.len() == 9
                && pid.chars().all(|c| {
                    return match c {
                        '0'..='9' | 'a'..='f' => true,
                        _ => false,
                    };
                })
        }),
    ];

    let mut valid = 0;
    let lines = common::read_lines("inputs/4.txt")?;
    let mut set: HashSet<String> = HashSet::new();
    for line in lines {
        let string = line?;
        let whitespace: Vec<String> = string.split_whitespace().map(String::from).collect();
        if whitespace.len() == 0 {
            if intersection(&set, &required_set) >= required_set.len() {
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
                    if let Some(index) = required_set.iter().position(|x| *x == name.as_str()) {
                        if (required_map[index].f)(&value) {
                            debug!("Accepted {}: {}", name, value);
                            set.insert(name);
                        } else {
                            debug!("Rejected {}: {}", name, value);
                        }
                    }
                } else {
                    set.insert(name);
                }
            }
        }
    }

    if intersection(&set, &required_set) >= required_set.len() {
        valid += 1;
    }

    Ok(valid)
}

fn intersection(set: &HashSet<String>, required_set: &[&str; 7]) -> usize {
    let mut size = 0;
    for field in required_set {
        if set.contains(*field) {
            size += 1;
        }
    }
    return size;
}
