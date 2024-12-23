use itertools::Itertools;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

fn find_3_interconnections(
    input: &HashMap<String, HashSet<String>>,
    start: String,
    result: &mut HashSet<String>,
) {
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.clone());

    for key in input.get(&start).unwrap_or(&HashSet::new()) {
        for key2 in input.get(key).unwrap_or(&HashSet::new()) {
            if visited.contains(key2) {
                continue;
            }

            if input.get(key2).unwrap_or(&HashSet::new()).contains(&start) {
                let mut group = vec![&start, key, key2];
                group.sort();
                result.insert(group.into_iter().join("-"));
            }
        }
        visited.insert(key.clone());
    }
}

// Find cliques
fn bron_kerbosch_rec(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    result: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        result.push(r.clone());
    }

    while !p.is_empty() {
        let v = p.iter().next().unwrap().clone();

        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let mut new_p = HashSet::new();
        for pi in p.iter() {
            if graph.get(&v).and_then(|set| set.get(pi)).is_some() {
                new_p.insert(pi.clone());
            }
        }

        let mut new_x = HashSet::new();
        for xi in x.iter() {
            if graph.get(&v).and_then(|set| set.get(xi)).is_some() {
                new_x.insert(xi.clone());
            }
        }
        bron_kerbosch_rec(&new_r, &mut new_p, &mut new_x, graph, result);

        p.remove(&v);
        x.insert(v);
    }
}
fn bron_kerbosch(graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut result = vec![];

    let r = HashSet::new();
    let mut p = graph.keys().cloned().collect();
    let mut x = HashSet::new();

    bron_kerbosch_rec(&r, &mut p, &mut x, graph, &mut result);

    result
}

impl Solver for Problem {
    type Input = HashMap<String, HashSet<String>>;
    type Output1 = usize;
    type Output2 = String;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut result: Self::Input = HashMap::new();
        for line in lines {
            let (a, b) = line.split("-").collect_tuple().unwrap();

            result
                .entry(String::from_str(a).unwrap())
                .and_modify(|vec| {
                    vec.insert(String::from_str(b).unwrap());
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(String::from_str(b).unwrap());
                    set
                });
            result
                .entry(String::from_str(b).unwrap())
                .and_modify(|vec| {
                    vec.insert(String::from_str(a).unwrap());
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(String::from_str(a).unwrap());
                    set
                });
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut result = HashSet::new();

        for start in input.keys() {
            if start.starts_with("t") {
                find_3_interconnections(input, start.clone(), &mut result)
            }
        }

        Ok(result.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let result = bron_kerbosch(input);

        let mut max = result
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .into_iter()
            .cloned()
            .collect_vec();
        max.sort();

        Ok(max.join(","))
    }
}
