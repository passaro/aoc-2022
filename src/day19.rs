use std::{str::FromStr, iter::once, cmp::max, collections::HashSet};

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

impl FromInput for Day19 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let blueprints = lines.map(|l| {
            l.parse().expect("invalid blueprint")
        }).collect();

        Self { blueprints }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robots: ResMap<ResMap<usize>>,
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (h, s) = s.split_once(": ").ok_or("no :")?;
        let h = h.strip_prefix("Blueprint ").ok_or("no blueprint")?;
        let id = h.parse().map_err(|_| "no id")?;
        let mut robots = ResMap::new();
        for r in s.split_terminator(".") {
            fn parse_robot(r: &str) -> Option<(Resource, ResMap<usize>)> {
                let mut seq = r.trim().split_whitespace();
                seq.next()?; // Each
                let robot = Resource::from_string(seq.next()?)?;
                seq.next()?; // robots
                let mut resources = ResMap::new();
                while let Some(_) = seq.next() {
                    let amount = seq.next()?.parse().ok()?;
                    let resource = Resource::from_string(seq.next()?)?;
                    resources.add(resource, amount);
                }
                Some((robot, resources))
            }
            let (r, res) = parse_robot(r).ok_or("error")?;
            robots.set(r, res);
        }
            
        Ok(Blueprint { id, robots })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    robots: ResMap<usize>,
    resources: ResMap<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn from_string(str: &str) -> Option<Resource> {
        match str {
            "ore" => Some(Self::Ore),
            "clay" => Some(Self::Clay),
            "obsidian" => Some(Self::Obsidian),
            "geode" => Some(Self::Geode),
            _ => None,
        }
    }

    fn values() -> [Resource; 4] {
        [
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct ResMap<T> {
    amounts: [T; 4],
}

impl <T> ResMap<T> 
where T: Default + Copy 
{
    fn new() -> Self {
        Self { amounts: [T::default(); 4] }
    }

    fn get(&self, key: Resource) -> T {
        self.amounts[Self::index(key)]
    }

    fn set(&mut self, key: Resource, value: T) {
        self.amounts[Self::index(key)] = value;
    }

    fn index(key: Resource) -> usize {
        key as usize
    }
}

impl <> ResMap<usize> {

    fn add(&mut self, key: Resource, amount: usize) {
        self.amounts[Self::index(key)] += amount;
    }

    fn contains(&self, other: &Self) -> bool {
        (0..4).all(|i| self.amounts[i] >= other.amounts[i])
    }

    fn remove_all(&mut self, other: &Self) {
        for i in 0..4 {
            self.amounts[i] -= other.amounts[i];    
        }
    }

    fn add_all(&mut self, other: &Self) {
        for i in 0..4 {
            self.amounts[i] += other.amounts[i];    
        }
    }
}

impl State {
    fn successors<'a>(&'a self, blueprint: &'a Blueprint) -> impl DoubleEndedIterator<Item=State> + 'a {
        once(None).chain(
            Resource::values().into_iter()
                .map(|r| (r, blueprint.robots.get(r)))
                .filter(|(_, required_resources)| self.resources.contains(required_resources))
                .map(|r| Some(r))
        ).map(|r| self.step(r))
    }

    fn collect_resources(&mut self) {
        self.resources.add_all(&self.robots);
    }

    fn step(&self, robot: Option<(Resource, ResMap<usize>)>) -> State {
        let mut new_state = self.clone();
        new_state.time += 1;
        new_state.collect_resources();
        if let Some((robot, required_resources)) = robot {
            new_state.resources.remove_all(&required_resources);
            new_state.robots.add(robot, 1);
        }

        new_state
    }
}

fn evolve(blueprint: &Blueprint, minutes: usize) -> usize {
    let ore_robot = { let mut r = ResMap::new(); r.add(Resource::Ore, 1); r };
    let start = State { time: 0, resources: ResMap::new(), robots: ore_robot };

    let mut max_geodes = 0;
    let mut states = vec![start];
    let mut visited = HashSet::new();

    while let Some(s) = states.pop() {
        if !visited.insert(s) { continue; }
        let geodes = s.resources.get(Resource::Geode);
        max_geodes = max(max_geodes, geodes);
        if s.time == minutes { continue; }

        for n in s.successors(blueprint).rev().take(2) {
            if s.resources.get(Resource::Geode) 
                + (minutes - s.time) * (s.robots.get(Resource::Geode) 
                + (minutes - s.time + 1) / 2) > max_geodes {
                states.push(n);
            }
        }
    }
    max_geodes
}

impl DaySolution for Day19 {
    fn part_one(&self) -> Solution {
        let sum = self.blueprints.iter()
            .map(|blueprint| {
                blueprint.id * evolve(blueprint, 24)
            })
            .sum();
        Solution::Unsigned(sum)
    }

    fn part_two(&self) -> Solution {
        let product = self.blueprints.iter().take(3)
            .map(|blueprint| {
                evolve(blueprint, 32)
            })
            .product();
        Solution::Unsigned(product)
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_sample() {
        test_day(Day19::from_sample(SAMPLE), 
            Solution::Unsigned(33), 
            Solution::Unsigned(56 * 62));
    }

    #[test]
    fn test_input() {
        test_day(Day19::from_lines(load_input(19)), 
            Solution::Unsigned(2193), 
            Solution::Unsigned(7200));
    }
}
