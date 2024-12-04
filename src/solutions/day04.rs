use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<String>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().map(|v| v.to_string()).collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut result = 0;
        for r in 0..input.len() {
            for c in 0..input[r].len() {
                result += find_xmas(input, (r, c));
            }
        }
        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut result = 0;
        for r in 0..input.len() {
            for c in 0..input[r].len() {
                if find_x_mas(input, (r, c)) {
                    result += 1
                }
            }
        }
        Ok(result)
    }
}

fn find_individual(
    input: &Vec<Vec<String>>,
    find: &str,
    (r, c): (usize, usize),
    (dr, dc): (isize, isize),
) -> bool {
    let (mut r, mut c) = (r as isize, c as isize);
    let (re, ce) = (
        r + dr * (find.len() as isize - 1),
        c + dc * (find.len() as isize - 1),
    );
    if re < 0 || re >= input.len() as isize || ce < 0 || ce >= input[re as usize].len() as isize {
        return false;
    }

    for i in 0..find.len() {
        if input[r as usize][c as usize] != find[i..i + 1] {
            return false;
        }

        r += dr;
        c += dc;
    }

    return true;
}
fn find_xmas(input: &Vec<Vec<String>>, start: (usize, usize)) -> usize {
    let mut result = 0;

    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for d in directions {
        if find_individual(input, "XMAS", start, d) {
            result += 1
        };
    }

    result
}

fn find_x_mas(input: &Vec<Vec<String>>, start: (usize, usize)) -> bool {
    if start.0 >= input.len() - 2 || start.1 >= input[start.0].len() - 2 {
        return false;
    }

    if !find_individual(input, "MAS", start, (1, 1))
        && !find_individual(input, "SAM", start, (1, 1))
    {
        return false;
    }

    return find_individual(input, "MAS", (start.0 + 2, start.1), (-1, 1))
        || find_individual(input, "SAM", (start.0 + 2, start.1), (-1, 1));
}
