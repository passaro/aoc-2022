use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day21 {
    monkeys: Vec<(String, Job)>,
}

impl FromInput for Day21 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let monkeys = lines.map(|l| {
            let (name, l) = l.split_once(": ").unwrap();
            
            let job = if let Ok(n) = l.parse() {
                Job::Yell(n)
            } else {
                let parts = l.split_whitespace().collect_vec();
                Job::Calc(Operation { 
                    lhs: parts[0].to_string(), 
                    op: parts[1].to_string(), 
                    rhs: parts[2].to_string()})
            };

            (name.to_string(), job)
        }).collect();
        Self { monkeys }
    }
}

impl DaySolution for Day21 {
    fn part_one(&self) -> Solution {
        let root = yell("root", &self.monkeys);
        Solution::Signed(root)
    }

    fn part_two(&self) -> Solution {
        let mut monkeys = self.monkeys.clone();
        let mut new_monkeys = Vec::new();
        let (i_humn, humn) = monkeys.iter().find_position(|(m, _)| m == "humn").unwrap();
        let mut to_find = humn.0.clone();
        monkeys.remove(i_humn);
        loop {
            let mut dependants = monkeys.iter()
                .enumerate()
                .filter_map(|(i, (m, j))| if let Job::Calc(op) = j { Some((i, (m, op))) } else { None })
                .filter(|(_, (_, op))| op.lhs == to_find || op.rhs == to_find);
            let (index, (monkey, operation)) = dependants.next().unwrap();
            assert!(dependants.next().is_none());
            if monkey == "root" {
                let other = if operation.lhs == to_find { &operation.rhs } else { &operation.lhs };
                let special = Operation { lhs: other.clone(), op: "=".to_string(), rhs: other.clone() };
                new_monkeys.push((to_find, Job::Calc(special)));
                monkeys.remove(index);
                break;
            } else {
                let inverted = invert(monkey, operation, &to_find);
                new_monkeys.push((to_find, Job::Calc(inverted)));
                to_find = monkey.clone();
                monkeys.remove(index);
            }
        }
        new_monkeys.append(&mut monkeys);
        let result = yell("humn", &new_monkeys);
        Solution::Signed(result)
    }
}

fn yell(root: &str, monkeys: &[(String, Job)]) -> isize {
    let mut values = HashMap::new();
    loop {
        let start = values.len();
        for (monkey, job) in monkeys {
            if !values.contains_key(monkey.as_str()) {
                if let Some(n) = match job {
                        Job::Yell(n) => Some(*n),
                        Job::Calc(operation) => calculate(operation, &values),
                    }
                {
                    values.insert(monkey.as_str(), n);
                }
            }
        }

        if let Some(result) = values.get(root) {
            return *result;
        }

        if values.len() == start {
            panic!("no new values!")
        }

    }
}

#[derive(Debug, Clone)]
enum Job {
    Yell(isize),
    Calc(Operation),
}

#[derive(Debug, Clone)]
struct Operation {
    lhs: String, 
    op: String, 
    rhs: String,
}

fn calculate(operation: &Operation, values: &HashMap<&str, isize>) -> Option<isize> {
    let lhs = values.get(operation.lhs.as_str());
    let rhs = values.get(operation.rhs.as_str());
    if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
        let result = match operation.op.as_str() {
            "+" => lhs + rhs,
            "*" => lhs * rhs,
            "-" => lhs - rhs,
            "/" => lhs / rhs,
            "=" => *lhs,
            _ => panic!("unsupported op")
        };

        return Some(result)
    }

    None
}

fn invert(m: &str, operation: &Operation, x: &str) -> Operation {
    // m = l # r 
    let is_left = operation.lhs == x;
    let m = m;
    let l = operation.lhs.as_str();
    let r = operation.rhs.as_str();
    let (l, op, r) = match (is_left, operation.op.as_str()) {
        (true, "+") => (m, "-", r),
        (false, "+") => (m, "-", l),
        (true, "-") => (m, "+", r),
        (false, "-") => (l, "-", m),
        (true, "*") => (m, "/", r),
        (false, "*") => (m, "/", l),
        (true, "/") => (m, "*", r),
        (false, "/") => (l, "/", m),
        _ => panic!("unsupported op")
    };
    Operation { 
        lhs: l.to_string(), 
        op: op.to_string(), 
        rhs: r.to_string() 
    }
}

#[cfg(test)]
mod test {
    use crate::solution::{test::test_day, load_input};

    use super::*;

    const SAMPLE: &str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_sample() {
        test_day(Day21::from_sample(SAMPLE), 
            Solution::Signed(152), 
            Solution::Signed(301));
    }

    #[test]
    fn test_input() {
        test_day(Day21::from_lines(load_input(21)), 
            Solution::Signed(24947355373338), 
            Solution::Signed(3876907167495));
    }
}
