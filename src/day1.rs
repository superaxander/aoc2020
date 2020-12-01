use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn main() -> io::Result<(i32, i32)> {
    let mut numbers = Vec::new();

    let lines = read_lines("inputs/1a.txt")?;
    let mut solution_a = -1;
    let mut solution_b = -1;
    for line in lines {
        if let Ok(num) = line?.parse::<i32>() {
            for number in &numbers {
                if num + *number == 2020 {
                    solution_a = num * *number;
                    debug!("{} * {} == {}", num, *number, solution_a);
                } else if num + *number < 2020 {
                    for number2 in &numbers {
                        if num + *number + *number2 == 2020 {
                            solution_b = num * *number * *number2;
                            debug!("{} * {} * {} == {}", num, number, number2, solution_b);
                        }
                    }
                }
            }
            numbers.push(num);
        }
    }

    return Ok((solution_a, solution_b));

    // if let Ok(lines) = read_lines("inputs/1a.txt") {
    //     for line in lines {
    //         if let Ok(ip) = line {
    //             if let Ok(num) = ip.parse::<i32>() {
    //                 for number in &numbers {
    //                     if num + *number == 2020 {
    //                         println!("{} * {} == {}", num, *number, num * *number);
    //                     } else if num + *number < 2020 {
    //                         for number2 in &numbers {
    //                             if num + *number + *number2 == 2020 {
    //                                 println!("{} * {} * {} == {}", num, number, number2, num * *number * *number2);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 numbers.push(num);
    //             }
    //         }
    //     }
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}