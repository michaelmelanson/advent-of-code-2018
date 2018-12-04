use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::string::String;

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


#[aoc(day2, part2)]
pub fn day2_part2(input: &str) -> String {

  let mut best_common = String::new();

  let lines = input.lines().collect::<Vec<&str>>();

  for first_index in 0..lines.len() {
    let first_word = String::from(*lines.get(first_index).unwrap());
    for second_index in first_index+1..lines.len() {
      let second_word = String::from(*lines.get(second_index).unwrap());

      let mut common: String = String::new();

      for char_index in 0..first_word.len() {
        let a: char = first_word.chars().nth(char_index).unwrap();
        let b: char = second_word.chars().nth(char_index).unwrap();

        if a == b {
          common.push(a);
        }
      }

      if common.len() > best_common.len() {
        best_common = common;
      }

    }
  }

  return best_common;
}
