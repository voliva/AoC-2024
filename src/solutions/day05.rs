use itertools::Itertools;

use crate::many_to_many::ManyToMany;

use super::Solver;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

#[derive(Debug)]
pub struct Input {
    rules: ManyToMany<usize, usize>,
    updates: Vec<Vec<usize>>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = 0;
        let mut result: Input = Input {
            rules: ManyToMany::new(),
            updates: Vec::new(),
        };
        for line in s.lines() {
            if line == "" {
                part += 1;
            } else if part == 0 {
                let (from, to) = line.split("|").collect_tuple().ok_or("Invalid format")?;
                let from_v: usize = from.parse().or(Err("not parsing"))?;
                let to_v: usize = to.parse().or(Err("not parsing"))?;

                result.rules.insert(from_v, to_v);
            } else {
                result
                    .updates
                    .push(line.split(",").map(|v| v.parse().unwrap()).collect())
            }
        }
        Ok(result)
    }
}

fn update_is_valid(rules: &ManyToMany<usize, usize>, update: &Vec<usize>) -> bool {
    update.iter().enumerate().all(|(i, v)| {
        if i == update.len() - 1 {
            return true;
        }
        update[(i + 1)..].iter().all(|other| {
            let set = rules.outer().get(other);
            let result = set.is_none() || !set.unwrap().contains(v);
            // println!("{:?}, {} -> {result}", set, v);
            return result;
        })
    })
}

fn sort_update(rules: &ManyToMany<usize, usize>, update: &Vec<usize>) -> Vec<usize> {
    let mut result = update.clone();
    result.sort_by(|a, b| {
        if let Some(rule_a) = rules.outer().get(a) {
            if rule_a.contains(b) {
                return Ordering::Greater;
            }
        }
        if let Some(rule_b) = rules.outer().get(b) {
            if rule_b.contains(a) {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });
    result
}

impl Solver for Problem {
    type Input = Input;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let content = file_reader.lines().map(|x| x.unwrap()).join("\n");
        Input::from_str(&content).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .updates
            .iter()
            .filter(|v| update_is_valid(&input.rules, &v))
            .map(|v| v[v.len() / 2])
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .updates
            .iter()
            .filter(|v| !update_is_valid(&input.rules, &v))
            .map(|v| sort_update(&input.rules, v))
            .map(|v| v[v.len() / 2])
            .sum())
    }
}
