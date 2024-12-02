use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<isize>, Vec<isize>);
    type Output1 = isize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines: Vec<(isize, isize)> = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.split("   ").map(|v| v.parse().unwrap()).collect())
            .map(|v: Vec<isize>| (v[0], v[1]))
            .collect();
        let list_a = lines.iter().map(|(a, _)| *a).collect();
        let list_b = lines.iter().map(|(_, b)| *b).collect();
        (list_a, list_b)
    }

    fn solve_first(&self, (a, b): &Self::Input) -> Result<Self::Output1, String> {
        let mut a_sorted = a.clone();
        a_sorted.sort();
        let mut b_sorted = b.clone();
        b_sorted.sort();

        Ok(zip(a_sorted, b_sorted).map(|(a, b)| (a - b).abs()).sum())
    }

    fn solve_second(&self, (a, b): &Self::Input) -> Result<Self::Output2, String> {
        let min = *b.iter().min().unwrap() as usize;
        let max = *b.iter().max().unwrap() as usize;
        let mut ocurrences = vec![0; max - min + 1];
        for v in b.iter() {
            ocurrences[*v as usize - min] += 1;
        }

        Ok(a.into_iter()
            .map(|v| *v as usize)
            .map(|v| {
                if v >= min && v <= max {
                    v * ocurrences[v - min]
                } else {
                    0
                }
            })
            .sum())
    }
}
