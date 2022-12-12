use itertools::Itertools;
use pathfinding::dijkstra;

use crate::{DaySolution, FromInput};

#[derive(Debug)]
pub struct Day12 {
    col_count: usize,
    row_count: usize,
    heights: Vec<u8>,
    start_pos: Position,
    target_pos: Position,
}

impl FromInput for Day12 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut heights = Vec::new();
        let mut col_count = 0;
        let mut row_count = 0;
        let mut start_pos = Position::origin();
        let mut target_pos = Position::origin();
        for line in lines.filter(|l| !l.trim().is_empty()) {
            col_count = line.len();
            for (col, c) in line.chars().enumerate() {
                let h = match c {
                    'S' => { start_pos = Position { x: col, y: row_count }; 0 },
                    'E' => { target_pos = Position { x: col, y: row_count }; 25},
                    c => ((c as u32) - 97) as u8,
                };
                heights.push(h);
            }
            row_count += 1;
        }

        Self { col_count, row_count, heights, start_pos, target_pos }
    }
}

impl DaySolution for Day12 {
    fn part_one(&self) -> Option<String> {
        let shortest = self.shortest_climb_from_start();

        shortest.map(|s| s.to_string())
    }

    fn part_two(&self) -> Option<String> {
        let shortest = (0..self.row_count)
            .cartesian_product(0..self.col_count)
            .map(|(y, x)| Position::new(x, y))
            .filter(|p| self.at(p) == 0)
            .filter_map(|p| self.shortest_climb(&p))
            .min();

        shortest.map(|s| s.to_string())
    }
}

impl Day12 {

    fn at(&self, pos: &Position) -> u8 {
        self.heights[pos.y * self.col_count + pos.x]
    }

    fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let &Position { x, y } = pos;
        let mut n = Vec::with_capacity(4);
        if x > 0 {
            n.push(Position::new(x - 1, y));
        }
        if x + 1 < self.col_count {
            n.push(Position::new(x + 1, y));
        }
        if y > 0 {
            n.push(Position::new(x, y - 1));
        }
        if y + 1 < self.row_count {
            n.push(Position::new(x, y + 1));
        }
        n
    }

    fn shortest_climb_from_start(&self) -> Option<usize> {     
        self.shortest_climb(&self.start_pos)
    }

    fn shortest_climb(&self, start: &Position) -> Option<usize> {     
        let result = dijkstra(start, 
            |p| {
                let v = self.at(p);
                self.neighbours(p).into_iter()
                    .filter(|n| self.at(n) <= v+1)
                    .map(|n| (n, 1))
                    .collect_vec()
            },
            |p| *p == self.target_pos);

        result.map(|r| r.1)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self { 
        Self { x, y }
    }

    fn origin() -> Self { 
        Self { x: 0, y: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_sample() {
        let day = Day12::from_sample(SAMPLE);
        day.solve();
    }
}
