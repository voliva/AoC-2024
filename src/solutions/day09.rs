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
            .fold(0, |a, b| a + b);

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

        let mut start = 0;
        let mut end = result.len() - 1;
        while start < end {
            while start < end && result[start].is_some() {
                start += 1;
            }
            while start < end && result[end].is_none() {
                end -= 1;
            }
            if start < end {
                result[start] = result[end];
                result[end] = None;
                start += 1;
                end -= 1;
            }
        }

        Ok(result
            .iter()
            .enumerate()
            .map(|(i, v)| i * v.unwrap_or(0))
            .fold(0, |a, b| a + b))
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
            .map(|(id, block)| {
                (block.position..(block.position + block.length))
                    .map(|i| i * id)
                    .fold(0, |a, b| a + b)
            })
            .fold(0, |a, b| a + b))
    }
}

struct Block {
    position: usize,
    length: usize,
}
