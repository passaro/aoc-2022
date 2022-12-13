use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::{DaySolution, FromInput, Solution};

pub struct Day7 {
    output: Vec<String>,
}

impl FromInput for Day7 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let output = lines.collect_vec();
        Self { output }
    }
}

impl DaySolution for Day7 {
    fn part_one(&self) -> Solution {
        let file_sizes = extract_file_sizes(&self.output);    
        let dir_sizes = calculate_dir_sizes(file_sizes);

        let sum: usize = dir_sizes.values()
            .filter(|&&size| size <= 100000)
            .sum();

        Solution::Unsigned(sum)
    }

    fn part_two(&self) -> Solution {
        let file_sizes = extract_file_sizes(&self.output);    
        let dir_sizes = calculate_dir_sizes(file_sizes);

        let total_space: usize = 70000000;
        let required_free_space: usize = 30000000;

        let used_space = *dir_sizes.get("/").expect("no root");

        if total_space < used_space || required_free_space <= (total_space - used_space) {
            panic!("No solution found");
        }

        let needed_space =  required_free_space - (total_space - used_space);
        let to_delete = dir_sizes.values()
            .filter(|s| **s >= needed_space)
            .min()
            .expect("no directory to delete");
            
        Solution::Unsigned(*to_delete)
    }
}

fn calculate_dir_sizes(file_sizes: HashMap<Vec<String>, usize>) -> HashMap<String, usize> {
    let mut dir_sizes = HashMap::new();
    for (path, size) in file_sizes {
        let parents = path.iter()
            .dropping_back(1)
            .scan(String::new(), |state, d| {
            if state.len() > 1 {
                *state += "/";
            }
            *state += d;
            Some(state.clone())
        });
        for dir_path in parents {
            dir_sizes.entry(dir_path)
                .and_modify(|s| *s += size)
                .or_insert(size);
        }
    }
    dir_sizes
}

fn extract_file_sizes(script: &[String]) -> HashMap<Vec<String>, usize> {
    let mut file_sizes = HashMap::new();

    let mut current = Vec::new();
    for line in script {
        if line == "$ ls" {
            continue;
        }
        else if line.starts_with("$ cd ") {
            let dir = line.strip_prefix("$ cd ").unwrap().to_owned();
            if dir == ".." {
                current.pop();
            }
            else {
                current.push(dir);
            }
        }
        else {
            let (data, name) = line.split_once(" ").expect("invalid listing");
            if data != "dir" {
                let size = data.parse().expect("invalid file size");
                let mut path = current.clone();
                path.push(name.to_owned());
                file_sizes.insert(path, size);
            }
        }
    }

    file_sizes
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_sample() {
        let day = Day7::from_sample(SAMPLE);
        day.solve();
    }
}
