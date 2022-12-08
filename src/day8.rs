use itertools::Itertools;
use take_until::TakeUntilExt;

use crate::{DaySolution, FromInput};

#[derive(Debug)]
pub struct Day8 {
    col_count: usize,
    row_count: usize,
    heights: Vec<u8>,
}

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut heights = Vec::new();
        let mut col_count = 0;
        let mut row_count = 0;
        for line in lines.filter(|l| !l.trim().is_empty()) {
            col_count = line.len();
            row_count += 1;
            for h in line.chars().map(|c| String::from(c).parse().expect("invalid height")) {
                heights.push(h);
            }
        }

        Self { col_count, row_count, heights }
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> Option<String> {
        let mut visible = 0;
        for col in 0..self.col_count {
            for row in 0..self.row_count {
                if self.visible(row, col) {
                    visible += 1;
                }
            }
        }

        Some(visible.to_string())
    }

    fn part_two(&self) -> Option<String> {
        let max_scenic_score = (1..(self.row_count-1))
            .cartesian_product(1..(self.col_count-1))
            .map(|(r,c)| self.scenic_score(r, c))
            .max();

        Some(max_scenic_score.expect("no trees").to_string())
    }
}

impl Day8 {
    fn at(&self, row: usize, column: usize) -> u8 {
        self.heights[row * self.col_count + column]
    }

    fn visible(&self, row: usize, column: usize) -> bool {
        let height = self.at(row, column);

        !(0..row).any(|r| self.at(r, column) >= height) ||
        !(0..column).any(|c| self.at(row, c) >= height) ||
        !((row+1)..self.row_count).any(|r| self.at(r, column) >= height) ||
        !((column+1)..self.col_count).any(|c| self.at(row, c) >= height)
    }


    fn scenic_score(&self, row: usize, column: usize) -> usize {
        let height = self.at(row, column);

        let up = (0..row).rev().take_until(|&r| self.at(r, column) >= height).count();
        let down = ((row+1)..self.row_count).take_until(|&r| self.at(r, column) >= height).count();
        let left = (0..column).rev().take_until(|&c| self.at(row, c) >= height).count();
        let right = ((column+1)..self.col_count).take_until(|&c| self.at(row, c) >= height).count();

        up * down * left * right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_sample() {
        let day = Day8::from_sample(SAMPLE);
        day.solve();
    }
}
