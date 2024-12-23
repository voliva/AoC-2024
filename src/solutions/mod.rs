mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod solver;

pub use solver::Solver;

pub fn solve(day: usize, parts: usize) {
    let filename = format!("inputs/{:02}", day);
    match day {
        1 => day01::Problem.solve(filename, parts),
        2 => day02::Problem.solve(filename, parts),
        3 => day03::Problem.solve(filename, parts),
        4 => day04::Problem.solve(filename, parts),
        5 => day05::Problem.solve(filename, parts),
        6 => day06::Problem.solve(filename, parts),
        7 => day07::Problem.solve(filename, parts),
        8 => day08::Problem.solve(filename, parts),
        9 => day09::Problem.solve(filename, parts),
        10 => day10::Problem.solve(filename, parts),
        11 => day11::Problem.solve(filename, parts),
        12 => day12::Problem.solve(filename, parts),
        13 => day13::Problem.solve(filename, parts),
        14 => day14::Problem.solve(filename, parts),
        15 => day15::Problem.solve(filename, parts),
        16 => day16::Problem.solve(filename, parts),
        17 => day17::Problem.solve(filename, parts),
        18 => day18::Problem.solve(filename, parts),
        19 => day19::Problem.solve(filename, parts),
        20 => day20::Problem.solve(filename, parts),
        21 => day21::Problem.solve(filename, parts),
        22 => day22::Problem.solve(filename, parts),
        23 => day23::Problem.solve(filename, parts),
        24 => day24::Problem.solve(filename, parts),
        _ => panic!("day not implemented"),
    }
}
