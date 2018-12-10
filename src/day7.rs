use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rule {
  step: char,
  prerequisite: char
}

#[aoc_generator(day7)]
pub fn rule_parser(input: &str) -> Vec<Rule> {
  let mut rules = Vec::new();
  lazy_static! {
      static ref RULE_RE: Regex  = Regex::new(r"Step (.) must be finished before step (.) can begin.$").unwrap();
  }

  for line in input.lines() {
    for cap in RULE_RE.captures_iter(line) {
      rules.push(Rule {
        step: cap[2].parse::<char>().unwrap(),
        prerequisite: cap[1].parse::<char>().unwrap()
      });
    }
  }

  rules
}

#[aoc(day7, part1)]
pub fn part1(rules: &Vec<Rule>) -> String {
  let mut prerequisites: HashMap<char, HashSet<char>> = HashMap::new();
  let mut open: HashSet<char> = HashSet::new();

  for rule in rules {
    prerequisites.entry(rule.step).or_default().insert(rule.prerequisite);
    open.insert(rule.step);
    open.insert(rule.prerequisite);
  }

  let mut closed: HashSet<char> = HashSet::new();
  let mut sequence: Vec<char> = Vec::new();

  loop {
    let mut satisfied: Vec<char> = Vec::new();

    println!("Round started with open={:?}, closed={:?}", open, closed);

    for step in open.iter() {
      let prereqs = prerequisites.entry(*step).or_default();

      if prereqs.is_subset(&closed) {
        println!("  All prerequisites of {} are closed: {:?}", step, prereqs);
        satisfied.push(*step);
      }
    }

    satisfied.sort();

    println!("  Satisfied list: {:?}", satisfied);

    let next = satisfied.first().unwrap();

    println!("  Doing step {}", next);

    closed.insert(*next);
    sequence.push(*next);
    open.remove(next);

    if open.is_empty() {
      break;
    }
  }

  sequence.into_iter().collect()
}

#[aoc(day7, part2)]
pub fn part2(rules: &Vec<Rule>) -> usize {
  part2_inner(5, 60, rules)
}

pub fn part2_inner(num_elves: usize, base_step_duration: usize, rules: &Vec<Rule>) -> usize {
  #[derive(Copy, Clone, Debug)]
  struct Elf {
    step: Option<char>,
    time_left: usize
  }

  let mut step_durations = HashMap::new();
  let mut duration = base_step_duration;
  for step in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
    duration += 1;
    step_durations.insert(step, duration);
  }

  let mut elves = Vec::new();
  elves.resize(num_elves, Elf { step: None, time_left: 0 });

  let mut prerequisites: HashMap<char, HashSet<char>> = HashMap::new();
  let mut open: HashSet<char> = HashSet::new();

  for rule in rules {
    prerequisites.entry(rule.step).or_default().insert(rule.prerequisite);
    open.insert(rule.step);
    open.insert(rule.prerequisite);
  }

  let mut closed: HashSet<char> = HashSet::new();
  let mut sequence: Vec<char> = Vec::new();

  let mut second = 0;
  loop {
    // finish any tasks that are done
    for elf in elves.iter_mut() {
      if let Some(step) = elf.step {
        elf.time_left -= 1;

        if elf.time_left == 0 {
          elf.step = None;
          closed.insert(step);
        }
      }
    }

    let mut satisfied: Vec<char> = Vec::new();

    // println!("Round started with open={:?}, closed={:?}", open, closed);

    for step in open.iter() {
      let prereqs = prerequisites.entry(*step).or_default();

      if prereqs.is_subset(&closed) {
        // println!("  All prerequisites of {} are closed: {:?}", step, prereqs);
        satisfied.push(*step);
      }
    }

    satisfied.sort();

    // println!("  Satisfied list: {:?}", satisfied);

    // assign the tasks
    for step in satisfied {
      let elf = elves.iter_mut().find(|e| e.step.is_none());

      match elf {
        Some(elf) => {
          elf.step = Some(step);
          elf.time_left = *step_durations.get(&step).unwrap();
          open.remove(&step);
        },
        None => break
      }
    }

    println!("{:4} {:?}", second, elves.iter().map(|e| e.step.unwrap_or('.')).collect::<Vec<char>>());

    if elves.iter().find(|e| e.step.is_some()).is_none() {
      break;
    }

    second += 1;

  }

  second
}


#[cfg(test)]
mod tests {
  use crate::day7::*;

  #[test]
  pub fn rule_parser_test() {
    assert_eq!([
                 Rule { step: 'A', prerequisite: 'C' },
                 Rule { step: 'F', prerequisite: 'C' },
                 Rule { step: 'B', prerequisite: 'A' },
                 Rule { step: 'D', prerequisite: 'A' },
                 Rule { step: 'E', prerequisite: 'B' },
                 Rule { step: 'E', prerequisite: 'D' },
                 Rule { step: 'E', prerequisite: 'F' },
    ].to_vec(), rule_parser("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."));
  }

  #[test]
  pub fn part1_test() {
    assert_eq!("CABDFE", part1(&rule_parser("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.")));
  }

  #[test]
  pub fn part2_test() {
    assert_eq!(20, part2_inner(2, 1, &rule_parser("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.")));
  }

}
