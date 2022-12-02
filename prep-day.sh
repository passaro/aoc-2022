#!/bin/sh

set -e

# Downloads the input and sets up module boilerplate for
# the given day. Expects that a `.session` file exists with the
# user's session key from the Advent of Code website. See the
# README for instructions on how to set it up.
#
# This script should be idempotent, so don't worry about things
# breaking if you run it for the same day multiple times.
#
# Usage:
# ./prep-day.sh 10

YEAR=2022

if test -z "$1"; then
  echo "Must provide day of month (not zero-padded) as first argument"
  exit 1
fi

if [[ 1 -gt "$1" || 25 -lt "$1" ]]; then
  echo "Day must be between 1 and 25, inclusive"
  exit 1
fi

SESSION=$(cat .session)
if test -z "$SESSION"; then
  echo "Must set the session from the Advent of Code site"
  exit 1
fi

if test -e ".input/$1.txt"; then
  echo "Data already exists for day $1, skipping download..."
else
  echo "Downloading data for day $1 to .input/$1.txt..."
  mkdir -p .input
  curl "https://adventofcode.com/$YEAR/day/$1/input" \
    --silent --max-time 10 --cookie "session=$SESSION" > ".input/$1.txt"
fi

if test -e "src/day$1.rs"; then
  echo "src/day$1.rs already exists, skipping..."
else
  echo "Creating boilerplate module for day $1 at src/day$1.rs..."
  echo "Remember to update main.rs:"
  echo "  - Add:"
  echo "mod day$1;"
  echo "use day$1::Day$1;"
  echo ""
  echo "  - Update 'get_day_solution' to use 'Day$1':"
  echo "$1 => Box::new(Day$1::from_lines(lines)),"

  cat <<-EOF > "src/day$1.rs"
use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day$1;

impl FromInput for Day$1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        todo!("Parse your input from the input file")
    }
}

impl DaySolution for Day$1 {
    fn part_one(&self) -> Option<String> {
        None
    }

    fn part_two(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"
    ";

    #[test]
    fn test_sample() {
        let day = Day$1::from_sample(SAMPLE);
        day.solve();
    }
}
EOF
fi

echo "Happy coding!"
