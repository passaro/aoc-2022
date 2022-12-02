use crate::{DaySolution, FromInput};

pub struct Day2 {
    matches: Vec<(String, String)>,
}

#[derive(Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn from_string(s: &str) -> Option<Self> {
        match s {
            "X" => Some(Self::Lose),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn shape_against(&self, other: &HandShape) -> HandShape {
        match (self, other) {
            (Outcome::Lose, HandShape::Rock) => HandShape::Scissor,
            (Outcome::Lose, HandShape::Paper) => HandShape::Rock,
            (Outcome::Lose, HandShape::Scissor) => HandShape::Paper,
            (Outcome::Draw, x) => *x,
            (Outcome::Win, HandShape::Rock) => HandShape::Paper,
            (Outcome::Win, HandShape::Paper) => HandShape::Scissor,
            (Outcome::Win, HandShape::Scissor) => HandShape::Rock,
        }
    }
}

impl HandShape {
    fn from_string(s: &str) -> Option<Self> {
        match s {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissor),
            _ => None,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn outcome_against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Rock, Self::Scissor) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissor) => Outcome::Lose,
            (Self::Scissor, Self::Rock) => Outcome::Lose,
            (Self::Scissor, Self::Paper) => Outcome::Win,
            (Self::Scissor, Self::Scissor) => Outcome::Draw,
        }
    }

    fn score_against(&self, other: &Self) -> u32 {
        self.outcome_against(other).score() + self.value()
    }
}

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let matches = lines
            .filter_map(|l| { 
                l.trim()
                 .split_once(" ")
                 .map(|(a, b)| 
                 (a.to_owned(), b.to_owned()))
            })
            .collect();
        Self { matches }
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> Option<String> {
        let score: u32 = self.matches
            .iter()
            .map(|(a, b)| 
                 (HandShape::from_string(a).expect("Invalid shape"), 
                 HandShape::from_string(b).expect("Invalid shape")))
            .map(|(opponent, you)| you.score_against(&opponent))
            .sum();
        Some(score.to_string())
    }

    fn part_two(&self) -> Option<String> {
        let score: u32 = self.matches
            .iter()
            .map(|(a, b)| 
                 (HandShape::from_string(a).expect("Invalid shape"), 
                 Outcome::from_string(b).expect("Invalid outcome")))
            .map(|(opponent, outcome)| outcome.score() + outcome.shape_against(&opponent).value())
            .sum();
        Some(score.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"A Y
B X
C Z    
";

    #[test]
    fn test_sample() {
        let day = Day2::from_sample(SAMPLE);
        day.solve();
    }
}