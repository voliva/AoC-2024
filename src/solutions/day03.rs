use itertools::Itertools;
use regex::Regex;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

pub enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

// lazy_static::lazy_static! {
//     static ref LINE_RGX: Regex = Regex::new(r"...").unwrap();
// }
impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let regex = Regex::new(r"(?:(mul)\((\d+),(\d+)\))|(do)\(\)|(don't)\(\)").unwrap();

        file_reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|line| {
                regex
                    .captures_iter(&line)
                    .map(|res| {
                        if let Some("mul") = res.get(1).map(|v| v.as_str()) {
                            Instruction::Mul(
                                res.get(2).unwrap().as_str().parse().unwrap(),
                                res.get(3).unwrap().as_str().parse().unwrap(),
                            )
                        } else if res.get(4).is_some() {
                            Instruction::Do
                        } else if res.get(5).is_some() {
                            Instruction::Dont
                        } else {
                            panic!("Unmatched instruction");
                        }
                    })
                    .collect_vec()
            })
            .collect()
    }

    // 181345830
    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .iter()
            .map(|instr| match instr {
                Instruction::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut enabled = true;
        let mut result = 0;
        for instr in input {
            match instr {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Mul(a, b) if enabled => result += a * b,
                _ => {}
            }
        }
        Ok(result)
    }
}
