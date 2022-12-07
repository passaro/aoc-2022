mod day1;
use day1::Day1;
mod day2;
use day2::Day2;
mod day3;
use day3::Day3;
mod day4;
use day4::Day4;
mod day5;
use day5::Day5;
mod day6;
use day6::Day6;
mod day7;
use day7::Day7;


use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

/// Reads the lines from the input file into a relevant
/// model of the data for the day's solution.
trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;

    fn from_sample(sample: &str) -> Self 
    where Self: Sized {
        Self::from_lines(sample.lines().map(|l| l.to_owned()))
    }
}

/// Solutions for a day of Advent of Code.
trait DaySolution {
    fn part_one(&self) -> Option<String>;
    fn part_two(&self) -> Option<String>;

    fn solve(&self) {
        let (part_one, duration) = time_execution(|| self.part_one());
        println!("Part 1: {} ({} seconds)", part_one.as_deref().unwrap_or("<missing>"), duration.as_secs_f32());

        let (part_two, duration) = time_execution(|| self.part_two());
        println!("Part 2: {} ({} seconds)", part_two.as_deref().unwrap_or("<missing>"), duration.as_secs_f32());
    }
}

/// Reads the input for a day from the `.input` directory.
fn load_input(day: usize) -> impl Iterator<Item = String> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!(".input/{day}.txt"))
        .expect(&format!("Failed to access data for day {}", day));
    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Failed to read line from data file"))
}

/// Gets the solution for the given day as a trait object.
fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        1 => Box::new(Day1::from_lines(lines)),
        2 => Box::new(Day2::from_lines(lines)),
        3 => Box::new(Day3::from_lines(lines)),
        4 => Box::new(Day4::from_lines(lines)),
        5 => Box::new(Day5::from_lines(lines)),
        6 => Box::new(Day6::from_lines(lines)),
        7 => Box::new(Day7::from_lines(lines)),
        _other => panic!("Day {} hasn't been solved yet", day),
    }
}

/// Times the execution of a function.
fn time_execution<T>(work: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = work();
    let duration = start.elapsed();

    (result, duration)
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Must provide a day to solve")
        .parse::<usize>()
        .expect("Provided day wasn't a valid integer");

    let input = load_input(day);
    let solution = get_day_solution(day, input);
    println!("Solving day {day}...");
    solution.solve();
}
