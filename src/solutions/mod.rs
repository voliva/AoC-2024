mod day01;
mod solver;

pub use solver::Solver;

pub fn solve(day: usize, parts: usize) {
    let filename = format!("inputs/{:02}", day);
    match day {
        1 => day01::Problem.solve(filename, parts),
        _ => panic!("day not implemented"),
    }
}
