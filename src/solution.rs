use std::{time::{Duration, Instant}, fmt::Display, io::{BufReader, BufRead}};

/// Solutions for a day of Advent of Code.
pub trait DaySolution {
    fn part_one(&self) -> Solution;
    fn part_two(&self) -> Solution;

    fn solve(&self) -> (Solution, Solution) {
        let (part_one, duration) = time_execution(|| self.part_one());
        println!("Part 1: {} ({} seconds)", part_one, duration.as_secs_f32());

        let (part_two, duration) = time_execution(|| self.part_two());
        println!("Part 2: {} ({} seconds)", part_two, duration.as_secs_f32());

        (part_one, part_two)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    NotImplemented,
    Signed(isize),
    Unsigned(usize),
    String(String),
}

impl Default for Solution {
    fn default() -> Self {
        Self::NotImplemented
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::NotImplemented => f.write_str("<not implemented>"),
            Solution::Signed(i) => f.write_fmt(format_args!("{}", i)),
            Solution::Unsigned(u) => f.write_fmt(format_args!("{}", u)),
            Solution::String(s) => f.write_fmt(format_args!("{}", s)),
        }
    }
}

/// Reads the lines from the input file into a relevant
/// model of the data for the day's solution.
pub trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;

    fn from_sample(sample: &str) -> Self 
    where Self: Sized {
        Self::from_lines(sample.lines().map(|l| l.to_owned()))
    }
}

/// Times the execution of a function.
fn time_execution<T>(work: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = work();
    let duration = start.elapsed();

    (result, duration)
}

/// Reads the input for a day from the `.input` directory.
pub fn load_input(day: usize) -> impl Iterator<Item = String> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!(".input/{day}.txt"))
        .expect(&format!("Failed to access data for day {}", day));
    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Failed to read line from data file"))
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn test_day(day: impl DaySolution, solution_one: Solution, solution_two: Solution) {
        let (part_one, part_two) = day.solve();
        assert_eq!(part_one, solution_one);
        assert_eq!(part_two, solution_two);
    }
}