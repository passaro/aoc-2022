use std::{str::FromStr, cmp::Ordering};

use itertools::{Itertools, EitherOrBoth};

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day13 {
    pairs: Vec<(Value, Value)>,
}

impl FromInput for Day13 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let pairs = lines.chunks(3).into_iter()
            .map(|mut chunk| {
                let (a,b) = chunk.next_tuple().expect("error");
                let a = a.parse().expect("error in first");
                let b = b.parse().expect("error in second");
                (a, b)
            })
            .collect();
        Self { pairs }
    }
}

impl DaySolution for Day13 {
    fn part_one(&self) -> Solution {
        let sum: usize = self.pairs.iter()
            .enumerate()
            .filter(|(_, (a,b))| { a <= b })
            .map(|(i, _)| i + 1)
            .sum();

        Solution::Unsigned(sum)
    }

    fn part_two(&self) -> Solution {
        let mut packets = Vec::with_capacity(self.pairs.len() * 2 + 2);
        for (a,b) in &self.pairs  {
            packets.push(a);
            packets.push(b);
        }
        let divider2 = Value::List(vec![Value::Int(2)]);
        let divider6 = Value::List(vec![Value::Int(6)]);
        packets.push(&divider2);
        packets.push(&divider6);
        packets.sort();

        let decoder_key: usize = packets.into_iter().enumerate()
            .filter(|(_, v)| **v == divider2 || **v == divider6)
            .map(|(i, _)| i + 1)
            .product();

        Solution::Unsigned(decoder_key)
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum Value {
    Int(u8),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Int(a), b) => Self::List(vec![Self::Int(*a)]).partial_cmp(b),
            (a, Self::Int(b)) => a.partial_cmp(&Self::List(vec![Self::Int(*b)])),
            (Self::List(a), Self::List(b)) => {
                let mut ordering = Some(Ordering::Equal);
                for pair in a.iter().zip_longest(b) {
                    ordering = match pair {
                        EitherOrBoth::Both(a, b) => a.partial_cmp(b),
                        EitherOrBoth::Left(_) => Some(Ordering::Greater),
                        EitherOrBoth::Right(_) => Some(Ordering::Less),
                    };
                    if let Some(Ordering::Equal) = ordering { } else { break; }
                }
                ordering
            }
        }
    }
}

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        let mut current = Vec::new();
        for mut chunk in s.split(",") {
            while let Some(strip) = chunk.strip_prefix("[") {
                stack.push(current);
                current = Vec::new();
                chunk = strip;
            }

            let mut closing = 0;
            while let Some(strip) = chunk.strip_suffix("]") {
                closing += 1;
                chunk = strip;
            }

            if !chunk.is_empty() {
                let value = chunk.parse().map_err(|_| "invalid value")?;
                current.push(Value::Int(value));
            }

            for _ in 0..closing {
                let value = Value::List(current);
                current = stack.pop().ok_or("extra ]")?;
                current.push(value);
            }
        }
        Ok(Value::List(current))
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_sample() {
        test_day(Day13::from_sample(SAMPLE), 
            Solution::Unsigned(13), 
            Solution::Unsigned(140));
    }

    #[test]
    fn test_input() {
        test_day(Day13::from_lines(load_input(13)), 
            Solution::Unsigned(5208), 
            Solution::Unsigned(25792));
    }
}
