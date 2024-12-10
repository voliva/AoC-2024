#![allow(unused)]

use std::{cmp::Ordering, fmt::Display};

use auto_ops::{impl_op, impl_op_ex};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Coordinate(pub isize, pub isize);
pub const ZERO: Coordinate = Coordinate(0, 0);
impl Coordinate {
    pub fn from_usize(r: usize, c: usize) -> Coordinate {
        Coordinate(r as isize, c as isize)
    }
    pub fn zero() -> Coordinate {
        Coordinate(0, 0)
    }

    pub fn is_in_bounds(&self, start: &Coordinate, end: &Coordinate) -> bool {
        start.0 <= self.0 && self.0 < end.0 && start.1 <= self.1 && self.1 < end.1
    }

    pub fn euclidean_distance(&self) -> isize {
        self.0.abs() + self.1.abs()
    }

    pub fn apply_vec<'a, T>(&self, vec: &'a Vec<Vec<T>>) -> Option<&'a T> {
        if (self.0 < 0 || self.1 < 0) {
            return None;
        }
        vec.get(self.0 as usize)
            .and_then(|i| i.get(self.1 as usize))
    }
    pub fn cardinals(&self) -> Vec<Coordinate> {
        vec![
            self.clone() + Coordinate::from(Direction::Up),
            self.clone() + Coordinate::from(Direction::Right),
            self.clone() + Coordinate::from(Direction::Down),
            self.clone() + Coordinate::from(Direction::Left),
        ]
    }
}

impl_op_ex!(+|a: &Coordinate, b: &Coordinate| -> Coordinate { Coordinate(a.0 + b.0, a.1 + b.1) });
impl_op_ex!(+= |a: &mut Coordinate, b: &Coordinate| { *a = &*a + b });
impl_op_ex!(-|a: &Coordinate, b: &Coordinate| -> Coordinate { Coordinate(a.0 - b.0, a.1 - b.1) });
impl_op_ex!(-= |a: &mut Coordinate, b: &Coordinate| { *a = &*a - b });
impl_op_ex!(*|a: &Coordinate, b: &isize| -> Coordinate { Coordinate(a.0 + b, a.1 + b) });
impl_op_ex!(/|a: &Coordinate, b: &isize| -> Coordinate { Coordinate(a.0 / b, a.1 / b) });

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

pub fn get_coordinates_from<T>(input: &Vec<Vec<T>>) -> impl Iterator<Item = (Coordinate, &T)> {
    input.iter().enumerate().flat_map(|(r, v)| {
        v.iter()
            .enumerate()
            .map(move |(c, value)| (Coordinate::from_usize(r, c), value))
    })
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coordinate(-1, 0),
            Direction::Down => Coordinate(1, 0),
            Direction::Left => Coordinate(0, -1),
            Direction::Right => Coordinate(0, 1),
        }
    }
}
impl From<&Direction> for Coordinate {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => Coordinate(-1, 0),
            Direction::Down => Coordinate(1, 0),
            Direction::Left => Coordinate(0, -1),
            Direction::Right => Coordinate(0, 1),
        }
    }
}
impl Direction {
    pub fn turn_90_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    pub fn turn_90_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    pub fn invert(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for r in grid.iter() {
        for c in r.iter() {
            print!("{c}");
        }
        println!();
    }
}
