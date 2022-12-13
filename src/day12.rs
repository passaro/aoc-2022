use itertools::Itertools;
use pathfinding::dijkstra;
use rayon::prelude::*;

use crate::solution::{DaySolution, FromInput, Solution};
use crate::grid::{Grid, Position};

#[derive(Debug)]
pub struct Day12 {
    heights: Grid<u8>,
    start_pos: Position,
    target_pos: Position,
}

impl FromInput for Day12 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut start_pos = Position::origin();
        let mut target_pos = Position::origin();
        let heights = Grid::from_lines(lines, |c, pos| {
            match c {
                'S' => { start_pos = *pos; 0 },
                'E' => { target_pos = *pos; 25},
                c => ((c as u32) - 97) as u8,
            }
        });
        
        Self { heights, start_pos, target_pos }
    }
}

impl DaySolution for Day12 {
    fn part_one(&self) -> Solution {
        let shortest = self.shortest_climb_from_start();

        Solution::Unsigned(shortest.expect("no path"))
    }

    fn part_two(&self) -> Solution {
        let shortest = self.heights.positions()
            .filter(|p| *self.heights.at(p) == 0)
            .collect_vec()
            .par_iter()
            .filter_map(|p| self.shortest_climb(&p))
            .min();

        Solution::Unsigned(shortest.expect("no path"))
    }
}

impl Day12 {
 
    fn shortest_climb_from_start(&self) -> Option<usize> {     
        self.shortest_climb(&self.start_pos)
    }

    fn shortest_climb(&self, start: &Position) -> Option<usize> {     
        let result = dijkstra(start, 
            |p| {
                let v = *self.heights.at(p);
                self.heights.neighbours(p).into_iter()
                    .filter(|n| *self.heights.at(n) <= v+1)
                    .map(|n| (n, 1))
                    .collect_vec()
            },
            |p| *p == self.target_pos);

        result.map(|r| r.1)
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
