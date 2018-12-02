use aoc_runner_derive::{aoc, aoc_lib};
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn day1_part1(input: &str) -> isize {
  input
    .lines()
    .map(|x| {
      x.parse::<isize>().unwrap()
    })
    .sum()
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &str) -> isize {
  use std::collections::BTreeSet;

  let mut seen = BTreeSet::new();
  let mut acc = 0;
  seen.insert(0);

  loop {
    for line in input.lines() {
      let number = line.parse::<isize>().unwrap();
      acc += number;

      if seen.contains(&acc) {
        return acc;
      } else {
        seen.insert(acc);
      }
    }
  }
}

#[aoc(day2, part1)]
pub fn day2_part1(input: &str) -> isize {
  let mut twos = 0;
  let mut threes = 0;

  for line in input.lines() {
    let mut counts: HashMap<char, isize> = HashMap::new();

    for char in line.chars() {
      *counts.entry(char).or_default() += 1;
    }

    let mut has_two = false;
    let mut has_three = false;

    for count in counts.values() {
      if *count == 2 {
        has_two = true;

      } else if *count == 3 {
        has_three = true;
      }
    }

    if has_two {
      twos += 1;
    }

    if has_three {
      threes += 1;
    }
  }

  twos * threes
}

aoc_lib!{ year = 2018 }