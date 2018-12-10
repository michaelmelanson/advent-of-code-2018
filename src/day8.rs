use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
pub struct Node(char, Vec<usize>, Vec<Node>);

impl Node {
  pub fn metadata(&self) -> &Vec<usize> { &self.1 }
  pub fn childen(&self) -> &Vec<Node> { &self.2 }
}

#[derive(Debug, PartialEq)]
pub struct Licence {
  root: Node
}

fn parse_node<A: Iterator<Item=usize>, B: Iterator<Item=char>>(numbers: &mut A, names: &mut B) -> Node {
  let num_children = numbers.next().unwrap();
  let num_metadata = numbers.next().unwrap();

  let name = names.next().expect("Ran out of names!");
  let mut children = Vec::new();
  let mut metadata = Vec::new();

  for _ in 0..num_children {
    children.push(parse_node(numbers, names));
  }

  for _ in 0..num_metadata {
    metadata.push(numbers.next().expect("No more numbers for metadata"));
  }

  Node(name, metadata, children)
}

pub fn parse_input(input: &str) -> Licence {
  let mut names = "ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890abcdefghijklmnopqrstuvwxyz".chars().into_iter().cycle();
  let mut numbers = input.split_whitespace().map(|str| str.parse().unwrap());

  let root = parse_node(&mut numbers, &mut names);

  Licence { root }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
  let licence = parse_input(input);

  println!("License: {:?}", licence);

  fn sum_metadata(node: &Node) -> usize {
    let mut sum = 0;

    for child in node.childen() {
      sum += sum_metadata(child);
    }

    for metadata in node.metadata() {
      sum += metadata;
    }

    sum
  }

  sum_metadata(&licence.root)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
  let licence = parse_input(input);

  fn node_value(node: Option<&Node>) -> usize {
    let mut value = 0;

    if let Some(node) = node {
      if node.childen().is_empty() {
        for metadata in node.metadata() {
          value += metadata
        }
      } else {
        for metadata in node.metadata() {
          value += node_value(node.childen().get(metadata - 1));
        }
      }
    }

    value
  }

  node_value(Some(&licence.root))
}

#[cfg(test)]
mod tests {
  use crate::day8::*;

  #[test]
  pub fn licence_parser_test() {
    assert_eq!(Licence {
      root: Node('A', vec![1, 1, 2], vec![
        Node('B', vec![10, 11, 12], vec![]),
        Node('C', vec![2], vec![
          Node('D', vec![99], vec![])
        ])
      ])
    }, parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
  }

  #[test]
  pub fn part1_test() {
    assert_eq!(138, part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
  }
}