use std::io;

use crate::common;

pub fn main(do_b: bool) -> io::Result<i64> {
    let lines = common::read_lines("inputs/3.txt")?;
    let mut map = Vec::new();
    for line in lines {
        let array = line?.chars().map(|char| match char {
            '.' => false,
            '#' => true,
            _ => panic!(),
        }).collect::<Vec<bool>>();
        map.push(array);
    }
    if do_b {
        let mut result: i64 = 1;
        let increments = [(1,1), (3,1), (5,1), (7,1), (1,2)];
        for (x_inc, y_inc) in increments.iter() {
            let mut x = 0;
            let mut trees = 0;

            for y in (0..map.len()).step_by(*y_inc) {
                if map[y][x % map[y].len()] {
                    trees += 1;
                }
                x += *x_inc;
            }

            result *= trees;
        }
        Ok(result)
    } else {
        let mut x = 0;
        let mut trees = 0;

        for y in 0..map.len() {
            if map[y][x % map[y].len()] {
                trees += 1;
            }
            x += 3;
        }

        Ok(trees)
    }
}