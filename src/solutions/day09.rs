use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|line| {
                line.chars()
                    .map(|c| String::from(c).parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let length = input
            .iter()
            .enumerate()
            .map(|(_, v)| *v)
            .reduce(|a, b| a + b)
            .unwrap();

        let mut result: Vec<Option<usize>> = Vec::with_capacity(length);
        for (i, value) in input.iter().enumerate() {
            for _ in 0..*value {
                if i % 2 == 0 {
                    result.push(Some(i / 2));
                } else {
                    result.push(None)
                }
            }
        }

        let mut explored = 0;
        while let Some((start, length)) = find_end_block(&result, explored) {
            let gap = find_start_gap(&result, explored).unwrap_or(0);
            let id = result[start].unwrap();

            for i in start..(start + length) {
                result[i] = None;
            }
            let mut to_fill = length;
            for i in gap.. {
                if result[i].is_none() {
                    result[i] = Some(id);
                    to_fill -= 1;
                }
                if to_fill == 0 {
                    break;
                }
                explored = i;
            }
        }

        // println!(
        //     "{:?}",
        //     result
        //         .iter()
        //         .filter(|v| v.is_some())
        //         .map(|v| v.unwrap() % 10)
        //         .join("")
        // );

        // 6346805260911
        Ok(result
            .iter()
            .enumerate()
            .map(|(i, v)| i * v.unwrap_or(0))
            .reduce(|a, b| a + b)
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut gaps = Vec::new();
        let mut files = HashMap::new();
        let mut position = 0;
        let mut id = 0;
        for (i, length) in input.iter().enumerate() {
            let length = *length;
            if i % 2 == 0 {
                files.insert(id, Block { position, length });
                id += 1;
            } else {
                gaps.push(Block { position, length });
            }
            position += length;
        }
        let ids = id;

        for id in (0..ids).rev() {
            let block = files.get_mut(&id).unwrap();
            if let Some((gap_i, gap)) = gaps.iter().find_position(|gap| gap.length >= block.length)
            {
                if block.position < gap.position {
                    continue;
                }
                block.position = gap.position;
                if gap.length == block.length {
                    gaps.splice(gap_i..gap_i + 1, vec![]);
                } else {
                    gaps[gap_i].length -= block.length;
                    gaps[gap_i].position += block.length;
                }
            }
        }

        Ok(files
            .iter()
            .flat_map(|(id, block)| {
                (block.position..(block.position + block.length))
                    .map(|i| i * id)
                    .collect_vec()
            })
            .reduce(|a, b| a + b)
            .unwrap())
    }
}

struct Block {
    position: usize,
    length: usize,
}

fn find_end_block<T: PartialEq + Copy>(
    vec: &Vec<Option<T>>,
    explored: usize,
) -> Option<(usize, usize)> {
    let mut len = 0;
    let mut value = None;
    for i in (explored..vec.len()).rev() {
        if vec[i].is_some() {
            if value.is_some() && vec[i] != value {
                return Some((i + 1, len));
            }
            value = vec[i];
            len += 1;
        } else if vec[i].is_none() && len > 0 {
            return Some((i + 1, len));
        }
    }
    return None;
}
fn find_start_gap<T>(vec: &Vec<Option<T>>, explored: usize) -> Option<usize> {
    for i in explored..vec.len() {
        if vec[i].is_none() {
            return Some(i);
        }
    }
    return None;
}
