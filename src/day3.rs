use aoc_runner_derive::{aoc, aoc_generator};
use std::prelude::v1::{Vec};
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Claim {
  id: usize,
  x: usize,
  y: usize,
  w: usize,
  h: usize
}

#[aoc_generator(day3)]
pub fn claim_parser(input: &str) -> Vec<Claim> {
  lazy_static! {
      static ref RE: Regex = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();
  }

  let mut claims = Vec::new();

  for line in input.lines() {
    let captures = RE.captures(line).unwrap();

    let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let w = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let h = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

    claims.push(Claim { id, x, y, w, h });
  }

  claims
}

#[aoc(day3, part1)]
pub fn part1(claims: &Vec<Claim>) -> usize {
  let mut occupied = HashSet::new();
  let mut duplicated = HashSet::new();

  for claim in claims {
    for x in claim.x..claim.x + claim.w {
      for y in claim.y..claim.y + claim.h {
        let coord = (x,y);

        if occupied.contains(&coord) {
          duplicated.insert(coord);
        } else {
          occupied.insert(coord);
        }
      }
    }
  }

  duplicated.len()
}

#[aoc(day3, part2)]
pub fn part2(claims: &Vec<Claim>) -> usize {
  let mut occupied_sets = HashMap::new();

  for claim in claims {
    let mut occupied = HashSet::new();

    for x in claim.x..claim.x + claim.w {
      for y in claim.y..claim.y + claim.h {
        let coord = (x,y);
        occupied.insert(coord);
      }
    }

    occupied_sets.insert(claim.id, occupied);
  }

  let mut nonoverlapping = Vec::new();

  for first in claims {
    let first_occupied = occupied_sets.get(&first.id).unwrap();
    let mut overlaps = false;

    for second in claims {
      if first == second {
        continue;
      }

      let second_occupied = occupied_sets.get(&second.id).unwrap();

      if !first_occupied.is_disjoint(second_occupied) {
        overlaps = true;
        break;
      }
    }

    if !overlaps {
      nonoverlapping.push(first.id);
    }
  }

  assert_eq!(1, nonoverlapping.len());
  *nonoverlapping.first().unwrap()
}

#[cfg(test)]
mod tests {
  use crate::day3::{
    Claim,
    claim_parser,
    part1,
    part2
  };

  #[test]
  pub fn claim_parser_test() {
    assert_eq!([Claim { id: 1, x: 1, y: 3, w: 4, h: 4 }], claim_parser("#1 @ 1,3: 4x4").as_ref());
    assert_eq!([Claim { id: 2, x: 3, y: 1, w: 4, h: 4 }], claim_parser("#2 @ 3,1: 4x4").as_ref());
    assert_eq!([Claim { id: 3, x: 5, y: 5, w: 2, h: 2 }], claim_parser("#3 @ 5,5: 2x2").as_ref());

    assert_eq!([Claim { id: 1, x: 287, y: 428, w: 27, h: 20 }], claim_parser("#1 @ 287,428: 27x20").as_ref());
    assert_eq!([Claim { id: 1397, x: 888, y: 761, w: 25, h: 24 }], claim_parser("#1397 @ 888,761: 25x24").as_ref());
  }

  #[test]
  pub fn part1_test() {
    assert_eq!(4, part1(&[
      Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
      Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
      Claim { id: 3, x: 5, y: 5, w: 2, h: 2 }
    ].iter().cloned().collect::<Vec<Claim>>()));
  }

  #[test]
  pub fn part2_test() {
    assert_eq!(3, part2(&[
      Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
      Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
      Claim { id: 3, x: 5, y: 5, w: 2, h: 2 }
    ].iter().cloned().collect::<Vec<Claim>>()));
  }

}
