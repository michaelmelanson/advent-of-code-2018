use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

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
  let mut ranks: HashMap<char, usize> = HashMap::new();

  loop {
    let mut changed = false;

    for rule in rules {
      let step_rank = *ranks.entry(rule.step).or_default();
      let prereq_rank = *ranks.entry(rule.prerequisite).or_default();

      if step_rank <= prereq_rank {
        println!("Step {} (rank {}) needs to be before step {} (rank {}).",
                 rule.step, step_rank,
                 rule.prerequisite, prereq_rank
        );
        ranks.insert(rule.step, prereq_rank + 1);
        changed = true;
      }
    }

    println!("Ranks: {:?}", ranks);

    if !changed {
      break;
    }
  }


  let mut result = ranks.keys().map(|c| *c).collect::<Vec<char>>();
  result.sort_by(|a, b| {
    match ranks.get(a).unwrap().cmp(ranks.get(b).unwrap()) {
      Ordering::Equal => a.cmp(b),
      x => x
    }
  });

  let mut sorted_ranks = ranks.values().map(|c| *c).collect::<Vec<usize>>();
  sorted_ranks.sort();
  sorted_ranks.dedup();

  for rank in sorted_ranks {
    let mut steps = Vec::new();

    for step in ranks.keys() {
      if *ranks.get(step).unwrap() == rank {
        steps.push(step);
      }
    }

    println!("Rank {}: {:?}", rank, steps);
  }

  result.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use crate::day7::*;

  #[test]
  pub fn rule_parser_test() {
    assert_eq!([
                 Rule { step: 'C', prerequisite: 'A' },
                 Rule { step: 'C', prerequisite: 'F' },
                 Rule { step: 'A', prerequisite: 'B' },
                 Rule { step: 'A', prerequisite: 'D' },
                 Rule { step: 'B', prerequisite: 'E' },
                 Rule { step: 'D', prerequisite: 'E' },
                 Rule { step: 'F', prerequisite: 'E' },
    ].to_vec(), rule_parser("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."));
  }
}
