use itertools::Itertools;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

#[derive(Debug, Clone)]
pub enum Operation {
    XOR(String, String),
    OR(String, String),
    AND(String, String),
    Hard(u8),
}

impl Operation {
    fn to_str_rec(&self, tag: &str, others: &HashMap<String, Operation>) -> String {
        match self {
            Operation::Hard(_) => format!("{tag}"),
            Operation::AND(l, r) => format!(
                "({} & {})",
                others.get(l).unwrap().to_str_rec(l, others),
                others.get(r).unwrap().to_str_rec(r, others)
            ),
            Operation::OR(l, r) => format!(
                "({} | {})",
                others.get(l).unwrap().to_str_rec(l, others),
                others.get(r).unwrap().to_str_rec(r, others)
            ),
            Operation::XOR(l, r) => format!(
                "({} ^ {})",
                others.get(l).unwrap().to_str_rec(l, others),
                others.get(r).unwrap().to_str_rec(r, others)
            ),
        }
    }
}

fn get_value(
    key: &String,
    ops: &HashMap<String, Operation>,
    values: &mut HashMap<String, u8>,
) -> u8 {
    if values.contains_key(key) {
        return *values.get(key).unwrap();
    }
    values.insert(key.clone(), 0);

    let op = ops.get(key).unwrap();
    let result = match op {
        Operation::XOR(l, r) => get_value(&l, ops, values) ^ get_value(&r, ops, values),
        Operation::OR(l, r) => get_value(&l, ops, values) | get_value(&r, ops, values),
        Operation::AND(l, r) => get_value(&l, ops, values) & get_value(&r, ops, values),
        Operation::Hard(v) => *v,
    };
    values.insert(key.clone(), result);
    result
}

fn get_full_value(
    prefix: &str,
    ops: &HashMap<String, Operation>,
    values: &mut HashMap<String, u8>,
) -> usize {
    let mut result = 0;

    for i in 0.. {
        let key = format!("{prefix}{:0>2}", i);
        if !ops.contains_key(&key) {
            break;
        }
        result = result | ((get_value(&key, ops, values) as usize) << i)
    }

    result
}

fn test_half_add(position: usize, key: &String, ops: &HashMap<String, Operation>) -> bool {
    let x_key = format!("x{:0>2}", position);
    let y_key = format!("y{:0>2}", position);

    let mut ops = ops.clone();
    for x in 0..2 {
        ops.insert(x_key.clone(), Operation::Hard(x));
        for y in 0..2 {
            ops.insert(y_key.clone(), Operation::Hard(y));
            if get_value(&key, &ops, &mut HashMap::new()) != x ^ y {
                return false;
            }
        }
    }

    true
}

fn test_carry(
    position: usize,
    key: &String,
    prev_carry: &Option<String>,
    ops: &HashMap<String, Operation>,
) -> bool {
    let x_key = format!("x{:0>2}", position - 1);
    let y_key = format!("y{:0>2}", position - 1);

    let mut ops = ops.clone();

    if let Some(prev_carry) = prev_carry {
        for c in 0..2 {
            ops.insert(prev_carry.clone(), Operation::Hard(c));
            for x in 0..2 {
                ops.insert(x_key.clone(), Operation::Hard(x));
                for y in 0..2 {
                    ops.insert(y_key.clone(), Operation::Hard(y));
                    let v = get_value(&key, &ops, &mut HashMap::new());
                    let correct = (x & y) | ((x ^ y) & c);
                    if v != correct {
                        return false;
                    }
                }
            }
        }
    } else {
        for x in 0..2 {
            ops.insert(x_key.clone(), Operation::Hard(x));
            for y in 0..2 {
                ops.insert(y_key.clone(), Operation::Hard(y));
                if get_value(&key, &ops, &mut HashMap::new()) != (x & y) {
                    return false;
                }
            }
        }
    }

    true
}

pub enum TestResult {
    Ok(String),
    Root,
    Carry(String),
    Half(String),
}

