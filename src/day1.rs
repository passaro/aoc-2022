use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day1 {
    calories: Vec<u32>,
}

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut calories = Vec::new();
        let mut current = Vec::new();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                if !current.is_empty() {
                    calories.push(current);
                    current = Vec::new();
                }
            }
            else {
                let cal = line.parse().expect(&format!("invalid value: {}", line));
                current.push(cal);
            }
        }
        if !current.is_empty() {
            calories.push(current);
        }

        let calories = calories.iter()
            .map(|e| e.iter().sum())
            .collect_vec();
        
        Self { calories }
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> Solution {
        let max = self.calories
            .iter()
            .max()
            .expect("no elves");
        Solution::Unsigned(*max as usize)
    }

    fn part_two(&self) -> Solution {
        let sum_max_3: u32 = self.calories
            .iter()
            .sorted()
            .rev()
            .take(3)
            .sum();
        Solution::Unsigned(sum_max_3 as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    ";

    #[test]
    fn test_sample() {
        let day = Day1::from_sample(SAMPLE);
        day.solve();
    }
}