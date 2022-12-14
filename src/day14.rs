use std::cmp::{min, max};

use itertools::Itertools;

use crate::{solution::{DaySolution, FromInput, Solution}, grid::{Position, Grid}};

pub struct Day14 {
    top_left: Position,
    bottom_right: Position,
    paths: Vec<Vec<Position>>,
}

impl FromInput for Day14 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut top_left = Position::new(usize::MAX, usize::MAX);
        let mut bottom_right = Position::new(0, 0);

        let paths = lines.map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(",").unwrap();
                    let x = x.parse().expect("invalid x");
                    let y = y.parse().expect("invalid y");
                    if x > bottom_right.x { bottom_right.x = x; }
                    if y > bottom_right.y { bottom_right.y = y; }
                    if x < top_left.x { top_left.x = x; }
                    if y < top_left.y { top_left.y = y; }
                    Position::new(x, y)
                })
                .collect_vec()
        })
        .collect_vec();

        Self { top_left, bottom_right, paths }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl DaySolution for Day14 {
    fn part_one(&self) -> Solution {
        let (mut grid, start) = self.create_grid();

        let mut count = 0;
        while drop_sand(&mut grid, start) {
            count += 1;
        }

        Solution::Unsigned(count)
    }

    fn part_two(&self) -> Solution {
        let (mut grid, start) = self.create_grid_with_floor();

        let mut count = 1;
        while drop_sand_floor(&mut grid, start) {
            count += 1;
        }

        Solution::Unsigned(count)
    }
}

impl Day14 {
    fn create_grid_with_floor(&self) -> (Grid<Cell>, usize) {
        let floor = self.bottom_right.y + 2;
        let min_x = min(self.top_left.x - 2, 500 - floor);
        let max_x = max(self.bottom_right.x + 2, 500 + floor);
        let col_count = max_x - min_x + 1;
        let row_count = floor + 1;
        let mut grid = Grid::new(col_count, row_count, Cell::Air);
        self.fill_grid(&mut grid, |p| Position::new(p.x - min_x, p.y));

        for x in 0..col_count {
            *grid.at_mut(&Position::new(x, row_count - 1)) = Cell::Rock;
        }

        let start = 500 - min_x;
        (grid, start)
    }

    fn create_grid(&self) -> (Grid<Cell>, usize) {
        let col_count = self.bottom_right.x - self.top_left.x + 1;
        let row_count = self.bottom_right.y - self.top_left.y + 1 + col_count;
        let mut grid = Grid::new(col_count, row_count, Cell::Air);
        self.fill_grid(&mut grid, |p| Position::new(p.x - self.top_left.x, p.y - self.top_left.y + col_count));
        let start = 500 - self.top_left.x;
        (grid, start)
    }

    fn fill_grid<F>(&self, grid: &mut Grid<Cell>, map: F)
    where
        F: Fn(&Position) -> Position
    {
        for path in &self.paths {
            for pair in path.windows(2) {
                if let &[start, end] = pair {
                    if start.x == end.x {
                        // Horizontal
                        for y in min(start.y, end.y)..=max(start.y, end.y) {
                            *grid.at_mut(&map(&Position::new(start.x, y))) = Cell::Rock;
                        }
                    } else if start.y == end.y {
                        // Vertical
                        for x in min(start.x, end.x)..=max(start.x, end.x) {
                            *grid.at_mut(&map(&Position::new(x, start.y))) = Cell::Rock;
                        }
                    } else {
                        panic!("diagonal line")
                    }
                }
            }
        }
    }
}

fn drop_sand(grid: &mut Grid<Cell>, start: usize) -> bool {    
    let mut pos = Position::new(start, 0);
    loop {
        if let Some(dropped) = drop_sand_step(grid, &mut pos) { 
            if dropped {
                *grid.at_mut(&pos) = Cell::Sand;
            }
            return dropped;
        }
    }
}

fn drop_sand_floor(grid: &mut Grid<Cell>, start: usize) -> bool {    
    let start_pos = Position::new(start, 0);
    let mut pos = start_pos;
    loop {
        if let Some(dropped) = drop_sand_step(grid, &mut pos) { 
            if dropped {
                *grid.at_mut(&pos) = Cell::Sand;
            } else {
                panic!("floor should be infinite!");
            }

            return pos != start_pos;
        }
    }
}

fn drop_sand_step(grid: &mut Grid<Cell>, pos: &mut Position) -> Option<bool> {
    let mut next = *pos;
    next.y += 1;
    if next.y == grid.row_count {
        return Some(false);
    }
    if *grid.at(&next) == Cell::Air {
        *pos = next;
        return None;
    }
    if next.x == 0 {
        return Some(false);
    }
    next.x -= 1;
    if *grid.at(&next) == Cell::Air {
        *pos = next;
        return None;
    }
    next.x += 2;
    if next.x >= grid.col_count {
        return Some(false);
    }
    if *grid.at(&next) == Cell::Air {
        *pos = next;
        return None;
    }
    return Some(true);
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_sample() {
        test_day(Day14::from_sample(SAMPLE), 
            Solution::Unsigned(24), 
            Solution::Unsigned(93));
    }

    #[test]
    fn test_input() {
        test_day(Day14::from_lines(load_input(14)), 
            Solution::Unsigned(873), 
            Solution::Unsigned(24813));
    }
}
