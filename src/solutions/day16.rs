use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag_collect};

use crate::coordinate::{Coordinate, Direction};

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn find_char(input: &Vec<Vec<char>>, needle: char) -> Option<Coordinate> {
    input.iter().enumerate().find_map(|(r, row)| {
        row.iter().enumerate().find_map(|(c, cv)| {
            if *cv == needle {
                Some(Coordinate::from_usize(r, c))
            } else {
                None
            }
        })
    })
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct State {
    position: Coordinate,
    facing: Direction,
}

impl State {
    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(State, usize)> {
        let mut result = vec![
            (
                State {
                    position: self.position.clone(),
                    facing: self.facing.turn_90_left(),
                },
                1000,
            ),
            (
                State {
                    position: self.position.clone(),
                    facing: self.facing.turn_90_right(),
                },
                1000,
            ),
        ];
        let forward = self.position.apply_dir(&self.facing);
        if let Some(c) = forward.apply_vec(grid) {
            if *c != '#' {
                result.push((
                    State {
                        position: forward,
                        facing: self.facing.clone(),
                    },
                    1,
                ));
            }
        }

        result
    }
}

impl Solver for Problem {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.chars().collect_vec())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let start = find_char(input, 'S').unwrap();
        let end = find_char(input, 'E').unwrap();

        let initial_state = State {
            position: start,
            facing: Direction::Right,
        };
        let result = astar(
            &initial_state,
            |state| state.successors(input),
            |state| state.position.euclidean_distance(&end) as usize,
            |state| state.position == end,
        )
        .unwrap();

        Ok(result.1)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let start = find_char(input, 'S').unwrap();
        let end = find_char(input, 'E').unwrap();

        let initial_state = State {
            position: start,
            facing: Direction::Right,
        };
        let result = astar_bag_collect(
            &initial_state,
            |state| state.successors(input),
            |state| state.position.euclidean_distance(&end) as usize,
            |state| state.position == end,
        )
        .unwrap();
        let coordinates: HashSet<Coordinate> = result
            .0
            .iter()
            .flat_map(|res| res.iter().map(|s| s.position.clone()))
            .collect();

        Ok(coordinates.len())
    }
}

/*
At some point I wanted a nested HashSet/HashMap. Here's how to impl Hash for one.
https://stackoverflow.com/questions/36562419/hashset-as-key-for-other-hashset

struct StateSet(HashSet<State>);

impl PartialEq for StateSet {
    fn eq(&self, other: &StateSet) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for StateSet {}

impl Hash for StateSet {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&State> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }

}

fn main() {
    let hmap: HashSet<StateSet> = HashSet::new();
}
 */
