use itertools::Itertools;

use crate::coordinate::{self, Coordinate, Direction};

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (HashSet<Coordinate>, Coordinate, Coordinate);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut obstacles = HashSet::new();
        let mut position = Coordinate(0, 0);

        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let size = Coordinate::from_usize(lines.len(), lines[0].len());
        for (r, line) in lines.iter().enumerate() {
            for (c, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        obstacles.insert(Coordinate::from_usize(r, c));
                    }
                    '^' => {
                        position = Coordinate::from_usize(r, c);
                    }
                    _ => {}
                }
            }
        }

        (obstacles, position, size)
    }

    fn solve_first(
        &self,
        (obstacles, position, size): &Self::Input,
    ) -> Result<Self::Output1, String> {
        let mut position = position.clone();
        let mut direction = Direction::Up;
        let mut result = HashSet::new();
        result.insert(position.clone());

        while position.is_in_bounds(&coordinate::ZERO, size) {
            let mut new_pos = &position + Coordinate::from(&direction);
            while obstacles.contains(&new_pos) {
                direction = direction.turn_90_right();
                new_pos = &position + Coordinate::from(&direction);
            }

            result.insert(new_pos.clone());
            position = new_pos;
        }

        Ok(result.len())
    }

    fn solve_second(
        &self,
        (obstacles, position, size): &Self::Input,
    ) -> Result<Self::Output2, String> {
        let mut position = position.clone();
        let mut direction = Direction::Up;
        let mut result = HashSet::new();
        let mut trail = HashSet::new();
        trail.insert(position.clone());

        while position.is_in_bounds(&coordinate::ZERO, size) {
            let mut new_pos = &position + Coordinate::from(&direction);
            while obstacles.contains(&new_pos) {
                direction = direction.turn_90_right();
                new_pos = &position + Coordinate::from(&direction);
            }

            if !trail.contains(&new_pos) {
                let mut new_obstacles = obstacles.clone();
                new_obstacles.insert(new_pos.clone());
                if !result.contains(&new_pos)
                    && find_loop(
                        new_obstacles.clone(),
                        position,
                        size.clone(),
                        direction.clone(),
                    )
                {
                    // println!("({r},{c}), {:?}", new_obstacles);
                    result.insert(new_pos.clone());
                }
            }

            trail.insert(new_pos.clone());
            position = new_pos;
        }

        Ok(result.len())
    }
}

fn find_loop(
    obstacles: HashSet<Coordinate>,
    position: Coordinate,
    size: Coordinate,
    direction: Direction,
) -> bool {
    let mut position = position.clone();
    let mut direction = direction;
    let mut trail = HashSet::new();
    trail.insert((position.0, position.1, direction.clone()));

    while position.is_in_bounds(&coordinate::ZERO, &size) {
        let mut new_pos = &position + Coordinate::from(&direction);
        while obstacles.contains(&new_pos) {
            direction = direction.turn_90_right();
            new_pos = &position + Coordinate::from(&direction);
        }

        if trail.contains(&(new_pos.0, new_pos.1, direction.clone())) {
            return true;
        }
        trail.insert((new_pos.0, new_pos.1, direction.clone()));
        position = new_pos;
    }

    false
}
