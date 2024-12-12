use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output1 = usize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|line| line.split(" ").map(|v| v.parse().unwrap()).collect_vec())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok((0..25).fold(input.clone(), |acc, _| evolve(acc)).len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut rock_results: HashMap<(isize, isize), isize> = HashMap::new();

        Ok(input
            .iter()
            .map(|v| find_result(*v, 75, &mut rock_results))
            .sum())
    }
}

fn find_result(
    rock: isize,
    depth: isize,
    rock_results: &mut HashMap<(isize, isize), isize>,
) -> isize {
    if depth == 0 {
        return 1;
    }
    if rock_results.contains_key(&(rock, depth)) {
        return *rock_results.get(&(rock, depth)).unwrap();
    }
    let evolved = evolve_single(rock);
    let result = evolved
        .iter()
        .map(|next| find_result(*next, depth - 1, rock_results))
        .sum();
    rock_results.insert((rock, depth), result);

    return result;
}

fn evolve_single(value: isize) -> Vec<isize> {
    if value == 0 {
        return vec![1];
    }
    let formatted = format!("{value}");
    let len = formatted.len();
    if len % 2 == 0 {
        return vec![
            (&formatted[..len / 2]).parse().unwrap(),
            (&formatted[len / 2..]).parse().unwrap(),
        ];
    }
    vec![value * 2024]
}
fn evolve(value: Vec<isize>) -> Vec<isize> {
    value.into_iter().flat_map(evolve_single).collect()
}
