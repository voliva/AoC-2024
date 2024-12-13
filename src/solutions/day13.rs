use itertools::Itertools;

use crate::coordinate::Coordinate;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

pub struct Machine {
    a: Coordinate,
    b: Coordinate,
    prize: Coordinate,
}

fn parse_coordinates(line: &str) -> Coordinate {
    let (_, data) = line.split(": ").collect_tuple().unwrap();
    let (x, y) = data.split(", ").collect_tuple().unwrap();

    Coordinate(x[2..].parse().unwrap(), y[2..].parse().unwrap())
}

impl Machine {
    fn get_price(&self) -> isize {
        let a = &self.a;
        let b = &self.b;
        let p = &self.prize;

        let a_num = p.1 * b.0 - p.0 * b.1;
        let a_den = a.1 * b.0 - a.0 * b.1;
        if a_den == 0 || a_num % a_den != 0 {
            return 0;
        }
        let a_moves = a_num / a_den;
        if b.0 == 0 {
            panic!("B is zero")
        };
        let b_moves = (p.0 - a_moves * a.0) / b.0;
        if a_moves < 0 || b_moves < 0 {
            return 0;
        }

        return a_moves * 3 + b_moves;
    }
}

impl Solver for Problem {
    type Input = Vec<Machine>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut result = Vec::new();

        for i in (0..lines.len()).step_by(4) {
            result.push(Machine {
                a: parse_coordinates(&lines[i]),
                b: parse_coordinates(&lines[i + 1]),
                prize: parse_coordinates(&lines[i + 2]),
            });
        }
        result
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|v| v.get_price()).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(input
            .iter()
            .map(|v| Machine {
                a: v.a.clone(),
                b: v.b.clone(),
                prize: &v.prize + Coordinate(10000000000000, 10000000000000),
            })
            .map(|v| v.get_price())
            .sum())
    }
}
