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
mod solution;
use solution::{DaySolution, FromInput, Solution};

use std::{env, io::{BufReader, BufRead}};

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

fn solve_day(day: usize, lines: impl Iterator<Item = String>) -> (Solution, Solution) {
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
        assert_eq!(part_one, Solution::Unsigned(71300));
        assert_eq!(part_two, Solution::Unsigned(209691));
    }

    #[test]
    fn test_day13() {
        let (part_one, part_two) = solve_day(13, load_input(13));
        assert_eq!(part_one, Solution::Unsigned(5208));
        assert_eq!(part_two, Solution::Unsigned(25792));
    }
}
