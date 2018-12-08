use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq)]
pub enum Polarity {
  Positive,
  Negative
}

#[derive(Copy, Clone, Debug)]
pub struct Unit(char);

impl Unit {
  pub fn letter(&self) -> char {
    // let's assume no unicode weirdness happens where lowercasing one character turns it into
    // multiple characters.
    self.0.to_lowercase().next().unwrap()
  }

  pub fn polarity(&self) -> Polarity {
    match self.0.is_lowercase() {
      true => Polarity::Negative,
      false => Polarity::Positive
    }
  }

  pub fn reacts_with(&self, rhs: &Self) -> bool {
    self.letter() == rhs.letter() && self.polarity() != rhs.polarity()
  }
}

#[aoc_generator(day5)]
pub fn polymer_parser(input: &str) -> Vec<Unit> {

  let mut polymer = Vec::with_capacity(input.len());

  for c in input.trim().chars() {
    polymer.push(Unit(c));
  }

  polymer
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Unit>) -> usize {
  let mut polymer = input.clone();

  loop {
    let mut result = Vec::new();

    let mut iter = polymer.iter().peekable();

    while let Some(first) = iter.next() {
      if let Some(second) = iter.peek() {
        if first.reacts_with(second) {
          // destroy the second too
          iter.next();
        } else {
          result.push(*first);
        }
      } else {
        result.push(*first);
      }
    }

    if result.len() == polymer.len() {
      return result.len();
    }

    polymer = result;
  }
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Unit>) -> usize {
  let mut best_result = input.len();

  for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
    let filtered = input.into_iter()
      .filter(|a| a.letter() != letter)
      .map(|x| *x)
      .collect::<Vec<Unit>>();

    let result = part1(&filtered);

    if result < best_result {
      best_result = result;
    }
  }

  best_result
}

#[cfg(test)]
mod tests {
  use crate::day5::*;

  #[test]
  pub fn unit_parsing_test() {
    assert_eq!('a', Unit('a').letter());
    assert_eq!('a', Unit('A').letter());
  }

  #[test]
  pub fn reacts_with_test() {
    assert!(Unit('a').reacts_with(&Unit('A')));
    assert!(Unit('A').reacts_with(&Unit('a')));

    assert!(!Unit('a').reacts_with(&Unit('a')));
    assert!(!Unit('a').reacts_with(&Unit('b')));
    assert!(!Unit('a').reacts_with(&Unit('B')));

  }

  #[test]
  pub fn part1_test() {
    assert_eq!(10, part1(&polymer_parser("dabAcCaCBAcCcaDA")));
  }
}
