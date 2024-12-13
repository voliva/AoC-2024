use itertools::Itertools;

use crate::coordinate::{get_coordinates_from, Coordinate, Direction, CARDINALS};

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

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
        let mut ids: HashMap<Coordinate, usize> = HashMap::new();
        let mut areas: HashMap<usize, usize> = HashMap::new();
        let mut perimeters: HashMap<usize, usize> = HashMap::new();
        let mut id_to_char: HashMap<usize, char> = HashMap::new();

        for (coord, char) in get_coordinates_from(input) {
            let mut perimeter = 0;

            if !ids.contains_key(&coord) {
                let id = id_to_char.len();
                fill_id(input, coord.clone(), id, &mut ids);
                id_to_char.insert(id, *char);
            }
            let id = *ids.get(&coord).unwrap();

            for outer in coord.cardinals() {
                let outer_v = outer.apply_vec(input);

                if outer_v.is_none() || outer_v.unwrap() != char {
                    perimeter += 1;
                }
            }

            let prev_perimeter = perimeters.get(&id).unwrap_or(&0);
            perimeters.insert(id, prev_perimeter + perimeter);
            let area = areas.get(&id).unwrap_or(&0);
            areas.insert(id, area + 1);
        }

        let areas_zipped = areas
            .iter()
            .map(|(id, area)| {
                (
                    id_to_char.get(id).unwrap(),
                    area,
                    perimeters.get(id).unwrap(),
                )
            })
            .collect_vec();
        // println!("{:?}", areas_zipped);

        Ok(areas_zipped
            .into_iter()
            .map(|(_, area, perimeter)| area * perimeter)
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut ids: HashMap<Coordinate, usize> = HashMap::new();
        let mut areas: HashMap<usize, usize> = HashMap::new();
        let mut perimeters: HashMap<usize, usize> = HashMap::new();
        let mut sides: HashMap<usize, usize> = HashMap::new();
        let mut id_to_char: HashMap<usize, char> = HashMap::new();

        for (coord, char) in get_coordinates_from(input) {
            let mut perimeter = 0;
            let mut side = 0;

            if !ids.contains_key(&coord) {
                let id = id_to_char.len();
                fill_id(input, coord.clone(), id, &mut ids);
                id_to_char.insert(id, *char);
            }
            let id = *ids.get(&coord).unwrap();

            for card in CARDINALS {
                let outer = coord.clone() + Coordinate::from(card);
                let outer_v = outer.apply_vec(input);

                if outer_v.is_none() || outer_v.unwrap() != char {
                    perimeter += 1;

                    let is_side = match card {
                        Direction::Left => {
                            coord.apply_dir(Direction::Up).apply_vec(input) != Some(char)
                                || coord
                                    .apply_dir(Direction::Up)
                                    .apply_dir(Direction::Left)
                                    .apply_vec(input)
                                    == Some(char)
                        }
                        Direction::Right => {
                            coord.apply_dir(Direction::Up).apply_vec(input) != Some(char)
                                || coord
                                    .apply_dir(Direction::Up)
                                    .apply_dir(Direction::Right)
                                    .apply_vec(input)
                                    == Some(char)
                        }
                        Direction::Up => {
                            coord.apply_dir(Direction::Left).apply_vec(input) != Some(char)
                                || coord
                                    .apply_dir(Direction::Up)
                                    .apply_dir(Direction::Left)
                                    .apply_vec(input)
                                    == Some(char)
                        }
                        Direction::Down => {
                            coord.apply_dir(Direction::Left).apply_vec(input) != Some(char)
                                || coord
                                    .apply_dir(Direction::Down)
                                    .apply_dir(Direction::Left)
                                    .apply_vec(input)
                                    == Some(char)
                        }
                    };
                    if is_side {
                        side += 1;
                    }
                }
            }

            let prev_perimeter = perimeters.get(&id).unwrap_or(&0);
            perimeters.insert(id, prev_perimeter + perimeter);
            let prev_side = sides.get(&id).unwrap_or(&0);
            sides.insert(id, prev_side + side);
            let area = areas.get(&id).unwrap_or(&0);
            areas.insert(id, area + 1);
        }

        let areas_zipped = areas
            .iter()
            .map(|(id, area)| (id_to_char.get(id).unwrap(), area, sides.get(id).unwrap()))
            .collect_vec();
        // println!("{:?}", areas_zipped);

        Ok(areas_zipped
            .into_iter()
            .map(|(_, area, sides)| area * sides)
            .sum())
    }
}

fn fill_id(
    input: &Vec<Vec<char>>,
    coord: Coordinate,
    id: usize,
    ids: &mut HashMap<Coordinate, usize>,
) {
    ids.insert(coord.clone(), id);
    let outer = coord.apply_vec(input).unwrap();
    for next in coord.cardinals() {
        let next_id = ids.get(&next);
        if next_id.is_none() {
            if let Some(inner) = next.apply_vec(input) {
                if inner == outer {
                    fill_id(input, next, id, ids);
                }
            }
        }
    }
}
