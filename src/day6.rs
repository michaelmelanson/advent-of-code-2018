use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub struct Bounds {
  min_x: i16,
  max_x: i16,
  min_y: i16,
  max_y: i16
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coord {
  x: i16,
  y: i16,
  name: char
}

#[aoc_generator(day6)]
pub fn coordinate_parser(input: &str) -> Vec<Coord> {
  let mut coords = Vec::new();


  for (index, line) in input.lines().enumerate() {
    let parts: Vec<&str> = line.split(", ").collect();

    let x = parts.get(0).unwrap().parse().unwrap();
    let y = parts.get(1).unwrap().parse().unwrap();
    let name = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890abcdefghijklmnopqrstuvwxyz".chars().nth(index).unwrap();

    coords.push(Coord { x, y, name });
  }

  coords
}

fn bounds_for_coords(coords: &Vec<Coord>) -> Bounds {
  let mut bounds = Bounds {
    min_x: 0,
    max_x: 0,
    min_y: 0,
    max_y: 0
  };

  for coord in coords {
    if coord.x < bounds.min_x {
      bounds.min_x = coord.x;
    }

    if coord.x > bounds.max_x {
      bounds.max_x = coord.x;
    }

    if coord.y < bounds.min_y {
      bounds.min_y = coord.y;
    }

    if coord.y > bounds.max_y {
      bounds.max_y = coord.y;
    }
  }

  bounds.min_x -= 1;
  bounds.min_y -= 1;
  bounds.max_x += 1;
  bounds.max_y += 1;

  bounds
}
#[aoc(day6, part1)]
pub fn part1(coords: &Vec<Coord>) -> u16 {

  let bounds = bounds_for_coords(coords);

  let mut region_size = HashMap::new();
  let mut infinite_regions = HashSet::new();

  for y in bounds.min_y..bounds.max_y {
    for x in bounds.min_x..bounds.max_x {
      let mut distances = HashMap::new();

      for (index, coord) in coords.iter().enumerate() {
        let dist = num::abs(x - coord.x) + num::abs(y - coord.y);

        distances.entry(dist)
          .or_insert(Vec::new())
          .push(index);
      }

      let best_distance = distances.keys().min().unwrap();
      let best_regions = distances.get(best_distance).unwrap();

      // one coordinate is closest
      if best_regions.len() == 1 {
        let best_index = *best_regions.first().unwrap();
        let best_coord = *coords.get(best_index).unwrap();
        // print!("{}", best_coord.name);

        *region_size.entry(best_index).or_insert(0) += 1;

        if x == bounds.min_x || x == bounds.max_x || y == bounds.min_y || y == bounds.max_y {
          infinite_regions.insert(best_index);
        }
      } else {
        // print!("{}", ".");
      }
    }
    // println!();
  }

  println!("Infinite regions: {:?}", infinite_regions);
  println!("Region sizes: {:?}", region_size);

  let mut biggest_size = 0;
  for (index, size) in region_size.iter() {
    if !infinite_regions.contains(index) && *size > biggest_size {
      biggest_size = *size;
    }
  }

  biggest_size
}

#[aoc(day6, part2)]
pub fn part2(coords: &Vec<Coord>) -> u16 {
  let mut region_size = 0;

  let bounds = bounds_for_coords(coords);

  for y in bounds.min_y..bounds.max_y {
    for x in bounds.min_x..bounds.max_x {

      let mut total_distance = 0;

      for (index, coord) in coords.iter().enumerate() {
        let dist = num::abs(x - coord.x) + num::abs(y - coord.y);

        total_distance += dist;
      }

      if total_distance < 10000 {
        region_size += 1;
        // print!("#");
      } else {
        // print!(".");
      }
    }

    // println!();
  }

  region_size
}

#[cfg(test)]
mod tests {
  use crate::day6::*;

  #[test]
  pub fn coordinate_parser_test() {
    assert_eq!([
      Coord { x: 1, y: 1, name: 'A' },
      Coord { x: 1, y: 6, name: 'B' },
      Coord { x: 8, y: 3, name: 'C' },
      Coord { x: 3, y: 4, name: 'D' },
      Coord { x: 5, y: 5, name: 'E' },
      Coord { x: 8, y: 9, name: 'F' },
    ].to_vec(), coordinate_parser("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9"));
  }

  #[test]
  pub fn part1_test() {
    assert_eq!(17, part1(&coordinate_parser("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9")))
  }
}