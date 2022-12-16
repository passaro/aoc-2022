use std::collections::HashSet;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day15 {
    sensors: Vec<Sensor>,
    row: isize,
}

impl FromInput for Day15 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        let sensors = lines
            .map(|l| {
                let l = l.strip_prefix("Sensor at x=").unwrap();
                let (x, l) = l.split_once(", y=").unwrap();
                let x = x.parse().unwrap();
                let (y, l) = l.split_once(": closest beacon is at x=").unwrap();
                let y = y.parse().unwrap();
                let (bx, by) = l.split_once(", y=").unwrap();
                let bx = bx.parse().unwrap();
                let by = by.parse().unwrap();
                Sensor { pos: (x, y), beacon_pos: (bx, by) }
            })
            .collect();

        Self { sensors, row: 2_000_000 }
    }
}

impl Day15 {
    #[cfg(test)]
    fn for_row(mut self, row: isize) -> Self {
        self.row = row;
        self
    }
}

#[derive(Debug)]
struct Sensor {
    pos: (isize, isize),
    beacon_pos: (isize, isize),
}

impl Sensor {
    fn distance(&self) -> usize {
        distance(self.pos, self.beacon_pos)
    }
}

fn distance(pos_a: (isize, isize), pos_b: (isize, isize)) -> usize {
    pos_a.0.abs_diff(pos_b.0) + pos_a.1.abs_diff(pos_b.1)
}

impl DaySolution for Day15 {
    fn part_one(&self) -> Solution {
        let empty = self.empty_at_row(self.row);

        let beacons = self.sensors.iter()
            .filter(|s| s.beacon_pos.1 == self.row)
            .map(|s| s.beacon_pos.0)
            .collect::<HashSet<isize>>();
         
        let count = empty.into_iter().map(|(s, e)| e - s + 1).sum::<isize>() as usize - beacons.len();

        Solution::Unsigned(count)
    }

    fn part_two(&self) -> Solution {
        let limit = self.row*2;
        for r in 0..=limit {
            let empty = self.empty_at_row(r);
            
            let mut beacon = 0;
            for (s, e) in empty {
                if s > beacon { break; }
                if e >= beacon {
                    beacon = e + 1;
                }
                if beacon > limit { break; }
            }

            if beacon <= limit {
                return Solution::Signed(4_000_000 * beacon + r);
            }
        }
        
        Solution::NotImplemented
    }
}

impl Day15 {
    fn empty_at_row(&self, row: isize) -> Vec<(isize, isize)> {
        let mut empty = Vec::new();
        for s in &self.sensors {
            let range = s.distance();
            let y_dist = s.pos.1.abs_diff(row);
            if y_dist <= range {
                let x_range = range - y_dist;
                empty.push((s.pos.0 - x_range as isize, s.pos.0 + x_range as isize));
            }
        }
        empty.sort_by_key(|(s, _)| *s);
        let mut reduced = Vec::new();
        for (start, end) in empty {
            if let Some((_, pe)) = reduced.last_mut() {
                if end <= *pe {
                    continue;
                }

                if start <= *pe + 1 {
                    *pe = end;
                    continue;
                }
            }
            reduced.push((start, end));
        }
        reduced
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_sample() {
        test_day(Day15::from_sample(SAMPLE).for_row(10), 
            Solution::Unsigned(26), 
            Solution::Signed(56000011));
    }

    #[test]
    fn test_input() {
        test_day(Day15::from_lines(load_input(15)).for_row(2_000_000), 
            Solution::Unsigned(5870800), 
            Solution::Signed(10908230916597));
    }
}
