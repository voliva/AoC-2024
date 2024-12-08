use itertools::Itertools;

use super::Solver;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

pub struct Problem;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Coordinate(isize, isize);
impl Coordinate {
    fn from_usize(r: usize, c: usize) -> Coordinate {
        Coordinate(r as isize, c as isize)
    }

    fn is_in_bounds(&self, start: &Coordinate, end: &Coordinate) -> bool {
        start.0 <= self.0 && self.0 < end.0 && start.1 <= self.1 && self.1 < end.1
    }
}

impl Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 == other.0 && self.1 == other.1 {
            Some(Ordering::Equal)
        } else if self.0 < other.0 && self.1 < other.1 {
            Some(Ordering::Less)
        } else if self.0 > other.0 && self.1 > other.1 {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Solver for Problem {
    type Input = (Coordinate, HashMap<char, HashSet<Coordinate>>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let grid: Vec<Vec<String>> = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().map(|v| String::from(v)).collect())
            .collect();
        let size = Coordinate::from_usize(grid.len(), grid[0].len());
        let mut antennas: HashMap<char, HashSet<Coordinate>> = HashMap::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if cell.len() == 0 {
                    continue;
                }
                let char: char = cell.chars().take(1).collect_vec()[0];
                if char == '.' {
                    continue;
                }
                let coord = Coordinate::from_usize(r, c);
                antennas
                    .entry(char)
                    .and_modify(|v| {
                        v.insert(coord.clone());
                    })
                    .or_insert({
                        let mut set = HashSet::new();
                        set.insert(coord);
                        set
                    });
            }
        }

        (size, antennas)
    }

    fn solve_first(&self, (size, antennas): &Self::Input) -> Result<Self::Output1, String> {
        let mut antinodes: HashSet<Coordinate> = HashSet::new();
        let origin = Coordinate(0, 0);
        for freq in antennas.values() {
            for pair in freq.iter().combinations(2) {
                let diff = pair[1] - pair[0];
                let antinode_a = pair[0] - &diff;
                if antinode_a.is_in_bounds(&origin, size) {
                    antinodes.insert(antinode_a);
                }
                let antinode_b = pair[1] + &diff;
                if antinode_b.is_in_bounds(&origin, size) {
                    antinodes.insert(antinode_b);
                }
            }
        }

        Ok(antinodes.len())
    }

    fn solve_second(&self, (size, antennas): &Self::Input) -> Result<Self::Output2, String> {
        let mut antinodes: HashSet<Coordinate> = HashSet::new();
        let origin = Coordinate(0, 0);
        for freq in antennas.values() {
            for pair in freq.iter().combinations(2) {
                let diff = pair[1] - pair[0];
                antinodes.insert(pair[0].clone());
                antinodes.insert(pair[1].clone());
                let mut antinode_a = pair[0].clone();
                antinode_a = &antinode_a - &diff;
                while antinode_a.is_in_bounds(&origin, size) {
                    antinodes.insert(antinode_a.clone());
                    antinode_a = &antinode_a - &diff;
                }

                let mut antinode_b = pair[0].clone();
                antinode_b = &antinode_b + &diff;
                while antinode_b.is_in_bounds(&origin, size) {
                    antinodes.insert(antinode_b.clone());
                    antinode_b = &antinode_b + &diff;
                }
            }
        }

        // for r in 0..size.0 {
        //     for c in 0..size.1 {
        //         if antinodes.contains(&Coordinate(r, c)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        // 1264
        Ok(antinodes.len())
    }
}
