use std::collections::HashSet;
use std::io;

use crate::common;

#[derive(Debug, Clone)]
struct Instruction {
    operator: String,
    operand: i32,
}

impl Instruction {
    fn new(operator: String, operand: i32) -> Instruction {
        return Instruction { operator, operand };
    }
}

pub fn main(do_b: bool) -> io::Result<i32> {
    let instructions: Vec<Instruction> = common::read_lines("inputs/8.txt")?
        .map(|line| line.unwrap())
        .map(|line| line.split(" ").map(String::from).collect::<Vec<String>>())
        .map(|vec| Instruction::new(vec[0].clone(), vec[1].parse::<i32>().unwrap()))
        .collect();

    if do_b {
        let result = do_run(&instructions);
        if let Err((_, set)) = result {
            for i in set {
                let instruction = instructions.get(i).unwrap();
                if instruction.operator.as_str() == "acc" {
                    continue;
                }
                let result = do_run2(&instructions, i);
                if let Ok(acc) = result {
                    return Ok(acc);
                }
            }
        }

        error!("Solution not found");
        Ok(-1)
    } else {
        let result = do_run(&instructions);
        if let Err((acc, _)) = result {
            return Ok(acc);
        }
        error!("Expected an err results for part a");
        Ok(-1)
    }
}

fn do_run(instructions: &Vec<Instruction>) -> Result<i32, (i32, HashSet<usize>)> {
    let mut set: HashSet<usize> = HashSet::new();
    let mut idx = 0;
    let mut acc = 0;

    while idx < instructions.len() {
        set.insert(idx);
        let instruction = instructions.get(idx).unwrap();
        debug!("Help: {:?}", instruction);
        match instruction.operator.as_str() {
            "nop" => {
                idx += 1;
            }
            "acc" => {
                acc += instruction.operand;
                debug!("{}: Accumulating {} to {}", idx, instruction.operand, acc);
                idx += 1;
            }
            "jmp" => {
                assert!(idx as i32 + instruction.operand >= 0);
                idx = (idx as i32 + instruction.operand) as usize;
                debug!("Jumping {} to {}", instruction.operand, idx);
            }
            _ => {}
        }
        if set.contains(&idx) {
            return Err((acc, set));
        }
    }
    Ok(acc)
}

fn do_run2(instructions: &Vec<Instruction>, changed_idx: usize) -> Result<i32, i32> {
    let mut set: HashSet<usize> = HashSet::new();
    let mut idx = 0;
    let mut acc = 0;

    while idx < instructions.len() {
        set.insert(idx);
        let instruction = instructions.get(idx).unwrap();
        debug!("Help: {:?}", instruction);
        match instruction.operator.as_str() {
            "nop" => {
                if idx == changed_idx {
                    assert!(idx as i32 + instruction.operand >= 0);
                    idx = (idx as i32 + instruction.operand) as usize;
                    debug!("Jumping {} to {}", instruction.operand, idx);
                } else {
                    idx += 1;
                }
            }
            "acc" => {
                acc += instruction.operand;
                debug!("{}: Accumulating {} to {}", idx, instruction.operand, acc);
                idx += 1;
            }
            "jmp" => {
                if idx == changed_idx {
                    idx += 1;
                } else {
                    assert!(idx as i32 + instruction.operand >= 0);
                    idx = (idx as i32 + instruction.operand) as usize;
                    debug!("Jumping {} to {}", instruction.operand, idx);
                }
            }
            _ => {}
        }
        if set.contains(&idx) {
            return Err(acc);
        }
    }
    Ok(acc)
}
