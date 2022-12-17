use std::cmp::max;

use itertools::Itertools;

use crate::{solution::{DaySolution, FromInput, Solution}, grid::{Grid, Position}};

pub struct Day17 {
    pattern: Vec<Direction>,
}

impl FromInput for Day17 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let pattern = lines.into_iter().next().unwrap()
            .chars()
            .map(|c| {
                match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("invalid jet"),
                }
            })
            .collect();

        Self { pattern }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn apply(&self, position: &Position, width: usize) -> Option<Position> {
        match self {
            Direction::Left => if position.x == 0 { None } else { Some(Position::new(position.x - 1, position.y)) }
            Direction::Right => if position.x + 1 == width { None } else { Some(Position::new(position.x + 1, position.y)) },
            Direction::Down => if position.y == 0 { None } else { Some(Position::new(position.x, position.y - 1)) },
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    pieces: Vec<(usize, usize)>,
}

impl Rock {
    fn sequence() -> Vec<Rock> {
        vec![
            Rock { pieces: vec![(0,0), (1,0), (2,0), (3,0)] },
            Rock { pieces: vec![(1,0), (0,1), (1,1), (2,1), (1,2)] },
            Rock { pieces: vec![(0,0), (1,0), (2,0), (2,1), (2,2)] },
            Rock { pieces: vec![(0,0), (0,1), (0,2), (0,3)] },
            Rock { pieces: vec![(0,0), (1,0), (0,1), (1,1)] },
        ]
    }

    fn positions(&self, bottom_left: Position) -> impl Iterator<Item = Position> + '_ {
        self.pieces.iter().map(move |(x, y)| {
            Position::new(bottom_left.x + x, bottom_left.y + y)
        })
    }

    fn fits(&self, grid: &Grid<bool>, pos: &Position) -> bool {
        self.positions(*pos).all(|p| {
            grid.is_valid(&p) && !*grid.at(&p)
        })
    }
}

impl DaySolution for Day17 {
    fn part_one(&self) -> Solution {
        let rock_count = 2022;
        let height = self.simulate(rock_count).last().unwrap();
        Solution::Unsigned(height)
    }

    fn part_two(&self) -> Solution {
        let test_count = 10_000;
        let deltas = self.simulate(test_count).into_iter().scan(0usize, |state, h| {
            let d = h - *state;
            *state = h;
            Some(d)
        }).collect_vec();

        let start = 1_000;
        let slice = &deltas[start..];
        let cycle = (1..(slice.len()/2)).into_iter().find(|&len| {
            slice[..len].into_iter().cycle()
                .zip(&slice[len..])
                .all(|(&a, &b)| a == b)
        }).expect("cycle not found");

        let rock_count = 1_000_000_000_000usize;
        let rep_count = (rock_count - start) / cycle;
        let offset = (rock_count - start) % cycle;

        let height = 
            &deltas[0..start].iter().sum::<usize>()
            + rep_count * &deltas[start..start+cycle].iter().sum::<usize>()
            + &deltas[start..start+offset].iter().sum::<usize>();

        Solution::Unsigned(height)
    }
}

impl Day17 {
    fn simulate(&self, rock_count: usize) -> impl Iterator<Item = usize> + '_ {
        let rocks = Rock::sequence().into_iter().cycle().take(rock_count);
        let mut room = Grid::new(7, rock_count * 4, false);
        let mut jets = self.pattern.iter().cycle();
        let mut height = 0usize;
        rocks.map(move |rock| {
            let mut position = Position::new(2, height + 3);
            loop {
                let jet = jets.next().unwrap();
                if let Some(next) = jet.apply(&position, room.col_count) {
                    if rock.fits(&room, &next) {
                        position = next;
                    }
                }
                if let Some(next) = Direction::Down.apply(&position, room.col_count) {
                    if rock.fits(&room, &next) {
                        position = next;
                        continue;
                    }
                }
                break;
            }
            for p in rock.positions(position) {
                *room.at_mut(&p) = true;
                height = max(height, p.y + 1);
            }

            height
        })
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_sample() {
        test_day(Day17::from_sample(SAMPLE), 
            Solution::Unsigned(3068), 
            Solution::Unsigned(1514285714288));
    }

    #[test]
    fn test_input() {
        test_day(Day17::from_lines(load_input(17)), 
            Solution::Unsigned(3175), 
            Solution::Unsigned(1555113636385));
    }
}