fn test_position(
    position: usize,
    prev_carry: &Option<String>,
    ops: &HashMap<String, Operation>,
) -> TestResult {
    let key = format!("z{:0>2}", position);

    let op = ops.get(&key).unwrap();
    if let Operation::XOR(l, r) = op {
        let carry = if test_half_add(position, l, ops) {
            r
        } else if test_half_add(position, r, ops) {
            l
        } else {
            if test_carry(position, l, prev_carry, ops) {
                return TestResult::Half(r.clone());
            } else if test_carry(position, r, prev_carry, ops) {
                return TestResult::Half(l.clone());
            }
            println!("Both are bad!");
            return TestResult::Root;
        };

        if !test_carry(position, carry, prev_carry, ops) {
            return TestResult::Carry(carry.clone());
        }

        TestResult::Ok(carry.clone())
    } else {
        TestResult::Root
    }
}

fn find_half_add(position: usize, ops: &HashMap<String, Operation>) -> Option<String> {
    for k in ops.keys() {
        if test_half_add(position, k, ops) {
            return Some(k.clone());
        }
    }
    None
}
fn find_carry(
    position: usize,
    prev_carry: &Option<String>,
    ops: &HashMap<String, Operation>,
) -> Option<String> {
    for k in ops.keys() {
        if test_carry(position, k, prev_carry, ops) {
            return Some(k.clone());
        }
    }
    None
}

impl Solver for Problem {
    type Input = HashMap<String, Operation>;
    type Output1 = usize;
    type Output2 = String;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut result: HashMap<String, Operation> = HashMap::new();
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let mut mode = 0;
        for l in lines {
            if l == "" {
                mode = 1;
                continue;
            }
            if mode == 0 {
                let (k, v) = l.split(": ").collect_tuple().unwrap();
                result.insert(
                    String::from_str(k).unwrap(),
                    Operation::Hard(v.parse().unwrap()),
                );
            } else {
                let (op, k) = l.split(" -> ").collect_tuple().unwrap();
                let (kl, op, kr) = op.split(" ").collect_tuple().unwrap();

                let (kl, kr) = (String::from_str(kl).unwrap(), String::from_str(kr).unwrap());
                if result.contains_key(k) {
                    panic!("Lol trick!");
                }
                result.insert(
                    String::from_str(k).unwrap(),
                    match op {
                        "XOR" => Operation::XOR(kl, kr),
                        "OR" => Operation::OR(kl, kr),
                        "AND" => Operation::AND(kl, kr),
                        _ => panic!(),
                    },
                );
            }
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut values: HashMap<String, u8> = HashMap::new();
        Ok(get_full_value("z", input, &mut values))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // let swaps = find_swaps(input, 4, &HashSet::new());
        // return Ok(swaps.unwrap().iter().join(","));

        // println!("{}", input.get("z00").unwrap().to_str_rec("z00", input));
        // println!("{}", input.get("z01").unwrap().to_str_rec("z00", input));
        // println!("{}", input.get("z02").unwrap().to_str_rec("z00", input));
        // println!("{}", input.get("z03").unwrap().to_str_rec("z00", input));
        // println!("{}", input.get("z04").unwrap().to_str_rec("z00", input));
        // println!("{}", input.get("z05").unwrap().to_str_rec("z00", input));

        let mut prev_carry = None;
        let mut ops = input.clone();
        for i in 1.. {
            let key = format!("z{:0>2}", i);
            if !ops.contains_key(&key) {
                break;
            }
            let result = test_position(i, &prev_carry, &ops);
            match result {
                TestResult::Ok(next_carry) => prev_carry = Some(next_carry),
                TestResult::Root => {
                    // todo!("Root {i}")
                }
                TestResult::Carry(wrong_carry) => {
                    let half = find_carry(i, &prev_carry, &ops).unwrap();
                    println!("Swap {half} {wrong_carry}");
                    let (a, b) = (
                        ops.remove(&wrong_carry).unwrap(),
                        ops.remove(&half).unwrap(),
                    );
                    ops.insert(wrong_carry, b);
                    ops.insert(half, a);
                }
                TestResult::Half(wrong_half) => {
                    let half = find_half_add(i, &ops).unwrap();
                    println!("Swap {half} {wrong_half}");
                    let (a, b) = (ops.remove(&wrong_half).unwrap(), ops.remove(&half).unwrap());
                    ops.insert(wrong_half, b);
                    ops.insert(half, a);
                }
            }
        }

        todo!()
    }
}

