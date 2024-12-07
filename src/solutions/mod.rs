mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
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
        _ => panic!("day not implemented"),
    }
}
