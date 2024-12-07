use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_dir(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
    fn move_position(&self, (x, y): &(isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.get_dir();
        (x + dx, y + dy)
    }
    fn turn_90(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

impl Solver for Problem {
    type Input = (HashSet<(isize, isize)>, (isize, isize), (isize, isize));
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut obstacles = HashSet::new();
        let mut position = (0, 0);

        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let size = (lines.len() as isize, lines[0].len() as isize);
        for (r, line) in lines.iter().enumerate() {
            for (c, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        obstacles.insert((r as isize, c as isize));
                    }
                    '^' => {
                        position = (r as isize, c as isize);
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
        let (mut r, mut c) = position;
        let mut direction = Direction::Up;
        let mut result = HashSet::new();
        result.insert(position.clone());

        while r > 0 && c > 0 && r < size.0 && c < size.1 {
            let mut new_pos = direction.move_position(&(r, c));
            while obstacles.contains(&new_pos) {
                direction = direction.turn_90();
                new_pos = direction.move_position(&(r, c));
            }

            result.insert(new_pos);
            (r, c) = new_pos;
        }

        Ok(result.len())
    }

    fn solve_second(
        &self,
        (obstacles, position, size): &Self::Input,
    ) -> Result<Self::Output2, String> {
        // let (mut r, mut c) = position;
        // let mut direction = Direction::Up;
        // let mut trail = HashSet::new();
        // trail.insert((position.clone(), direction.clone()));
        // let mut result = 0;

        // while r >= 0 && c >= 0 && r < size.0 && c < size.1 {
        //     let right_dir = direction.turn_90();
        //     let opposite_dir = right_dir.turn_90();
        //     for (rs, cs) in (1..).map(|i| {
        //         let (x, y) = right_dir.get_dir();
        //         (x * i, y * i)
        //     }) {
        //         if !(rs >= 0 && cs >= 0 && rs < size.0 && cs < size.1)
        //             || obstacles.contains(&(rs, cs))
        //         {
        //             break;
        //         }
        //         if trail.contains(&((rs, cs), opposite_dir.clone())) {
        //             result += 1;
        //             break;
        //         }
        //     }

        //     let mut new_pos = direction.move_position(&(r, c));
        //     while obstacles.contains(&new_pos) {
        //         direction = direction.turn_90();
        //         new_pos = direction.move_position(&(r, c));
        //     }

        //     trail.insert((new_pos, direction.clone()));
        //     (r, c) = new_pos;
        // }

        // Ok(result)

        let (mut r, mut c) = position;
        let mut direction = Direction::Up;
        // let mut steps = 0;
        let mut result = HashSet::new();
        let mut trail = HashSet::new();
        trail.insert(position.clone());

        while r > 0 && c > 0 && r < size.0 && c < size.1 {
            let mut new_pos = direction.move_position(&(r, c));
            while obstacles.contains(&new_pos) {
                direction = direction.turn_90();
                new_pos = direction.move_position(&(r, c));
            }

            if !trail.contains(&new_pos) {
                let mut new_obstacles = obstacles.clone();
                new_obstacles.insert(new_pos);
                if !result.contains(&new_pos)
                    && find_loop(
                        new_obstacles.clone(),
                        (r, c),
                        size.clone(),
                        direction.clone(),
                    )
                {
                    // println!("({r},{c}), {:?}", new_obstacles);
                    result.insert(new_pos.clone());
                }
            }

            (r, c) = new_pos;
            // steps += 1;
            trail.insert(new_pos);
        }

        Ok(result.len())
    }
}

fn find_loop(
    obstacles: HashSet<(isize, isize)>,
    position: (isize, isize),
    size: (isize, isize),
    direction: Direction,
) -> bool {
    let (mut r, mut c) = position;
    let mut direction = direction;
    let mut trail = HashSet::new();
    trail.insert((position.0, position.1, direction.clone()));

    while r >= 0 && c >= 0 && r < size.0 && c < size.1 {
        let mut new_pos = direction.move_position(&(r, c));
        while obstacles.contains(&new_pos) {
            direction = direction.turn_90();
            new_pos = direction.move_position(&(r, c));
        }

        if trail.contains(&(new_pos.0, new_pos.1, direction.clone())) {
            return true;
        }
        trail.insert((new_pos.0, new_pos.1, direction.clone()));
        (r, c) = new_pos;
    }

    false
}
