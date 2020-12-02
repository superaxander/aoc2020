use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn main() -> io::Result<(i32, i32)> {
    let mut numbers = HashSet::new();

    let lines = read_lines("inputs/1a.txt")?;
    let mut solution_a = -1;
    let mut solution_b = -1;
    for line in lines {
        if solution_a != -1 && solution_b != -1 {
            return Ok((solution_a, solution_b));
        }
        if let Ok(num) = line?.parse::<i32>() {
            let number = 2020 - num;
            if numbers.contains(&number) {
                solution_a = num * number;
                debug!("{} * {} == {}", num, number, solution_a);
            }
            for number in &numbers {
                let number2 = 2020 - num - *number;
                if numbers.contains(&number2) {
                    solution_b = num * *number * number2;
                    debug!("{} * {} * {} == {}", num, number, number2, solution_b);
                }
            }
            numbers.insert(num);
        }
    }

    return Ok((solution_a, solution_b));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}