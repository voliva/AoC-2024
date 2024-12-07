use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

pub struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl Equation {
    fn is_calibrated(&self) -> bool {
        if self.values.len() == 1 {
            return self.values[0] == self.result;
        }
        let last = *self.values.last().unwrap();
        if self.result % last == 0 {
            let inner = Equation {
                result: self.result / last,
                values: Vec::from(&self.values[..self.values.len() - 1]),
            };
            if inner.is_calibrated() {
                return true;
            }
        }
        if self.result >= last {
            let inner = Equation {
                result: self.result - last,
                values: Vec::from(&self.values[..self.values.len() - 1]),
            };
            if inner.is_calibrated() {
                return true;
            }
        }
        return false;
    }

    fn is_calibrated_concat(&self) -> bool {
        if self.values.len() == 1 {
            return self.values[0] == self.result;
        }
        let last = *self.values.last().unwrap();
        let values = Vec::from(&self.values[..self.values.len() - 1]);
        if self.result % last == 0 {
            let inner = Equation {
                result: self.result / last,
                values: values.clone(),
            };
            if inner.is_calibrated_concat() {
                return true;
            }
        }
        if self.result >= last {
            let inner = Equation {
                result: self.result - last,
                values: values.clone(),
            };
            if inner.is_calibrated_concat() {
                return true;
            }
        }
        let digits = format!("{last}").len();
        let b10 = (10_u64).pow(digits as u32) as usize;
        if self.result % b10 == last {
            let inner = Equation {
                result: self.result / b10,
                values: values.clone(),
            };
            if inner.is_calibrated_concat() {
                return true;
            }
        }

        return false;
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (res, values) = s.split(": ").collect_tuple().unwrap();

        Ok(Equation {
            result: res.parse().unwrap(),
            values: values.split(" ").map(|v| v.parse().unwrap()).collect(),
        })
    }
}

impl Solver for Problem {
    type Input = Vec<Equation>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .iter()
            .filter(|v| v.is_calibrated())
            .map(|v| v.result)
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .iter()
            .filter(|v| v.is_calibrated_concat())
            .map(|v| v.result)
            .sum())
    }
}
