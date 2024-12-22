use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::coordinate::{Coordinate, Direction};

use super::Solver;
use core::panic;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

pub struct Code {
    value: isize,
    code: Vec<char>,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.chars().collect_vec();
        let value = s[0..(s.len() - 1).max(1)].parse().unwrap();
        return Ok(Code { value, code });
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
enum Action {
    Move(Direction),
    Push,
}

impl Action {
    fn from_coordinate(coordinate: &Coordinate) -> Self {
        match coordinate {
            Coordinate(0, 1) => Action::Move(Direction::Up),
            Coordinate(0, 2) => Action::Push,
            Coordinate(1, 0) => Action::Move(Direction::Left),
            Coordinate(1, 1) => Action::Move(Direction::Down),
            Coordinate(1, 2) => Action::Move(Direction::Right),
            _ => panic!(),
        }
    }
    fn to_coordinate(&self) -> Coordinate {
        match self {
            Action::Push => Coordinate(0, 2),
            Action::Move(Direction::Up) => Coordinate(0, 1),
            Action::Move(Direction::Left) => Coordinate(1, 0),
            Action::Move(Direction::Down) => Coordinate(1, 1),
            Action::Move(Direction::Right) => Coordinate(1, 2),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct State {
    last_action: Option<Action>,
    positions: Vec<Coordinate>,
    last_position: Coordinate,
    found: usize,
}

fn get_direction(from: &Coordinate, to: &Coordinate, is_numeric: bool) -> Vec<Direction> {
    let Coordinate(r, c) = to - from;
    let mut result = vec![];

    if c < 0
        && ((is_numeric && from != &Coordinate(3, 1)) || (!is_numeric && from != &Coordinate(0, 1)))
    {
        result.push(Direction::Left);
    }
    if c > 0 {
        result.push(Direction::Right);
    }
    if r < 0 && (is_numeric || (!is_numeric && from != &Coordinate(1, 0))) {
        result.push(Direction::Up);
    }
    if r > 0 && ((is_numeric && from != &Coordinate(2, 0)) || !is_numeric) {
        result.push(Direction::Down);
    }

    result
}

fn char_to_coordinate(char: char) -> Coordinate {
    if char == 'A' {
        Coordinate(3, 2)
    } else if char == '0' {
        Coordinate(3, 1)
    } else {
        let i_val = (char as isize) - ('1' as isize);
        Coordinate(2 - i_val / 3, i_val % 3)
    }
}

fn get_horizontal_direction(c: isize) -> Direction {
    if c < 0 {
        Direction::Left
    } else {
        Direction::Right
    }
}
fn get_vertical_direction(r: isize) -> Direction {
    if r < 0 {
        Direction::Up
    } else {
        Direction::Down
    }
}

fn get_paths(
    from: &Coordinate,
    to: &Coordinate,
    forbidden: &Coordinate,
    // start position on DPAD
    // starting: &Coordinate,
) -> Vec<Vec<Direction>> {
    let Coordinate(r, c) = to - from;

    match (r, c) {
        (0, 0) => vec![vec![]],
        (0, c) => vec![vec![get_horizontal_direction(c); c.abs() as usize]],
        (r, 0) => vec![vec![get_vertical_direction(r); r.abs() as usize]],
        (r, c) => {
            let hd = get_horizontal_direction(c);
            let vd = get_vertical_direction(r);

            let preferred = None; // get_preferred(starting, &hd, &vd);

            let fh = from.0 == forbidden.0 && to.1 == forbidden.1;
            let fv = from.1 == forbidden.1 && to.0 == forbidden.0;

            let first_horizontal = if fh || (!fv && preferred.is_some() && preferred == Some(&vd)) {
                None
            } else {
                let mut result = vec![hd.clone(); c.abs() as usize];
                for _ in 0..r.abs() {
                    result.push(vd.clone());
                }
                Some(result)
            };
            let first_vertical = if fv || (!fh && preferred.is_some() && preferred == Some(&hd)) {
                None
            } else {
                let mut result = vec![vd; r.abs() as usize];
                let direction = hd;
                for _ in 0..c.abs() {
                    result.push(direction.clone());
                }
                Some(result)
            };

            vec![first_horizontal, first_vertical]
                .into_iter()
                .filter_map(|v| v)
                .collect()
        }
    }
}

fn get_length(
    from: &Coordinate,
    to: &Coordinate,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(Coordinate, Coordinate, usize), usize>,
) -> usize {
    let key = (from.clone(), to.clone(), depth);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let forbidden = if depth == 1 {
        Coordinate(3, 0)
    } else {
        Coordinate(0, 0)
    };
    let paths = get_paths(&from, &to, &forbidden);
    // for _ in 1..depth {
    //     print!(" ");
    // }
    // println!("paths {:?}->{:?} {:?}", from, to, paths);

    let result = if depth == max_depth {
        paths.iter().map(|p| p.len()).min().unwrap() + 1
    } else {
        paths
            .iter()
            .map(|p| -> usize {
                vec![Action::Push]
                    .into_iter()
                    .chain(p.iter().map(|v| Action::Move(v.clone())))
                    .chain(vec![Action::Push])
                    .tuple_windows()
                    .map(|(prev, next)| {
                        get_length(
                            &prev.to_coordinate(),
                            &next.to_coordinate(),
                            depth + 1,
                            max_depth,
                            cache,
                        )
                    })
                    .sum()
            })
            .min()
            .unwrap()
    };

    cache.insert(key, result);
    return result;
}

fn get_code_length(
    code: &[char],
    depth: usize,
    cache: &mut HashMap<(Coordinate, Coordinate, usize), usize>,
) -> usize {
    vec!['A']
        .iter()
        .chain(code)
        .map(|c| char_to_coordinate(*c))
        .tuple_windows()
        .map(|(prev, next)| get_length(&prev, &next, 1, depth, cache))
        .sum::<usize>()

    // let mut position = Coordinate(3, 2);
    // let forbidden = Coordinate(3, 0);
    // // let confirm = Coordinate(0, 2);

    // for c in code {
    //     let dest = char_to_coordinate(*c);
    //     let paths = get_paths(&position, &dest, &forbidden);
    //     println!("{:?}, {:?} -> {:?}", paths, position, dest);
    //     position = dest;
    // }

    // todo!()
}

fn direction_to_pos(dir: &Direction) -> Coordinate {
    match dir {
        Direction::Down => Coordinate(1, 1),
        Direction::Left => Coordinate(1, 0),
        Direction::Right => Coordinate(1, 2),
        Direction::Up => Coordinate(0, 1),
    }
}

impl State {
    fn move_idx(&self, idx: usize, direction: &Direction) -> Option<(Vec<Coordinate>, Coordinate)> {
        if idx < self.positions.len() {
            let Coordinate(r, c) = self.positions[idx].apply_dir(direction);
            if r > 1 || c > 2 || r < 0 || c < 0 || (r == 0 && c == 0) {
                None
            } else {
                Some((
                    self.positions
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            if i == idx {
                                Coordinate(r, c)
                            } else {
                                v.clone()
                            }
                        })
                        .collect(),
                    self.last_position.clone(),
                ))
            }
        } else {
            assert!(idx == self.positions.len());

            let Coordinate(r, c) = self.last_position.apply_dir(direction);
            if r > 3 || c > 2 || r < 0 || c < 0 || (r == 3 && c == 0) {
                None
            } else {
                Some((self.positions.clone(), Coordinate(r, c)))
            }
        }
    }

    fn press_idx(&self, idx: usize) -> Option<(Vec<Coordinate>, Coordinate, Option<char>)> {
        if idx < self.positions.len() {
            let action = Action::from_coordinate(&self.positions[idx]);
            if let Action::Move(direction) = action {
                self.move_idx(idx + 1, &direction)
                    .map(|(positions, last_position)| (positions, last_position, None))
            } else {
                self.press_idx(idx + 1)
            }
        } else {
            assert!(idx == self.positions.len());

            let Coordinate(r, c) = self.last_position;
            let pressed = match (r, c) {
                (3, 1) => '0',
                (3, 2) => 'A',
                (r, c) if r >= 0 && r <= 2 && c >= 0 && c <= 2 => {
                    ('1' as i8 + ((2 - r) * 3 + c) as i8) as u8 as char
                }
                (r, c) => panic!("Pressed out of bounds {r} {c}"),
            };

            Some((
                self.positions.clone(),
                self.last_position.clone(),
                Some(pressed),
            ))
        }
    }

    fn perform_action(&self, action: Action, code: &[char]) -> Option<State> {
        if let Action::Move(direction) = &action {
            self.move_idx(0, direction)
                .map(|(positions, last_position)| State {
                    last_action: Some(action),
                    positions,
                    last_position,
                    found: self.found,
                })
        } else {
            self.press_idx(0)
                .and_then(|(positions, last_position, maybe_char)| {
                    let mut result = State {
                        last_action: Some(action),
                        positions,
                        last_position,
                        found: self.found,
                    };

                    if let Some(char) = maybe_char {
                        if char == code[result.found] {
                            result.found += 1;
                            Some(result)
                        } else {
                            None
                        }
                    } else {
                        Some(result)
                    }
                })
        }
    }

    fn get_targets(&self, idx: usize, code: &[char]) -> Vec<Coordinate> {
        if idx < self.positions.len() {
            let child_is_last = idx + 1 >= self.positions.len();
            let child_position = if child_is_last {
                &self.last_position
            } else {
                &self.positions[idx + 1]
            };

            let result: HashSet<Coordinate> = self
                .get_targets(idx + 1, code)
                .into_iter()
                .flat_map(|child_target| {
                    if child_position == &child_target {
                        vec![Coordinate(0, 2)]
                    } else {
                        let directions =
                            get_direction(child_position, &child_target, child_is_last);
                        let current_pos = &self.positions[idx];

                        let positions_with_distance = directions
                            .iter()
                            .map(|dir| direction_to_pos(dir))
                            .map(|pos| {
                                let distance = pos.euclidean_distance(current_pos);
                                (pos, distance)
                            })
                            .collect_vec();

                        // let min_distance = positions_with_distance
                        //     .iter()
                        //     .map(|(_, v)| v)
                        //     .min()
                        //     .unwrap();

                        positions_with_distance
                            .iter()
                            // .filter(|(_, v)| v == min_distance)
                            .map(|(v, _)| v)
                            .cloned()
                            .collect()
                    }
                })
                .collect();

            result.into_iter().collect()
        } else {
            assert!(idx == self.positions.len());

            let char = code[self.found];
            vec![char_to_coordinate(char)]
        }
    }

    fn get_best_action(&self, code: &[char]) -> Vec<Action> {
        let position = &self.positions[0];

        let result = self
            .get_targets(0, code)
            .into_iter()
            .flat_map(|target| {
                if &target == position {
                    vec![Action::Push]
                } else {
                    get_direction(position, &target, false)
                        .iter()
                        .map(|dir| Action::Move(dir.clone()))
                        .collect()
                }
            })
            .collect_vec();
        // if result.len() > 3 {
        //     println!("{}", result.len());
        // }
        return result;
    }

    fn successors(&self, code: &[char]) -> Vec<(State, usize)> {
        // let mut result = CARDINALS
        //     .iter()
        //     .filter_map(|direction| self.perform_action(Action::Move(direction.clone()), code))
        //     .map(|v| (v, 1))
        //     .collect_vec();
        // if let Some(state) = self.perform_action(Action::Push, code) {
        //     result.push((state, 1))
        // }
        // result
        self.get_best_action(code)
            .into_iter()
            .filter_map(|action| self.perform_action(action, code))
            .map(|v| (v, 1))
            .collect()
    }
}

fn solve(code: &Code, robots: usize) -> Option<Vec<State>> {
    let initial_state = State {
        last_action: None,
        positions: (0..robots).map(|_| Coordinate(0, 2)).collect(),
        last_position: Coordinate(3, 2),
        found: 0,
    };

    dijkstra(
        &initial_state,
        |state| state.successors(&code.code),
        |state| state.found == code.code.len(),
    )
    .map(|(v, _)| v)
}

impl Solver for Problem {
    type Input = Vec<Code>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input
            .iter()
            .map(|code| {
                let result = solve(code, 2).unwrap();
                // println!("first {}", result.len() - 1);
                code.value * (result.len() - 1) as isize
            })
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut cache = HashMap::new();

        Ok(input
            .iter()
            .map(|code| {
                let result = get_code_length(&code.code, 26, &mut cache);
                // println!("second {}", result);
                code.value * result as isize
            })
            .sum())
        // Ok(get_code_length(&input[0].code, 3, &mut cache))
        // Ok(input
        //     .iter()
        //     .map(|code| {
        //         println!("Do it");
        //         let result = solve(code, 25).unwrap();
        //         code.value * (result.len() - 1) as isize
        //     })
        //     .sum())
    }
}
