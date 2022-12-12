use std::{str::FromStr, collections::HashSet};

use crate::{DaySolution, FromInput};

pub struct Day9 {
    moves: Vec<Move>,
}

impl FromInput for Day9 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let moves = lines.map(|l| l.parse().expect("invalid move")).collect();
        Self { moves }
    }
}

impl DaySolution for Day9 {
    fn part_one(&self) -> Option<String> {
        let mut head = Position::new();
        let mut tail = Position::new();

        let mut visited = HashSet::new();
        for m in &self.moves {
            for _ in 0..m.steps {
                m.step(&mut head);
                tail.follow(&head);
                visited.insert(tail);
            }
        }

        Some(visited.len().to_string())
    }

    fn part_two(&self) -> Option<String> {
        let mut rope = [Position::new(); 10];

        let mut visited = HashSet::new();
        for m in &self.moves {
            for _ in 0..m.steps {
                m.step(&mut rope[0]);
                let mut previous = rope[0];
                for knot in &mut rope[1..] {
                    knot.follow(&previous);
                    previous = *knot;
                }
                
                visited.insert(previous);
            }
        }

        Some(visited.len().to_string())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn follow(&mut self, head: &Position) {
        let dx = (head.x - self.x).abs();
        let dy = (head.y - self.y).abs();

        if dx > 1 || dy > 1 {
            self.x += (head.x - self.x).signum();
            self.y += (head.y - self.y).signum();
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: char,
    steps: usize,
}

impl Move {
    fn step(&self, pos: &mut Position) {
        match self.direction {
            'R' => pos.x += 1,
            'L' => pos.x -= 1,
            'D' => pos.y -= 1,
            'U' => pos.y += 1,
            _ => {},
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dir, steps)) = s.split_once(" ") { 
            if let Some(direction) = dir.chars().next() {
                if "RLUD".contains(direction) {
                    if let Ok(steps) = steps.parse() {
                        return Ok(Move { direction, steps });
                    }
                }
            }
        }
        
        Err("invalid move")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_sample() {
        let day = Day9::from_sample(SAMPLE);
        day.solve();
    }

    #[test]
    fn test_sample2() {
        Day9::from_sample(r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")
        .solve();
    }
}
