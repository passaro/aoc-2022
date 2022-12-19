use std::collections::HashSet;

use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day18 {
    cubes: Vec<(isize, isize, isize)>,
}

impl FromInput for Day18 {

    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let cubes = lines.map(|l| {
            l.splitn(3, ',')
                .map(|n| n.parse::<isize>().expect("invalid coordinate") )
                .collect_tuple()
                .unwrap()
        })
        .collect();
        Self { cubes }
    }
}

fn neighbours((x, y, z): (isize, isize, isize)) -> impl Iterator<Item=(isize, isize, isize)> {
    [(x+1, y, z), (x-1, y, z), (x, y+1, z), (x, y-1, z), (x, y, z+1), (x, y, z-1)].into_iter()
}

fn diagonal_neighbours((x, y, z): (isize, isize, isize)) -> impl Iterator<Item=(isize, isize, isize)> {
    [(x+1, y+1, z), (x+1, y-1, z), (x-1, y+1, z), (x-1, y-1, z), (x+1, y, z+1), (x+1, y, z-1),
     (x-1, y, z+1), (x-1, y, z-1), (x, y+1, z+1), (x, y+1, z-1), (x, y-1, z+1), (x, y+1, z-1),
     (x+1, y+1, z+1), (x+1, y+1, z-1), (x+1, y-1, z+1), (x+1, y-1, z-1), 
     (x-1, y+1, z+1), (x-1, y+1, z-1), (x-1, y-1, z+1), (x-1, y-1, z-1)].into_iter()
}

impl DaySolution for Day18 {
    fn part_one(&self) -> Solution {
        let set = self.cubes.iter().collect::<HashSet<&(isize, isize, isize)>>();

        let free_sides = self.cubes.iter()
            .flat_map(|c| neighbours(*c).filter(|n| !set.contains(n)))
            .count();

        Solution::Unsigned(free_sides)
    }

    fn part_two(&self) -> Solution {
        let set = self.cubes.iter().collect::<HashSet<&(isize, isize, isize)>>();

        let mut free = HashSet::new();
        for c in &self.cubes {
            for n in neighbours(*c).chain(diagonal_neighbours(*c)).filter(|n| !set.contains(n)) {
                free.insert(n);
            }
        }

        let mut visited = HashSet::new();
        let mut components : Vec<HashSet<(isize, isize, isize)>> = Vec::new();
        for &f in free.iter() {
            if !visited.insert(f) {
                continue;
            }
            let mut component = HashSet::new();
            component.insert(f);
            let mut stack = vec![f];
            while let Some(c) = stack.pop() {
                for n in neighbours(c).filter(|n| free.contains(n)) {
                    if component.insert(n) && visited.insert(n) {
                        stack.push(n);
                    }
                }
            }
            components.push(component);
        }

        let outside = components.iter().max_by_key(|c| c.len()).unwrap();
        let surface = outside.iter()
            .flat_map(|c| neighbours(*c).filter(|n| set.contains(n)))
            .count();

        Solution::Unsigned(surface)
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_sample() {
        test_day(Day18::from_sample(SAMPLE), 
            Solution::Unsigned(64), 
            Solution::Unsigned(58));
    }

    #[test]
    fn test_input() {
        test_day(Day18::from_lines(load_input(18)), 
            Solution::Unsigned(4474), 
            Solution::Unsigned(2518));
    }
}
