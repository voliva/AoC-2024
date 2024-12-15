#![allow(unused)]
use itertools::Itertools;

use crate::coordinate::{Coordinate, Direction, ZERO};

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

pub struct Problem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarehouseElement {
    Wall,
    Box,
}

#[derive(Debug, Clone)]
pub struct Warehouse {
    map: HashMap<Coordinate, WarehouseElement>,
    robot: Coordinate,
}

impl Warehouse {
    fn move_robot(&mut self, dir: &Direction) {
        let moving_pos = self.robot.apply_dir(dir);
        let mut next_pos = moving_pos.clone();
        loop {
            match self.map.get(&next_pos) {
                Some(WarehouseElement::Wall) => {
                    return;
                }
                Some(WarehouseElement::Box) => {
                    next_pos = next_pos.apply_dir(dir);
                }
                None => {
                    break;
                }
            }
        }
        self.map.remove(&moving_pos);
        if next_pos != moving_pos {
            self.map.insert(next_pos, WarehouseElement::Box);
        }
        self.robot = moving_pos;
    }

    fn gps_sum(&self) -> isize {
        self.map
            .iter()
            .filter(|(_, v)| v == &&WarehouseElement::Box)
            .map(|(coord, _)| coord.0 * 100 + coord.1)
            .sum()
    }

    fn widen(&self) -> Warehouse {
        Warehouse {
            map: self
                .map
                .iter()
                .flat_map(|(c, e)| match e {
                    WarehouseElement::Box => vec![(Coordinate(c.0, c.1 * 2), e.clone())],
                    WarehouseElement::Wall => {
                        let first = (Coordinate(c.0, c.1 * 2), e.clone());
                        let mut next = first.clone();
                        next.0 .1 += 1;
                        vec![first, next]
                    }
                })
                .collect(),
            robot: Coordinate(self.robot.0, self.robot.1 * 2),
        }
    }
    fn can_move(&self, dir: &Direction, pos: &Coordinate) -> Option<Vec<Coordinate>> {
        match self.map.get(pos) {
            None => match dir {
                Direction::Down | Direction::Up | Direction::Left => {
                    let left = Coordinate(pos.0, pos.1 - 1);
                    match self.map.get(&left) {
                        Some(WarehouseElement::Box) => self.can_move(dir, &left),
                        _ => Some(vec![]),
                    }
                }
                Direction::Right => Some(vec![]),
            },
            Some(WarehouseElement::Wall) => None,
            Some(WarehouseElement::Box) => (match dir {
                Direction::Left => {
                    let next = pos.apply_dir(dir);
                    self.can_move(dir, &next)
                }
                Direction::Right => {
                    let next = pos.apply_dir(dir).apply_dir(dir);
                    self.can_move(dir, &next)
                }
                _ => {
                    let next = pos.apply_dir(dir);
                    let next_right = Coordinate(next.0, next.1 + 1);
                    self.can_move(dir, &next).and_then(|left| {
                        self.can_move(dir, &next_right)
                            .map(|v| left.into_iter().chain(v.into_iter()).collect())
                    })
                }
            })
            .map(|mut v| {
                v.push(pos.clone());
                v
            }),
        }
    }
    fn move_robot_wide(&mut self, dir: &Direction) {
        let moving_pos = self.robot.apply_dir(dir);
        if let Some(boxes) = self.can_move(dir, &moving_pos) {
            for coord in boxes.iter() {
                self.map.remove(coord);
            }
            for coord in boxes.iter() {
                self.map.insert(coord.apply_dir(dir), WarehouseElement::Box);
            }
            self.robot = moving_pos.clone();
        }
    }

    fn print(&self, wide: bool) {
        let mut grid = iter::repeat_with(|| vec![' '; 100]).take(100).collect_vec();
        grid[self.robot.0 as usize][self.robot.1 as usize] = '@';

        let mut max = ZERO.clone();
        for (coord, value) in self.map.iter() {
            max = max.max(&coord);
            if wide {
                match value {
                    WarehouseElement::Box => {
                        grid[coord.0 as usize][coord.1 as usize] = '[';
                        grid[coord.0 as usize][coord.1 as usize + 1] = ']';
                    }
                    WarehouseElement::Wall => grid[coord.0 as usize][coord.1 as usize] = '#',
                };
            } else {
                let char = match value {
                    WarehouseElement::Box => 'O',
                    WarehouseElement::Wall => '#',
                };
                grid[coord.0 as usize][coord.1 as usize] = char;
            }
        }

        println!();
        for r in 0..=max.0 {
            for c in 0..=max.1 {
                print!("{}", grid[r as usize][c as usize]);
            }
            println!();
        }
    }
}

impl Solver for Problem {
    type Input = (Warehouse, Vec<Direction>);
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut warehouse = Warehouse {
            map: HashMap::new(),
            robot: ZERO,
        };
        let mut directions = Vec::new();

        let mut mode = 0;
        for (r, line) in lines.iter().enumerate() {
            if line == "" {
                mode = 1;
                continue;
            }
            if mode == 0 {
                for (c, char) in line.chars().enumerate() {
                    let coord = Coordinate::from_usize(r, c);
                    match char {
                        '#' => {
                            warehouse.map.insert(coord, WarehouseElement::Wall);
                        }
                        'O' => {
                            warehouse.map.insert(coord, WarehouseElement::Box);
                        }
                        '@' => {
                            warehouse.robot = coord;
                        }
                        _ => {}
                    }
                }
            } else {
                directions.push(
                    line.chars()
                        .map(|char| Direction::from_arrow_char(char).expect("Unknown direction"))
                        .collect_vec(),
                );
            }
        }

        (warehouse, directions.concat())
    }

    fn solve_first(&self, (warehouse, directions): &Self::Input) -> Result<Self::Output1, String> {
        let mut warehouse = warehouse.clone();

        for dir in directions {
            // warehouse.print();
            warehouse.move_robot(dir);
        }

        // warehouse.print();

        Ok(warehouse.gps_sum())
    }

    fn solve_second(&self, (warehouse, directions): &Self::Input) -> Result<Self::Output2, String> {
        let mut warehouse = warehouse.widen().clone();

        for dir in directions {
            warehouse.move_robot_wide(dir);
            // print!("\x1Bc");
            // warehouse.print(true);
            // println!("{:?}", dir);
            // sleep(time::Duration::from_millis(6));
        }

        Ok(warehouse.gps_sum())
    }
}
