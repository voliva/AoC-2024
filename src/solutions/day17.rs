use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Clone, Debug)]
pub struct Machine {
    registers: [usize; 3],
    program: Vec<usize>,
    pc: usize,
    output: Vec<usize>,
}

impl Machine {
    fn step(&mut self) -> bool {
        let instruction = self.program[self.pc];
        let operand = self.program[self.pc + 1];
        self.pc += 2;

        match instruction {
            0 => self.registers[0] /= 1 << self.read_combo(operand),
            1 => self.registers[1] ^= operand,
            2 => self.registers[1] = self.read_combo(operand) % 8,
            3 => {
                if self.registers[0] != 0 {
                    self.pc = operand
                }
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push(self.read_combo(operand) % 8),
            6 => self.registers[1] = self.registers[0] / (1 << self.read_combo(operand)),
            7 => self.registers[2] = self.registers[0] / (1 << self.read_combo(operand)),
            _ => panic!(),
        }

        self.pc < self.program.len()
    }
    fn read_combo(&self, combo: usize) -> usize {
        match combo {
            0..=3 => combo,
            4..=6 => self.registers[combo - 4],
            _ => panic!("invalid operand"),
        }
    }
}

impl Solver for Problem {
    type Input = Machine;
    type Output1 = String;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let reg_a = lines[0][12..].parse().unwrap();
        let reg_b = lines[1][12..].parse().unwrap();
        let reg_c = lines[2][12..].parse().unwrap();
        let program = lines[4][9..]
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect_vec();

        Machine {
            output: vec![],
            pc: 0,
            program,
            registers: [reg_a, reg_b, reg_c],
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut machine = input.clone();

        // println!("{:?}", machine);
        while machine.step() {
            // println!("{:?}", machine);
        }
        // println!("{:?}", machine);

        Ok(machine.output.iter().join(","))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // It moves stuff around*, but tl;dr; value goes in groups of 3 bits
        // and the value it outputs depends on the higher positions of A
        // so we can just construct starting from the end.

        let missing_ouptut = input.program.clone().into_iter().rev().collect_vec();
        let mut dontjump = input.clone();
        dontjump.program.pop();
        dontjump.program.pop();

        let result = find_output(0, &mut dontjump, &missing_ouptut);

        Ok(result.unwrap())
    }
}

fn simulate_dontjump(dontjump: &Machine) -> usize {
    let mut machine = dontjump.clone();
    while machine.step() {}
    return machine.output[0];
}

/*
* spoilers
what the program does is read the low 3-bits of A
this gives a number that's the position to read another 3-bits within A
then combines the two values (with some transformation in-between)
outputs the result
and decreases the A number by 3 bits.
the program halts when A reaches 0.

It's easier to solve the problem if we look from the end. What's the last value it has to output?
Now, what's the value of A so that it outputs that value? because we're at the end, it has to be a number between 0 and 8.

This can create a map of A => output, but it's only valid for this last output, since the program can read from any position based on the value of A.
So we can just repeat this process by growing A 3-bits at a time.

It could happen that there are multiple possibilities in a round, that lead to a dead end.
So we also need some backtracking.
 */
fn find_output(a: usize, dontjump: &mut Machine, output: &[usize]) -> Option<usize> {
    let target = output[0];
    for v in 0..8 {
        let new_a = (a << 3) | v;
        dontjump.registers[0] = new_a;
        let dj = simulate_dontjump(&dontjump);
        if dj == target {
            if output.len() == 1 {
                return Some(new_a);
            }
            if let Some(v) = find_output(new_a, dontjump, &output[1..]) {
                return Some(v);
            }
        }
    }
    None
}
