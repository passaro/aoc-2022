use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day20 {
    numbers: Vec<isize>,
}

impl FromInput for Day20 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let numbers = lines
            .map(|l| l.parse().expect("invalid number"))
            .collect();

        Self { numbers }
    }
}

struct Circular {
    buffer: Vec<(isize, usize)>,
}

impl Circular {
    fn new(values: &[isize], decryption_key: isize) -> Self {
        let buffer = values.iter().enumerate().map(|(p, v)| (*v * decryption_key, p)).collect();
        Self { buffer }
    }

    fn mix(&mut self) {
        for i in 0..self.buffer.len() {
            let (current, &(value, original)) = self.buffer.iter().enumerate().find(|&(_, &(_, o))| o == i).unwrap();
            self.buffer.remove(current);
            let dest = (current as isize + value).rem_euclid(self.buffer.len() as isize) as usize;
            self.buffer.insert(dest, (value, original));
        }
    }

    fn result(&self) -> isize {
        let (zero, _) = self.buffer.iter()
            .enumerate()
            .find(|(_, (v, _))| *v == 0)
            .expect("no zero");

        self.buffer[(1000 + zero as isize).rem_euclid(self.buffer.len() as isize) as usize].0 
            + self.buffer[(2000 + zero as isize).rem_euclid(self.buffer.len() as isize) as usize].0
            + self.buffer[(3000 + zero as isize).rem_euclid(self.buffer.len() as isize) as usize].0
    }
}

impl DaySolution for Day20 {
    fn part_one(&self) -> Solution {
        let mut circular = Circular::new(&self.numbers, 1);
        circular.mix();

        let result = circular.result();

        Solution::Signed(result)
    }

    fn part_two(&self) -> Solution {
        let mut circular = Circular::new(&self.numbers, 811589153);
        
        for _ in 0..10 {
            circular.mix();
        }

        let result = circular.result();

        Solution::Signed(result)
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"1
2
-3
3
-2
0
4";

    #[test]
    fn test_sample() {
        test_day(Day20::from_sample(SAMPLE), 
            Solution::Signed(3), 
            Solution::Signed(1623178306));
    }

    #[test]
    fn test_input() {
        test_day(Day20::from_lines(load_input(20)), 
            Solution::Signed(13883), 
            Solution::Signed(19185967576920));
    }
}
