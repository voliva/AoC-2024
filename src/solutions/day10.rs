use itertools::Itertools;

use super::Solver;
use crate::coordinate::Coordinate;
use std::collections::HashSet;
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
            .map(|line| {
                line.chars()
                    .map(|v| String::from(v).parse().unwrap())
                    .collect_vec()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut result = 0;
        for r in 0..input.len() {
            for c in 0..input[r].len() {
                if input[r][c] == 0 {
                    let mut reached = HashSet::new();
                    count_trails(input, Coordinate::from_usize(r, c), &mut reached);
                    result += reached.len();
                }
            }
        }
        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut result = 0;
        for r in 0..input.len() {
            for c in 0..input[r].len() {
                if input[r][c] == 0 {
                    let mut reached = HashSet::new();
                    result += count_trails(input, Coordinate::from_usize(r, c), &mut reached);
                }
            }
        }
        Ok(result)
    }
}

fn count_trails(
    map: &Vec<Vec<isize>>,
    start: Coordinate,
    reached: &mut HashSet<Coordinate>,
) -> usize {
    let value = start.apply_vec(map);
    if value.is_none() {
        return 0;
    }
    let value = value.unwrap();
    if *value == 9 {
        reached.insert(start);
        return 1;
    }

    start
        .cardinals()
        .into_iter()
        .map(|c_next| {
            if let Some(next) = c_next.apply_vec(map) {
                if *next == value + 1 {
                    return count_trails(map, c_next, reached);
                }
            }
            return 0;
        })
        .fold(0, |a, b| a + b)
}
