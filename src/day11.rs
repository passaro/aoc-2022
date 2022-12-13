use std::str::FromStr;

use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day11 {
    monkeys: Vec<Monkey>,
    factor: usize,
}

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let monkeys = lines.chunks(7)
        .into_iter()
        .map(|chunk| {
            let block = chunk.collect_vec();
            if !block[0].starts_with("Monkey") {
                panic!("not a monkey")
            }

            let items = block[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|i| i.parse().expect("invalid item"))
                .collect();

            let update = block[2]
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .parse()
                .expect("invalid operation");

            let divisible = block[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .expect("invalid test");

            let throw_if_true = block[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .expect("invalid true branch");

            let throw_if_false = block[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .expect("invalid false branch");

            Monkey { items, update, divisible, throw_if_true, throw_if_false }
        }).collect_vec();

        let factor = monkeys.iter().map(|m| m.divisible).product();
        
        Self { monkeys, factor }
    }
}

impl DaySolution for Day11 {
    fn part_one(&self) -> Solution {
        let monkey_business = self.monkey_business(20, 3);
        Solution::Unsigned(monkey_business)
    }

    fn part_two(&self) -> Solution {
        let monkey_business = self.monkey_business(10000, 1);
        Solution::Unsigned(monkey_business)
    }
}

impl Day11 {
    fn monkey_business(&self, rounds: usize, divide_worry_by: usize) -> usize {
        let mut busy_monkeys = vec![0usize; self.monkeys.len()];
        let mut monkeys = self.monkeys.clone();
        for _ in 0..rounds {
            for m in 0..monkeys.len() {
                let monkey = monkeys[m].clone();

                for item in &monkey.items {
                    let new = monkey.update.apply(*item);
                    let divided = (new / divide_worry_by) % self.factor;
                    let receiver = if divided % monkey.divisible == 0 {
                        monkey.throw_if_true
                    } else {
                        monkey.throw_if_false
                    };
                    monkeys[receiver].items.push(divided);
                }

                busy_monkeys[m] += monkey.items.len();
                monkeys[m].items.clear();
            }
        }

        busy_monkeys.sort_by(|a, b| b.partial_cmp(a).unwrap());
        busy_monkeys.iter().take(2).product()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    update: Update,
    divisible: usize,
    throw_if_true: usize,
    throw_if_false: usize,
}

#[derive(Debug, Clone, Copy)]
struct Update {
    operation: char,
    value: Option<usize>   
}

impl Update {
    fn apply(&self, old: usize) -> usize {
        let other = self.value.unwrap_or(old);
        match self.operation {
            '+' => old + other,
            '*' => old * other,
            _ => panic!("invalid op"),
        }
    }
}

impl FromStr for Update {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s.chars().next().unwrap();
        let value = s[2..].parse().ok();
        Ok(Self { operation, value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_sample() {
        let day = Day11::from_sample(SAMPLE);
        day.solve();
    }
}
