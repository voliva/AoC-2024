use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn is_safe(line: &Vec<isize>) -> bool {
    let diffs: Vec<_> = line
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect();

    let positives = diffs.iter().filter(|v| **v > 0 && **v <= 3).count();
    if positives == diffs.len() {
        return true;
    }

    let negatives = diffs.iter().filter(|v| **v < 0 && **v >= -3).count();
    return negatives == diffs.len();
}

impl Solver for Problem {
    type Input = Vec<Vec<isize>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split(" ").map(|v| v.parse().unwrap()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().filter(|line| is_safe(*line)).count())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .iter()
            .filter(|line| {
                if is_safe(*line) {
                    return true;
                }

                return (0..line.len()).any(|idx| {
                    is_safe(
                        &line
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != idx)
                            .map(|(_, v)| *v)
                            .collect(),
                    )
                });
            })
            .count())
    }
}
