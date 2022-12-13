mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;


mod grid;

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

    fn solve(&self) -> (Option<String>, Option<String>) {
        let (part_one, duration) = time_execution(|| self.part_one());
        println!("Part 1: {} ({} seconds)", part_one.as_deref().unwrap_or("<missing>"), duration.as_secs_f32());

        let (part_two, duration) = time_execution(|| self.part_two());
        println!("Part 2: {} ({} seconds)", part_two.as_deref().unwrap_or("<missing>"), duration.as_secs_f32());

        (part_one, part_two)
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

fn solve_day(day: usize, lines: impl Iterator<Item = String>) -> (Option<String>, Option<String>) {
    macro_rules! day_name {
        ($value:expr) => {
            paste::paste! {
                [<day $value>]::[<Day $value>]
            }
        };
    }

    macro_rules! match_day_and_solve {
        ($day:ident, $lines:expr, $($value:expr),* $(,)?) => {
            {
                let day = $day;
                let lines = $lines;
                match day {
                $(
                    $value => <day_name!($value)>::from_lines(lines).solve(),
                )*
                    _other => panic!("Day {} hasn't been solved yet", day),
                }
            }
        };
    }

    match_day_and_solve!(day, lines, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13)
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
    println!("Solving day {day}...");
    solve_day(day, input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let (part_one, part_two) = solve_day(1, load_input(1));
        assert_eq!(part_one, Some("71300".to_string()));
        assert_eq!(part_two, Some("209691".to_string()));
    }
}
