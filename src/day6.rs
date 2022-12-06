use std::collections::HashSet;

use itertools::Itertools;

use crate::{DaySolution, FromInput};

pub struct Day6 {
    datastream: String,
}

impl FromInput for Day6 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let datastream = lines.filter(|l| !l.trim().is_empty()).last().expect("invalid datastream");
        Self { datastream }
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> Option<String> {
        for ((_, c0), (_, c1), (_, c2), (i, c3)) in self.datastream.char_indices().into_iter().tuple_windows() {
            if c0 != c1 && c0 != c2 && c0 != c3 && c1 != c2 && c1 != c3 && c2 != c3 {
                return Some((i + 1).to_string());
            }
        }

        None
    }

    fn part_two(&self) -> Option<String> {
        find_first_unique_seq(&self.datastream, 14).map(|i| i.to_string())
    }
}

fn find_first_unique_seq(stream: &str, len: usize) -> Option<usize> {
    let mut index = 0;
    for window in stream.chars().collect_vec().windows(len) {
        if window.into_iter().collect::<HashSet<_>>().len() == len {
            return Some(index + len);
        }
        index += 1;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_sample() {
        let day = Day6::from_sample(SAMPLE);
        day.solve();
    }

    #[test]
    fn test_other_samples() {
        Day6::from_sample("bvwbjplbgvbhsrlpgdmjqwftvncz").solve();
        Day6::from_sample("nppdvjthqldpwncqszvftbrmjlhg").solve();
        Day6::from_sample("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").solve();
        Day6::from_sample("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").solve();
    }
}
