use itertools::Itertools;

use super::Solver;
use core::panic;
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

fn get_value(key: &str, ops: &HashMap<String, Operation>, values: &mut HashMap<String, u8>) -> u8 {
    if values.contains_key(key) {
        return *values.get(key).unwrap();
    }
    values.insert(key.to_string(), 0);

    let op = ops.get(key).unwrap();
    let result = match op {
        Operation::XOR(l, r) => get_value(&l, ops, values) ^ get_value(&r, ops, values),
        Operation::OR(l, r) => get_value(&l, ops, values) | get_value(&r, ops, values),
        Operation::AND(l, r) => get_value(&l, ops, values) & get_value(&r, ops, values),
        Operation::Hard(v) => *v,
    };
    values.insert(key.to_string(), result);
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
fn find_half_add(
    position: usize,
    ops: &HashMap<String, Operation>,
    fixed: &HashSet<String>,
) -> Option<String> {
    for k in ops.keys() {
        if !fixed.contains(k) && test_half_add(position, k, ops) {
            return Some(k.clone());
        }
    }
    None
}

fn test_and(key_a: &str, key_b: &str, key: &str, ops: &HashMap<String, Operation>) -> bool {
    let mut ops = ops.clone();
    for x in 0..2 {
        ops.insert(key_a.to_string(), Operation::Hard(x));
        for y in 0..2 {
            ops.insert(key_b.to_string(), Operation::Hard(y));
            if get_value(key, &ops, &mut HashMap::new()) != x & y {
                return false;
            }
        }
    }

    true
}
fn find_and(
    key_a: &str,
    key_b: &str,
    ops: &HashMap<String, Operation>,
    fixed: &HashSet<String>,
) -> Option<String> {
    for k in ops.keys() {
        if !fixed.contains(k) && test_and(key_a, key_b, k, ops) {
            return Some(k.clone());
        }
    }
    None
}

fn test_carry(
    position: usize,
    key: &str,
    prev_v: &str,
    prev_c: &str,
    ops: &HashMap<String, Operation>,
) -> bool {
    let x_p = format!("x{:0>2}", position - 1);
    let y_p = format!("y{:0>2}", position - 1);
    let value = ops.get(key).unwrap();
    if let Operation::OR(l, r) = value {
        (test_and(&x_p, &y_p, l, ops) && test_and(prev_v, prev_c, r, ops))
            || (test_and(&x_p, &y_p, r, ops) && test_and(prev_v, prev_c, l, ops))
    } else {
        false
    }
}

fn solve_carry(
    position: usize,
    key: &str,
    prev_v: &str,
    prev_c: &str,
    ops: &mut HashMap<String, Operation>,
    fixed: &mut HashSet<String>,
    swaps: &mut HashSet<String>,
) -> String {
    let x_p = format!("x{:0>2}", position - 1);
    let y_p = format!("y{:0>2}", position - 1);

    let value = ops.get(key).unwrap().clone();
    if let Operation::OR(l, r) = value {
        let (yx, vc) = if test_and(&x_p, &y_p, &l, ops) || test_and(prev_v, prev_c, &r, ops) {
            (l, r)
        } else if test_and(&x_p, &y_p, &r, ops) || test_and(prev_v, prev_c, &l, ops) {
            (r, l)
        } else {
            todo!("both are bad!");
        };

        if !test_and(&x_p, &y_p, &yx, ops) {
            let correct_yx = find_and(&x_p, &y_p, ops, fixed).unwrap();
            swap(&yx, &correct_yx, ops, fixed, swaps);
        } else if !test_and(prev_v, prev_c, &vc, ops) {
            if let Operation::AND(l, r) = ops.get(&vc).unwrap().clone() {
                if l == prev_v {
                    swap(&r, &prev_c.to_string(), ops, fixed, swaps);
                } else if l == prev_c {
                    swap(&r, &prev_v.to_string(), ops, fixed, swaps);
                } else if r == prev_v {
                    swap(&l, &prev_c.to_string(), ops, fixed, swaps);
                } else if r == prev_c {
                    swap(&l, &prev_v.to_string(), ops, fixed, swaps);
                }
            } else {
                let correct_vc = find_and(prev_v, prev_c, ops, fixed).unwrap();
                swap(&vc, &correct_vc, ops, fixed, swaps);
            }
        }

        return key.to_string();
    } else {
        for k in ops.keys() {
            if !fixed.contains(k) && test_carry(position, k, prev_v, prev_c, ops) {
                return k.clone();
            }
        }
    }

    panic!("No suitable carry found");
}

/*
zN <- vN ^ cN
vN <- xN ^ yN
cN <- ((yP & xP) | (vP & cP))
*/
#[derive(Debug)]
pub struct LineSolution {
    value: String,
    carry: Option<String>,
}

fn swap(
    a_key: &String,
    b_key: &String,
    ops: &mut HashMap<String, Operation>,
    fixed: &mut HashSet<String>,
    swaps: &mut HashSet<String>,
) {
    let (a, b) = (ops.remove(a_key).unwrap(), ops.remove(b_key).unwrap());
    ops.insert(a_key.clone(), b);
    ops.insert(b_key.clone(), a);
    fixed.insert(a_key.clone());
    fixed.insert(b_key.clone());
    swaps.insert(a_key.clone());
    swaps.insert(b_key.clone());
}
fn solve_position(
    position: usize,
    prev: Option<LineSolution>,
    ops: &mut HashMap<String, Operation>,
    fixed: &mut HashSet<String>,
    swaps: &mut HashSet<String>,
) -> LineSolution {
    let key = format!("z{:0>2}", position);
    let value = ops.get(&key).unwrap();

    if let Some(prev) = prev {
        let prev_value = prev.value;
        if let Some(prev_carry) = prev.carry {
            // test vN -> If not, then find vN
            // test cN -> If not, then find cN
            let key = format!("z{:0>2}", position);
            let test_result = test_position(position, &key, &prev_value, &prev_carry, ops);
            match test_result {
                TestResult::Ok { carry, value } => {
                    fixed.insert(value.clone());
                    fixed.insert(carry.clone());
                    LineSolution {
                        value,
                        carry: Some(carry),
                    }
                }
                TestResult::Value { value, carry } => {
                    let correct_value = find_half_add(position, ops, &fixed).unwrap();
                    swap(&value, &correct_value, ops, fixed, swaps);
                    LineSolution {
                        value: correct_value,
                        carry: Some(carry),
                    }
                }
                TestResult::Carry { carry, value } => {
                    let correct = solve_carry(
                        position,
                        &carry,
                        &prev_value,
                        &prev_carry,
                        ops,
                        fixed,
                        swaps,
                    );
                    LineSolution {
                        value,
                        carry: Some(correct),
                    }
                }
                TestResult::Root => {
                    println!("Find root");
                    let (correct, value, carry) =
                        find_position(position, &prev_value, &prev_carry, &ops, &fixed).unwrap();
                    swap(&correct, &key, ops, fixed, swaps);
                    LineSolution {
                        value,
                        carry: Some(carry),
                    }
                }
            }
        } else {
            if let Operation::XOR(l, r) = value {
                fixed.insert(l.clone());
                fixed.insert(r.clone());
                LineSolution {
                    value: l.clone(),
                    carry: Some(r.clone()),
                }
            } else {
                todo!("Second")
            }
        }
    } else {
        if !test_half_add(position, &key, ops) {
            todo!("First")
        } else {
            fixed.insert(key.clone());
            LineSolution {
                value: key.clone(),
                carry: None,
            }
        }
    }
}

#[derive(Debug)]
pub enum TestResult {
    Ok { carry: String, value: String },
    Root,
    Carry { carry: String, value: String },
    Value { carry: String, value: String },
}

fn test_position(
    position: usize,
    key: &str,
    prev_v: &str,
    prev_c: &str,
    ops: &HashMap<String, Operation>,
) -> TestResult {
    let op = ops.get(key).unwrap();
    if let Operation::XOR(l, r) = op {
        let (value, carry) = if test_half_add(position, l, ops) {
            (l, r)
        } else if test_half_add(position, r, ops) {
            (r, l)
        } else {
            if test_carry(position, l, prev_v, prev_c, ops) {
                return TestResult::Value {
                    value: r.clone(),
                    carry: l.clone(),
                };
            } else if test_carry(position, r, prev_v, prev_c, ops) {
                return TestResult::Value {
                    value: l.clone(),
                    carry: r.clone(),
                };
            }
            // println!("root => both!");
            return TestResult::Root;
        };

        if !test_carry(position, carry, prev_v, prev_c, ops) {
            return TestResult::Carry {
                carry: carry.clone(),
                value: value.clone(),
            };
        }

        TestResult::Ok {
            carry: carry.clone(),
            value: value.clone(),
        }
    } else {
        TestResult::Root
    }
}
fn find_position(
    position: usize,
    prev_v: &str,
    prev_c: &str,
    ops: &HashMap<String, Operation>,
    fixed: &HashSet<String>,
) -> Option<(String, String, String)> {
    for k in ops.keys() {
        if !fixed.contains(k) {
            if let TestResult::Ok { carry, value } =
                test_position(position, &k, prev_v, prev_c, ops)
            {
                return Some((k.clone(), value, carry));
            }
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

        // let mut prev_carry = None;
        let mut ops = input.clone();
        let mut fixed = HashSet::new();
        let mut swaps = HashSet::new();
        let mut prev = None;
        for i in 0.. {
            let key = format!("z{:0>2}", i);
            if !ops.contains_key(&key) {
                break;
            }

            println!("{i}, {:?}", swaps.iter().sorted().join(","));
            prev = Some(solve_position(i, prev, &mut ops, &mut fixed, &mut swaps));
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
