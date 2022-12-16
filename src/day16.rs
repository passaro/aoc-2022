use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::solution::{DaySolution, FromInput, Solution};
use crate::bitset::BitSet;

pub struct Day16 {
    valves: HashMap<String, Valve>,
}

impl FromInput for Day16 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let valves = lines.map(|l| {
            let l = l.strip_prefix("Valve ").unwrap();
            let (name, l) = l.split_once(" has flow rate=").unwrap();
            let (flow_rate, l) = l.split_once("; tunnels lead to valves ")
                .or_else(|| { l.split_once("; tunnel leads to valve ") })
                .unwrap();
            let flow_rate = flow_rate.parse().unwrap();
            let tunnels = l.split(", ").map(|t| t.to_string()).collect();

            let valve = Valve { name: name.to_string(), flow_rate, tunnels };

            (name.to_string(), valve)
        }).collect();
        
        Self { valves }
    }
}

impl DaySolution for Day16 {
    fn part_one(&self) -> Solution {
        let max_pressure = self.find_pressure_paths(30)
            .into_iter().map(|(p, _)| p).max().unwrap();
        Solution::Unsigned(max_pressure)
    }

    fn part_two(&self) -> Solution {
        let paths = self.find_pressure_paths(26);

        let max_paths = paths.into_iter()
            .into_grouping_map_by(|&(_, v)| v)
            .max_by_key(|_, &(p, _)| p)
            .into_iter()
            .map(|(_, x)| x)
            .collect_vec();

        let max_pressure = max_paths.iter().tuple_combinations()
            .filter(|(&m, &e)| m.1.intersection(&e.1) == BitSet::with(0))
            .map(|(m, e)| m.0 + e.0)
            .max()
            .unwrap();

        Solution::Unsigned(max_pressure)
    }
}

impl Day16 {
    fn build_graph(&self) -> Vec<(&Valve, Vec<(usize, usize)>)> {
        let mut graph = self.valves.iter()
            .filter(|(_, v)| v.name == "AA" || v.flow_rate > 0)
            .sorted_by_key(|&(k, _)| k)
            .map(|(_, v)| (v, Vec::new()))
            .collect_vec();

        let count = graph.len();
        for (a, b) in (0..count).tuple_combinations() {
            if a == b { continue; }

            let start = &graph[a].0.name;
            let target = &graph[b].0.name;

            let (_, cost) = dijkstra(start, 
                |name| { 
                    self.valves.get(name).unwrap().tunnels.iter().map(|t| (t.clone(), 1usize)) 
                }, 
                |v| v == target).unwrap();
            
            graph[a].1.push((b, cost));
            graph[b].1.push((a, cost));
        }

        graph
    }

    fn find_pressure_paths(&self, time: usize) -> Vec<(usize, BitSet)> {
        let graph = self.build_graph();
        struct Node {
            index: usize,
            time_left: usize,
            visited: BitSet,
            pressure: usize,
        }
        let mut stack = vec![Node {
            index: 0,
            time_left: time,
            visited: BitSet::with(0),
            pressure: 0,
        }];

        let mut paths = Vec::new();
        while let Some(current) = stack.pop() {
            for &(index, cost) in &graph[current.index].1 {
                if current.visited.is_enabled(index) || cost + 1 > current.time_left {
                    continue;
                }

                let mut visited = current.visited;
                visited.enable(index);
                let time_left = current.time_left - cost - 1;
                stack.push(Node {
                    index,
                    time_left,
                    visited,
                    pressure: current.pressure + time_left * graph[index].0.flow_rate,
                });
            }

            paths.push((current.pressure, current.visited));
        }
        
        paths
    }

}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_sample() {
        test_day(Day16::from_sample(SAMPLE), 
            Solution::Unsigned(1651), 
            Solution::Unsigned(1707));
    }

    #[test]
    fn test_input() {
        test_day(Day16::from_lines(load_input(16)), 
            Solution::Unsigned(2119), 
            Solution::Unsigned(2615));
    }
}
