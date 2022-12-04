use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::{DaySolution, FromInput};

pub struct Day4 {
    range_pairs: Vec<(RangeInclusive<u32>,RangeInclusive<u32>)>
}

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        fn parse_range(s: &str) -> RangeInclusive<u32> {
            let (start, end) = s.split_once("-").expect("invalid range");
            let start = start.parse::<u32>().expect("invalid range start");
            let end = end.parse::<u32>().expect("invalid range end");
            RangeInclusive::new(start, end)
        }

        let range_pairs = lines
            .filter_map(|l| l.split_once(",").map(|(a,b)| (parse_range(a), parse_range(b))))
            .collect_vec();

        Self { range_pairs }
    }
}

impl DaySolution for Day4 {
    fn part_one(&self) -> Option<String> {
        fn fully_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
            a.start() <= b.start() && a.end() >= b.end()
        }

        let count = self.range_pairs.iter()
            .filter(|p| fully_contains(&p.0, &p.1) || fully_contains(&p.1, &p.0))
            .count();

        Some(count.to_string())
    }

    fn part_two(&self) -> Option<String> {
        fn overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
            a.start() <= b.end() && a.end() >= b.start()
        }

        let count = self.range_pairs.iter()
            .filter(|p| overlap(&p.0, &p.1))
            .count();

        Some(count.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_sample() {
        let day = Day4::from_sample(SAMPLE);
        day.solve();
    }
}
