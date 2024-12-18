use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::coordinate::{Coordinate, ZERO};

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Coordinate>;
    type Output1 = usize;
    type Output2 = String;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let (x, y) = line
                    .split(",")
                    .map(|v| v.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Coordinate::from_usize(x, y)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let bounds = Coordinate(71, 71);
        // let bounds = Coordinate(7, 7);
        let corrupted = input.iter().take(1024).collect_vec();
        // let corrupted = input.iter().take(12).collect_vec();

        let end = Coordinate(bounds.0 - 1, bounds.1 - 1);

        Ok(astar(
            &ZERO,
            |origin| {
                origin
                    .cardinals()
                    .iter()
                    .filter(|c| !corrupted.contains(c) && c.is_in_bounds(&ZERO, &bounds))
                    .cloned()
                    .map(|v| (v, 1))
                    .collect_vec()
            },
            |origin| origin.euclidean_distance(&end),
            |origin| origin == &end,
        )
        .unwrap()
        .0
        .len()
            - 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let bounds = Coordinate(71, 71);
        // let bounds = Coordinate(7, 7);
        let mut corrupted: HashSet<Coordinate> = input.iter().take(1024).cloned().collect();
        // let mut corrupted: HashSet<Coordinate> = input.iter().take(12).cloned().collect();

        let end = Coordinate(bounds.0 - 1, bounds.1 - 1);

        'outer: while let Some(path) = astar(
            &ZERO,
            |origin| {
                origin
                    .cardinals()
                    .iter()
                    .filter(|c| !corrupted.contains(c) && c.is_in_bounds(&ZERO, &bounds))
                    .cloned()
                    .map(|v| (v, 1))
                    .collect_vec()
            },
            |origin| origin.euclidean_distance(&end),
            |origin| origin == &end,
        ) {
            let coordinates: HashSet<&Coordinate> = path.0.iter().collect();
            let range = (corrupted.len() - 1)..input.len();
            for i in range {
                corrupted.insert(input[i].clone());
                if coordinates.contains(&input[i]) {
                    continue 'outer;
                }
            }
        }

        let corruption = input[corrupted.len() - 1].clone();
        Ok(format!("{},{}", corruption.0, corruption.1))
    }
}
