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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod grid;
mod bitset;
mod solution;

use std::env;

use solution::{DaySolution, FromInput, Solution};

use crate::solution::load_input;

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

    match_day_and_solve!(day, lines, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19)
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
