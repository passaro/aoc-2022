use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day2;

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        todo!("Parse your input from the input file")
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> Option<String> {
        None
    }

    fn part_two(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"
    ";

    #[test]
    fn test_sample() {
        let day = Day2::from_sample(SAMPLE);
        day.solve();
    }
}