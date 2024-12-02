use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

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
        Ok(input
            .iter()
            .map(|v| {
                if v.len() <= 1 {
                    return 1;
                };
                let mut prev = v[0];
                let is_increase = v[1] > v[0];
                for k in &v[1..] {
                    if (k - prev).abs() > 3 || *k == prev {
                        return 0;
                    }
                    if is_increase && *k < prev {
                        return 0;
                    } else if !is_increase && *k > prev {
                        return 0;
                    }
                    prev = *k;
                }
                return 1;
            })
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .iter()
            .map(|v| {
                if v.len() <= 1 {
                    return 1;
                };

                let diffs: Vec<_> = v.into_iter().tuple_windows().map(|(a, b)| a - b).collect();

                let positives = diffs.iter().filter(|v| **v > 0 && **v <= 3).count();
                let negatives = diffs.iter().filter(|v| **v < 0 && **v >= -3).count();

                if positives == diffs.len() {
                    return 1;
                }
                if positives >= diffs.len() - 2 {
                    let (miss_idx, _) =
                        diffs.iter().find_position(|v| **v <= 0 || **v > 3).unwrap();
                    let positives_a = v
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != miss_idx)
                        .map(|(_, a)| a)
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .filter(|v| *v > 0 && *v <= 3)
                        .count();
                    let positives_b = v
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != miss_idx + 1)
                        .map(|(_, a)| a)
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .filter(|v| *v > 0 && *v <= 3)
                        .count();
                    if positives_a == diffs.len() - 1 || positives_b == diffs.len() - 1 {
                        return 1;
                    } else {
                        return 0;
                    }
                }

                if negatives == diffs.len() {
                    return 1;
                }
                if negatives >= diffs.len() - 2 {
                    let (miss_idx, _) = diffs
                        .iter()
                        .find_position(|v| **v >= 0 || **v < -3)
                        .unwrap();
                    let tmp = v
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != miss_idx)
                        .map(|(_, a)| a)
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .collect_vec();
                    let negatives_a = tmp.iter().filter(|v| **v < 0 && **v >= -3).count();
                    let negatives_b = v
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != miss_idx + 1)
                        .map(|(_, a)| a)
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .filter(|v| *v < 0 && *v >= -3)
                        .count();
                    if negatives_a == diffs.len() - 1 || negatives_b == diffs.len() - 1 {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                return 0;
            })
            .sum())
    }
}
