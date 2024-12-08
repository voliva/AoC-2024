use itertools::Itertools;

use super::Solver;
use crate::coordinate::{self, get_coordinates_from, Coordinate};
use crate::many_to_many::ManyToMany;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (Coordinate, ManyToMany<char, Coordinate>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let grid: Vec<Vec<String>> = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().map(|v| String::from(v)).collect())
            .collect();

        let size = Coordinate::from_usize(grid.len(), grid[0].len());
        let mut antennas = ManyToMany::new();
        for (coord, cell) in get_coordinates_from(&grid) {
            if cell.len() == 0 {
                continue;
            }
            if cell == "." {
                continue;
            }
            let char: char = cell.chars().take(1).collect_vec()[0];
            antennas.insert(char, coord);
        }

        (size, antennas)
    }

    fn solve_first(&self, (size, antennas): &Self::Input) -> Result<Self::Output1, String> {
        let mut antinodes: HashSet<Coordinate> = HashSet::new();
        for freq in antennas.outer().values() {
            for pair in freq.iter().combinations(2) {
                let diff = pair[1] - pair[0];
                let antinode_a = pair[0] - &diff;
                if antinode_a.is_in_bounds(&coordinate::ZERO, size) {
                    antinodes.insert(antinode_a);
                }
                let antinode_b = pair[1] + &diff;
                if antinode_b.is_in_bounds(&coordinate::ZERO, size) {
                    antinodes.insert(antinode_b);
                }
            }
        }

        Ok(antinodes.len())
    }

    fn solve_second(&self, (size, antennas): &Self::Input) -> Result<Self::Output2, String> {
        let mut antinodes: HashSet<Coordinate> = HashSet::new();
        for freq in antennas.outer().values() {
            for pair in freq.iter().combinations(2) {
                antinodes.insert(pair[0].clone());
                antinodes.insert(pair[1].clone());

                let diff = pair[1] - pair[0];
                let mut antinode_a = pair[0].clone();
                antinode_a -= &diff;
                while antinode_a.is_in_bounds(&coordinate::ZERO, size) {
                    antinodes.insert(antinode_a.clone());
                    antinode_a -= &diff;
                }

                let mut antinode_b = pair[0].clone();
                antinode_b += &diff;
                while antinode_b.is_in_bounds(&coordinate::ZERO, size) {
                    antinodes.insert(antinode_b.clone());
                    antinode_b += &diff;
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

        Ok(antinodes.len())
    }
}
