use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

pub struct Problem;

fn starts_with(design: &[char], pattern: &Vec<char>) -> bool {
    if pattern.len() > design.len() {
        false
    } else {
        zip(design, pattern).all(|(d, p)| d == p)
    }
}
fn find_design(design: &[char], patterns: &Vec<Vec<char>>) -> Option<Vec<usize>> {
    if design.len() == 0 {
        return Some(vec![]);
    }
    for (i, p) in patterns.iter().enumerate() {
        if starts_with(design, p) {
            if let Some(mut v) = find_design(&design[p.len()..], patterns) {
                v.push(i);
                return Some(v);
            }
        }
    }
    None
}

fn count_designs(
    design: &[char],
    patterns: &Vec<Vec<char>>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let key = String::from_iter(design);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    if design.len() == 0 {
        return 1;
    }
    let result = patterns
        .iter()
        .map(|p| {
            if starts_with(design, p) {
                count_designs(&design[p.len()..], patterns, cache)
            } else {
                0
            }
        })
        .sum();
    cache.insert(key, result);

    result
}

impl Solver for Problem {
    type Input = (Vec<Vec<char>>, Vec<Vec<char>>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let patterns = lines[0]
            .split(", ")
            .map(|p| p.chars().collect_vec())
            .collect_vec();
        let designs = (&lines[2..])
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        (patterns, designs)
    }

    fn solve_first(&self, (patterns, designs): &Self::Input) -> Result<Self::Output1, String> {
        Ok(designs
            .iter()
            .filter(|d| find_design(&d[..], patterns).is_some())
            .count())
    }

    fn solve_second(&self, (patterns, designs): &Self::Input) -> Result<Self::Output2, String> {
        let mut cache = HashMap::new();

        Ok(designs
            .iter()
            .map(|d| count_designs(&d[..], patterns, &mut cache))
            .sum())
    }
}
