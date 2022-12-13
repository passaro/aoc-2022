use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day3 {
    items: Vec<String>,
}

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self { items: lines.collect() }
    }
}

fn priority(item: &char) -> u32 {
    const UPPER_CASE_A: u32 = 65;
    const LOWER_CASE_A: u32 = 97;

    let v = *item as u32; 
    if v < LOWER_CASE_A {
        v - UPPER_CASE_A + 27
    } else {
        v - LOWER_CASE_A + 1
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> Solution {
        fn find_misplaced(rucksack: &str) -> Option<char> {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let first = first.chars().collect::<HashSet<char>>();
            let second = second.chars().collect();
            let misplaced = first.intersection(&second);
            misplaced.into_iter().next().map(|c| *c)
        }

        let sum = self.items.iter()
            .filter_map(|items| find_misplaced(items))
            .map(|item| { priority(&item)} )
            .sum::<u32>();

        Solution::Unsigned(sum as usize)
    }

    fn part_two(&self) -> Solution {
        let sum = self.items.iter()
            .chunks(3).into_iter()
            .filter_map(|team| team.map(|r| 
                r.chars().collect::<HashSet<char>>()).reduce(|acc, item| acc.intersection(&item).map(|c| *c).collect()))
            .filter_map(|items| items.into_iter().next())
            .map(|item| { priority(&item)} )
            .sum::<u32>();

        Solution::Unsigned(sum as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_sample() {
        let day = Day3::from_sample(SAMPLE);
        day.solve();
    }
}
