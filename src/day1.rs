use aoc_runner_derive::aoc;

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
