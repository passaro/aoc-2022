use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};


pub struct Day5 {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_string(s: &str) -> Move {
        let mut words = s.split_whitespace();
        assert_eq!(words.next(), Some("move"));
        let count = words.next().unwrap().parse().expect("missing count");
        assert_eq!(words.next(), Some("from"));
        let from = words.next().unwrap().parse().expect("missing from");
        assert_eq!(words.next(), Some("to"));
        let to = words.next().unwrap().parse().expect("missing to");
        Move { count, from, to }
    }
}

impl FromInput for Day5 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let lines = lines.collect_vec();
        let mut blocks = lines.split(|l| l.is_empty());

        let stack_lines = blocks.next().expect("missing stacks");
        let stack_count = stack_lines.last().expect("no stacks")
            .chars().filter(|c| !c.is_whitespace()).count();
        let mut stacks = vec![Vec::new(); stack_count];
        for line in stack_lines.into_iter().rev().skip(1) {
            for (i, c) in line.chars()
                .skip(1).step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace()) {
                stacks[i].push(c);
            }
        }

        let move_lines = blocks.next().expect("missing moves");
        let moves = move_lines.into_iter().map(|l| Move::from_string(l)).collect();
        Self { stacks, moves }
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> Solution {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            apply_move(&mut stacks, m);
        }

        let top = get_tops(&stacks);
        Solution::String(top)
    }

    fn part_two(&self) -> Solution {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            apply_move_9001(&mut stacks, m);
        }

        let top = get_tops(&stacks);
        Solution::String(top)
    }
}

fn get_tops(stacks: &[Vec<char>]) -> String {
    stacks.iter().filter_map(|s| s.last()).join("")
}

fn apply_move(stacks: &mut [Vec<char>], m: &Move) {
    for _ in 0..m.count {
        let moved = stacks[m.from - 1].pop().expect("missing crate");
        stacks[m.to - 1].push(moved);
    }
}

fn apply_move_9001(stacks: &mut [Vec<char>], m: &Move) {
    let mut moved = Vec::with_capacity(m.count);
    for _ in 0..m.count {
        moved.insert(0, stacks[m.from - 1].pop().expect("missing crate"));
    }
    stacks[m.to - 1].append(&mut moved);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_sample() {
        let day = Day5::from_sample(SAMPLE);
        day.solve();
    }
}
