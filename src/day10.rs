use std::{str::FromStr, iter::once};

use itertools::Itertools;

use crate::{DaySolution, FromInput};


pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let instructions = lines.map(|l| l.parse().expect("error")).collect();
        Self { instructions }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }

        if let Some(value) = s.strip_prefix("addx ") {
            if let Some(value) = value.parse().ok() {
                return Ok(Self::Addx(value));
            }
        }

        Err("error")
    }
}

impl DaySolution for Day10 {
    fn part_one(&self) -> Option<String> {

        let sum: isize = self.register_x_values()
            .enumerate()
            .skip(20).step_by(40)
            .map(|(cycle, x)| x * cycle as isize)
            .sum();
        
        Some(sum.to_string())
    }

    fn part_two(&self) -> Option<String> {
         let screen = self.register_x_values().skip(1)
            .zip((0isize..40).cycle())
            .map(|(x, p)| {
                if x.abs_diff(p) < 2 { "#" } else { "." }
            })
            .chunks(40).into_iter()
            .map(|line| format!("\n{}", line.into_iter().join("")))
            .join("");

        Some(screen)
    }
}

impl Day10 {
    fn register_x_values(&self) -> impl Iterator<Item = isize> + '_ {
        once(0)
            .chain(self.instructions.iter()
                .flat_map(|instruction| {
                    match instruction {
                        Instruction::Noop => { 
                            vec![ 0 as isize ]
                        },
                        Instruction::Addx(value) => {
                            vec![ 0 as isize, *value ]
                        },
                    }
                }))
            .scan(1 as isize, |state, value| {
                let x = *state;
                *state += value;
                Some(x)
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_sample() {
        let day = Day10::from_sample(SAMPLE);
        day.solve();
    }
}