fn count_bits(value: usize) -> usize {
    let mut result = 0;
    let mut value = value;
    while value > 0 {
        if value & 0x1 == 0 {
            result += 1;
        }
        value = value >> 1;
    }
    result
}

fn find_swaps(
    ops: &HashMap<String, Operation>,
    swaps_left: usize,
    swaps: &HashSet<String>,
) -> Option<HashSet<String>> {
    let mut values: HashMap<String, u8> = HashMap::new();
    let (x, y, z) = (
        get_full_value("x", &ops, &mut values),
        get_full_value("y", &ops, &mut values),
        get_full_value("z", &ops, &mut values),
    );
    let actual_sum = x + y;
    let mut diff = z ^ actual_sum;
    println!("{:0b}", diff);
    todo!();

    if diff == 0 {
        return Some(HashSet::new());
    } else if swaps_left == 0 {
        return None;
    }

    let mut i = 0;
    while (diff & 0x1) == 0 {
        i += 1;
        diff = diff >> 1;
    }
    let target = format!("z{:0>2}", i);
    let mut visited = HashSet::new();
    let gates = find_gates(&target, ops, &mut visited);
    let correct_bit = (actual_sum >> i) & 0x1;
    // let initial_bits = count_bits(diff);

    for perm in gates.iter().permutations(2) {
        if perm[0] == perm[1] {
            continue;
        }
        if swaps_left >= 4 {
            println!("Next {swaps_left} {:?}", perm);
        }

        let mut ops_copy = ops.clone();
        let (a, b) = (
            ops_copy.remove(perm[0]).unwrap(),
            ops_copy.remove(perm[1]).unwrap(),
        );
        ops_copy.insert(perm[0].clone(), b);
        ops_copy.insert(perm[1].clone(), a);
        let mut swaps_copy = swaps.clone();
        swaps_copy.insert(perm[0].clone());
        swaps_copy.insert(perm[1].clone());

        let mut values: HashMap<String, u8> = HashMap::new();
        let new_value = get_full_value("z", &ops_copy, &mut values);
        if (new_value >> i) & 0x1 != correct_bit {
            // if count_bits(new_value ^ actual_sum) > initial_bits {
            continue;
        }

        let result = find_swaps(&ops_copy, swaps_left - 1, &swaps_copy);
        if result.is_some() {
            let mut result = result.unwrap();
            result.insert(perm[0].clone());
            result.insert(perm[1].clone());
            return Some(result);
        }
    }

    None
}

fn find_gates(
    start: &String,
    ops: &HashMap<String, Operation>,
    visited: &mut HashSet<String>,
) -> HashSet<String> {
    if visited.contains(start) {
        return HashSet::new();
    }
    visited.insert(start.clone());

    let top_gate = ops.get(start).unwrap();
    let (l, r) = match top_gate {
        Operation::AND(l, r) => (find_gates(l, ops, visited), find_gates(r, ops, visited)),
        Operation::XOR(l, r) => (find_gates(l, ops, visited), find_gates(r, ops, visited)),
        Operation::OR(l, r) => (find_gates(l, ops, visited), find_gates(r, ops, visited)),
        _ => return HashSet::new(),
    };

    vec![start]
        .into_iter()
        .chain(l.iter())
        .chain(r.iter())
        .cloned()
        .collect()
}
