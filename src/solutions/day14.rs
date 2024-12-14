#![allow(unused)]
use itertools::Itertools;

use crate::coordinate::Coordinate;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

#[derive(Clone, Debug)]
pub struct Robot {
    position: Coordinate,
    velocity: Coordinate,
}

fn parse_coordinate(str: &str) -> Coordinate {
    let (x, y) = str.split(",").collect_tuple().unwrap();
    Coordinate(x.parse().unwrap(), y.parse().unwrap())
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split(" ").collect_tuple().unwrap();
        Ok(Robot {
            position: parse_coordinate(&p[2..]),
            velocity: parse_coordinate(&v[2..]),
        })
    }
}

const W: isize = 101;
const H: isize = 103;
// const W: isize = 11;
// const H: isize = 7;
const W_H: isize = W / 2;
const H_H: isize = H / 2;

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
pub fn period(a: usize, m: usize) -> usize {
    if a == 0 {
        return 1;
    }
    let d = gcd(a, m);
    if d == 1 {
        m
    } else {
        m / d
    }
}

impl Robot {
    pub fn step(&mut self, steps: isize) {
        self.position.0 = (self.position.0 + self.velocity.0 * steps) % W;
        if self.position.0 < 0 {
            self.position.0 += W;
        }
        self.position.1 = (self.position.1 + self.velocity.1 * steps) % H;
        if self.position.1 < 0 {
            self.position.1 += H;
        }
    }
    fn get_quadrant(&self) -> Option<Coordinate> {
        if self.position.0 == W_H || self.position.1 == H_H {
            None
        } else {
            let x = if self.position.0 < W_H { 0 } else { 1 };
            let y = if self.position.1 < H_H { 0 } else { 1 };

            Some(Coordinate(x, y))
        }
    }
    fn period(&self) -> usize {
        period(self.velocity.0.abs() as usize, W as usize)
            * period(self.velocity.1.abs() as usize, H as usize)
    }
}

impl Solver for Problem {
    type Input = Vec<Robot>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut robots = input.iter().cloned().collect_vec();

        for robot in robots.iter_mut() {
            robot.step(100);
        }

        // let robot_map: HashMap<Coordinate, usize> =
        //     robots.iter().cloned().counts_by(|c| c.position);
        // for y in 0..H {
        //     for x in 0..W {
        //         print!("{}", robot_map.get(&Coordinate(x, y)).unwrap_or(&0))
        //     }
        //     println!("")
        // }
        // println!("");

        let quadrants = robots.iter().filter_map(|v| v.get_quadrant()).counts();

        Ok(quadrants.values().fold(1, |a, b| a * b))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut robots = input.iter().cloned().collect_vec();

        // for robot in robots {
        //     println!("{}", robot.period());
        // }
        // See that the period is 10403 for all of them (which is W*H)
        // Means that it repeats every 10403 steps.

        for i in 0..10403 {
            for robot in robots.iter_mut() {
                robot.step(1);
            }

            // See that every now and then bots arrange into something blurry
            // note the period is 103 for a horizontal blur and 101 for a vertical blur
            // Then just print those specifics to reduce the noise
            // if i % 103 == 0 || i % 101 == 45 {
            //     println!("{i}");
            //     let robot_map: HashMap<Coordinate, usize> =
            //         robots.iter().cloned().counts_by(|c| c.position);
            //     for y in 0..H {
            //         for x in 0..W {
            //             let c = robot_map.get(&Coordinate(x, y)).unwrap_or(&0);
            //             if *c <= 0 {
            //                 print!(" ")
            //             } else {
            //                 print!("{c}")
            //             }
            //         }
            //         println!("")
            //     }
            //     println!("");
            // }
        }

        // Visually see that it's when i=7519, meaning 7520 steps.
        Ok(7519 + 1)
    }
}
