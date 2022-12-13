use take_until::TakeUntilExt;

use crate::solution::{DaySolution, FromInput, Solution};
use crate::grid::{Grid, Position};

#[derive(Debug)]
pub struct Day8 {
    heights: Grid<u8>,
}

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let heights = Grid::from_lines(lines, |c, _| {
            String::from(c).parse().expect("invalid height")
        });
        
        Self { heights }
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> Solution {
        let visible = self.heights.positions()
            .filter(|p| self.visible(p))
            .count();

        Solution::Unsigned(visible)
    }

    fn part_two(&self) -> Solution {
        let max_scenic_score = self.heights.positions()
            .map(|p| self.scenic_score(&p))
            .max();

        Solution::Unsigned(max_scenic_score.expect("no trees"))
    }
}

impl Day8 {
    fn visible(&self, pos: &Position) -> bool {
        let height = self.heights.at(pos);

        !(0..pos.y).any(|y| self.heights.at(&Position::new(pos.x, y)) >= height) ||
        !(0..pos.x).any(|x| self.heights.at(&Position::new(x, pos.y)) >= height) ||
        !((pos.y+1)..self.heights.row_count).any(|y| self.heights.at(&Position::new(pos.x, y)) >= height) ||
        !((pos.x+1)..self.heights.col_count).any(|x| self.heights.at(&Position::new(x, pos.y)) >= height)
    }


    fn scenic_score(&self, pos: &Position) -> usize {
        let height = self.heights.at(pos);

        let up = (0..pos.y).rev().take_until(|&y| self.heights.at(&Position::new(pos.x, y)) >= height).count();
        let down = ((pos.y+1)..self.heights.row_count).take_until(|&y| self.heights.at(&Position::new(pos.x, y)) >= height).count();
        let left = (0..pos.x).rev().take_until(|&x| self.heights.at(&Position::new(x, pos.y)) >= height).count();
        let right = ((pos.x+1)..self.heights.col_count).take_until(|&x| self.heights.at(&Position::new(x, pos.y)) >= height).count();

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
