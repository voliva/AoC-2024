use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn next_secret(secret: isize) -> isize {
    let secret = (secret * 64 ^ secret) % 16777216;
    let secret = (secret / 32 ^ secret) % 16777216;
    (secret * 2048 ^ secret) % 16777216
}
fn nth_secret(secret: isize, n: isize) -> isize {
    let mut secret = secret;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

type Sequence = (isize, isize, isize, isize);
fn sequences(secret: isize) -> HashMap<Sequence, isize> {
    let mut result = HashMap::new();
    let mut secrets = vec![secret];
    let mut secret = secret;
    for _ in 0..2000 {
        secret = next_secret(secret);
        secrets.push(secret);
    }
    let diffs = secrets
        .iter()
        .map(|v| v % 10)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    for (i, key) in diffs.into_iter().tuple_windows::<Sequence>().enumerate() {
        if !result.contains_key(&key) {
            let bananas = secrets[i + 4] % 10;
            result.insert(key, bananas);
        }
    }

    result
}

fn join_sequences(sequences: Vec<HashMap<Sequence, isize>>) -> HashMap<Sequence, isize> {
    let mut result = HashMap::new();

    for seq in sequences {
        for (k, v) in seq {
            let value = result.get(&k).unwrap_or(&0) + v;
            result.insert(k, value);
        }
    }

    result
}

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|v| nth_secret(*v, 2000)).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let sequences = input.iter().map(|v| sequences(*v)).collect_vec();
        let joined = join_sequences(sequences);

        let winning = joined.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

        // 9245
        Ok(*winning.1)
    }
}
