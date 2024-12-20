use super::Solver;
use crate::coordinate::{Coordinate, ZERO};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct State {
    position: Coordinate,
    cheat: Option<(Coordinate, Coordinate)>,
    cheat_time: usize,
}

impl State {
    fn successors(&self, grid: &Vec<Vec<char>>, cheat_length: usize) -> Vec<State> {
        let mut result = self
            .position
            .cardinals()
            .iter()
            .cloned()
            .filter(|p| {
                let pos = p.apply_vec(grid);
                pos.is_some() && *pos.unwrap() != '#'
            })
            .map(|p| self.step(&p))
            .collect_vec();

        if self.cheat.is_none() && cheat_length > 0 {
            for r in (self.position.0 - cheat_length as isize)
                ..=(self.position.0 + cheat_length as isize)
            {
                for c in (self.position.1 - cheat_length as isize)
                    ..=(self.position.1 + cheat_length as isize)
                {
                    let exit = Coordinate(r, c);
                    let pos = exit.apply_vec(grid);
                    let distance = exit.euclidean_distance(&self.position);
                    if pos.is_some()
                        && *pos.unwrap() != '#'
                        && distance > 1
                        && distance <= cheat_length as isize
                    {
                        result.push(self.cheat(&exit));
                    }
                }
            }
        }

        result
    }
}

impl State {
    fn new(coord: &Coordinate) -> Self {
        State {
            position: coord.clone(),
            cheat: None,
            cheat_time: 0,
        }
    }
    fn step(self: &Self, position: &Coordinate) -> Self {
        let mut result = self.clone();
        result.position = position.clone();
        result
    }
    fn cheat(self: &Self, position: &Coordinate) -> Self {
        State {
            position: position.clone(),
            cheat: Some((self.position.clone(), position.clone())),
            cheat_time: position.euclidean_distance(&self.position) as usize,
        }
    }
}

fn solve(input: &Vec<Vec<char>>, threshold: isize, cheats: usize) -> usize {
    let start = find_char(input, 'S').unwrap();
    let end = find_char(input, 'E').unwrap();

    // Start from end without cheats, fill in distance to end for every position
    let initial = State::new(&end);
    let mut queue = PriorityQueue::new();
    queue.push(initial, 0);

    // BFS
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut distances: HashMap<Coordinate, isize> = HashMap::new();
    while let Some((state, inv_time)) = queue.pop() {
        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position.clone());
        distances.insert(state.position.clone(), -inv_time);

        for successor in state.successors(input, 0) {
            queue.push(successor, inv_time - 1);
        }
    }

    // for r in 0..input.len() {
    //     for c in 0..input[r].len() {
    //         if let Some(d) = distances.get(&Coordinate::from_usize(r, c)) {
    //             print!("{d}")
    //         } else {
    //             print!("{}", input[r][c])
    //         }
    //     }
    //     println!("")
    // }

    let mut queue = PriorityQueue::new();
    let initial = State::new(&start);
    queue.push(initial, 0);

    // BFS
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut result: HashMap<(Coordinate, Coordinate), isize> = HashMap::new();
    while let Some((state, inv_time)) = queue.pop() {
        if let Some((entry, exit)) = state.cheat {
            let difference = distances.get(&entry).unwrap()
                - distances.get(&exit).unwrap()
                - state.cheat_time as isize;
            if difference >= threshold {
                result.insert((entry, exit), difference);
            }
            continue;
        }
        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position.clone());

        // println!("");
        // println!("{:?}", state);
        for successor in state.successors(input, cheats) {
            // println!("  {:?}", successor);
            queue.push(successor, inv_time - 1);
        }
    }

    // for (c, d) in result.iter() {
    //     println!("{d} {:?}", c);
    // }
    // let cheats: HashMap<Coordinate, char> = result
    //     .keys()
    //     .cloned()
    //     .flat_map(|(first, second)| vec![(first, '1'), (second, '2')])
    //     .collect();
    // for r in 0..input.len() {
    //     for c in 0..input[r].len() {
    //         if let Some(d) = cheats.get(&Coordinate::from_usize(r, c)) {
    //             print!("{d}")
    //         } else {
    //             print!("{}", input[r][c])
    //         }
    //     }
    //     println!("")
    // }
    // let mut counts: HashMap<isize, usize> = HashMap::new();
    // for (_, d) in result.iter() {
    //     let count = counts.get(d).unwrap_or(&0);
    //     counts.insert(*d, count + 1);
    // }
    // for (ps, c) in counts {
    //     println!("{ps} {c}");
    // }

    result.len()
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
        Ok(solve(input, 100, 2))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(solve(input, 100, 20))
    }
}

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
