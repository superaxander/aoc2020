use std::fmt::{Debug, Formatter};
use std::{cmp, io};

use crate::common;
use crate::day11::State::{EmptySeat, Floor, OccupiedSeat};

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Floor => ".",
            EmptySeat => "L",
            OccupiedSeat => "#",
        })?;
        return Ok(());
    }
}

pub fn main(do_b: bool) -> io::Result<usize> {
    let mut valid = 0;

    let mut map: Vec<Vec<State>> = common::read_lines("inputs/11.txt")?
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| match c {
                    'L' => EmptySeat,
                    '.' => Floor,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let mut map2: Vec<Vec<State>> = Vec::new();
    let mut changed = false;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    loop {
        map2.clone_from(&map);
        for i in 0..height {
            for j in 0..width {
                let mut occupied_neighbours = 0;
                if map2[i as usize][j as usize] != Floor && do_b {
                    // left
                    for k in 1..j + 1 {
                        match map2[i as usize][(j - k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // right
                    for k in 1..width - j {
                        match map2[i as usize][(j + k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // up
                    for k in 1..i + 1 {
                        match map2[(i - k) as usize][j as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // down
                    for k in 1..height - i {
                        match map2[(i + k) as usize][j as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // left-up
                    for k in 1..cmp::min(i + 1, j + 1) {
                        match map2[(i - k) as usize][(j - k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // right-down
                    for k in 1..cmp::min(width - j, height - i) {
                        match map2[(i + k) as usize][(j + k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // right-up
                    for k in 1..cmp::min(i + 1, width - j) {
                        match map2[(i - k) as usize][(j + k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }

                    // left-down
                    for k in 1..cmp::min(j + 1, height - i) {
                        match map2[(i + k) as usize][(j - k) as usize] {
                            Floor => {}
                            EmptySeat => break,
                            OccupiedSeat => {
                                occupied_neighbours += 1;
                                break;
                            }
                        }
                    }
                } else {
                    for k in (-1)..=1 {
                        for l in (-1)..=1 {
                            if k == 0 && l == 0 {
                                continue;
                            }
                            if i + k >= 0 && j + l >= 0 && i + k < height && j + l < width {
                                let neighbour = &map2[(i + k) as usize][(j + l) as usize];
                                if *neighbour == OccupiedSeat {
                                    occupied_neighbours += 1;
                                }
                            }
                        }
                    }
                }
                // debug!("Found {} neighbours at {},{}", occupied_neighbours, i, j);
                let vec = &mut map[i as usize];
                vec[j as usize] = match vec[j as usize] {
                    Floor => Floor,
                    EmptySeat => {
                        if occupied_neighbours == 0 {
                            changed = true;
                            OccupiedSeat
                        } else {
                            EmptySeat
                        }
                    }
                    OccupiedSeat => {
                        if (do_b && occupied_neighbours >= 5) || (!do_b && occupied_neighbours >= 4)
                        {
                            changed = true;
                            EmptySeat
                        } else {
                            OccupiedSeat
                        }
                    }
                }
            }
        }
        // debug!("Map:");
        // for v in &map {
        //     debug!("{:?}", v);
        // }
        if !changed {
            return Ok(map
                .iter()
                .flat_map(|x| x)
                .filter(|x| **x == OccupiedSeat)
                .count());
        }
        changed = false;
    }
}
