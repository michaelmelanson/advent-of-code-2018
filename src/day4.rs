use aoc_runner_derive::{aoc, aoc_generator};
use std::prelude::v1::{Vec};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::ops::Sub;
use std::fmt::{Formatter, Error, Debug};
use std::iter::Step;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time {
  pub year: u16,
  pub month: u16,
  pub day: u16,
  pub hour: u16,
  pub minute: u16
}

impl Debug for Time {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}-{:#02}-{:#02} {:#02}:{:#02}", self.year, self.month, self.day, self.hour, self.minute)
  }
}

impl Ord for Time {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.year != other.year {
      self.year.cmp(&other.year)

    } else if self.month != other.month {
      self.month.cmp(&other.month)

    } else if self.day != other.day {
      self.day.cmp(&other.day)

    } else if self.hour != other.hour {
      self.hour.cmp(&other.hour)

    } else if self.minute != other.minute {
      self.minute.cmp(&other.minute)
    } else {

      Ordering::Equal
    }
  }
}

impl PartialOrd for Time {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Sub for Time {
  type Output = isize;

  fn sub(self, rhs: Time) -> <Self as Sub<Time>>::Output {
    (self.year as isize - rhs.year as isize)   * 12 * 30 * 24 * 60 +
    (self.month as isize - rhs.month as isize)      * 30 * 24 * 60 +
    (self.day as isize   - rhs.day as isize)             * 24 * 60 +
    (self.hour as isize  - rhs.hour as isize)                 * 60 +
    (self.minute as isize - rhs.minute as isize)
  }
}

impl Step for Time {
  fn steps_between(start: &Self, end: &Self) -> Option<usize> {
    let diff = *end - *start;

    if diff > 0 {
      Some(diff as usize)
    } else {
      None
    }
  }

  fn replace_one(&mut self) -> Self {
    unimplemented!()
  }

  fn replace_zero(&mut self) -> Self {
    unimplemented!()
  }

  fn add_one(&self) -> Self {
    Time {
      year: self.year,
      month: self.month,
      day: self.day,
      hour: self.hour,
      minute: self.minute + 1
    }
  }

  fn sub_one(&self) -> Self {
    unimplemented!()
  }

  fn add_usize(&self, n: usize) -> Option<Self> {
    Some(Time {
      year: self.year,
      month: self.month,
      day: self.day,
      hour: self.hour,
      minute: (self.minute as usize + n) as u16
    })
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RecordEvent {
  BeginShift(u64),
  FallsAsleep,
  WakesUp
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Record {
  pub time: Time,
  pub event: RecordEvent
}

impl PartialOrd for Record {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.time.partial_cmp(&other.time)
  }
}

impl Ord for Record {
  fn cmp(&self, other: &Self) -> Ordering {
    self.time.cmp(&other.time)
  }
}

#[aoc_generator(day4)]
pub fn record_parser(input: &str) -> Vec<Record> {
  lazy_static! {
      static ref BEGIN_SHIFT_RE: Regex  = Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)\] Guard \#(\d*) begins shift$").unwrap();
      static ref FALLS_ASLEEP_RE: Regex = Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)\] falls asleep$").unwrap();
      static ref WAKES_UP_RE: Regex     = Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)\] wakes up$").unwrap();
  }

  let mut records = Vec::new();

  for line in input.lines() {
    for cap in BEGIN_SHIFT_RE.captures_iter(line) {
      records.push(Record {
        time: Time {
          year: cap[1].parse::<u16>().unwrap(),
          month: cap[2].parse::<u16>().unwrap(),
          day: cap[3].parse::<u16>().unwrap(),
          hour: cap[4].parse::<u16>().unwrap(),
          minute: cap[5].parse::<u16>().unwrap(),
        },
        event: RecordEvent::BeginShift(cap[6].parse::<u64>().unwrap())
      });
    }

    for cap in FALLS_ASLEEP_RE.captures_iter(line) {
      records.push(Record {
        time: Time {
          year: cap[1].parse::<u16>().unwrap(),
          month: cap[2].parse::<u16>().unwrap(),
          day: cap[3].parse::<u16>().unwrap(),
          hour: cap[4].parse::<u16>().unwrap(),
          minute: cap[5].parse::<u16>().unwrap(),
        },
        event: RecordEvent::FallsAsleep
      });
    }

    for cap in WAKES_UP_RE.captures_iter(line) {
      records.push(Record {
        time: Time {
          year: cap[1].parse::<u16>().unwrap(),
          month: cap[2].parse::<u16>().unwrap(),
          day: cap[3].parse::<u16>().unwrap(),
          hour: cap[4].parse::<u16>().unwrap(),
          minute: cap[5].parse::<u16>().unwrap(),
        },
        event: RecordEvent::WakesUp
      });
    }
  }

  records
}

pub fn process(records: &Vec<Record>) -> (HashMap<u64, isize>, HashMap<u64, HashMap<u16, usize>>) {
  let mut records = records.clone();
  records.sort();

  let mut events:HashMap<u64, Vec<Record>> = HashMap::new();

  let mut guard = 0;

  for record in records {
    //
    assert_eq!(record.time.year, 1518);

    if let RecordEvent::BeginShift(id) = record.event {
      guard = id;
    }

    let entry = events.entry(guard).or_insert(Vec::new());
    entry.push(record);
  }

  let mut time_asleep = HashMap::new();
  let mut minutes_asleep = HashMap::new();

  for (guard, records) in events {
    let iter = records.iter();

    let mut sleep_time = None;

    for record in iter {
      if record.event == RecordEvent::FallsAsleep {
        sleep_time = Some(record.time);
      } else if record.event == RecordEvent::WakesUp && sleep_time.is_some(){
        let sleep_time = sleep_time.unwrap();

        if sleep_time.day == record.time.day && sleep_time.month == record.time.month {
          let diff = record.time - sleep_time;

          *time_asleep.entry(guard).or_insert(0) += diff;

          for t in sleep_time..record.time {
            *minutes_asleep
              .entry(guard).or_insert(HashMap::new())
              .entry(t.minute).or_insert(0) += 1;
          }
        }
      }
    }
  }

  (time_asleep, minutes_asleep)
}

#[aoc(day4, part1)]
pub fn part1(records: &Vec<Record>) -> usize {

  let (time_asleep, minutes_asleep) = process(records);

  let (best_guard,_) = time_asleep.iter()
    .max_by(|(_,x),(_,y)| x.cmp(y))
    .unwrap();

  let (best_minute,_) = minutes_asleep.get(best_guard).unwrap().iter()
    .max_by(|(_,x),(_,y)| x.cmp(y))
    .unwrap();

  (*best_guard as usize) * (*best_minute as usize)
}

#[aoc(day4, part2)]
pub fn part2(records: &Vec<Record>) -> usize {
  let (_time_asleep, minutes_asleep) = process(records);

  let mut best_count: usize = 0;
  let mut best_guard: usize = 0;
  let mut best_minute: usize = 0;

  for (guard, map) in minutes_asleep {
    for (minute, count) in map {
      if count > best_count {
        best_count = count;
        best_guard = guard as usize;
        best_minute = minute as usize;
      }
    }
  }

  best_guard * best_minute
}

#[cfg(test)]
mod tests {
  use crate::day4::*;
  use std::cmp::Ordering;

  #[test]
  pub fn time_ordering_test() {
    let input = [
      Time { year: 1001, month: 10, day: 11, hour: 11, minute: 11 },
      Time { year: 1001, month: 11, day: 11, hour: 10, minute: 11 },
      Time { year: 1000, month: 11, day: 11, hour: 11, minute: 11 },
      Time { year: 1001, month: 11, day: 11, hour: 11, minute: 10 },
      Time { year: 1001, month: 11, day: 10, hour: 11, minute: 11 },
    ].to_vec();

    let expected = [
      Time { year: 1000, month: 11, day: 11, hour: 11, minute: 11 },
      Time { year: 1001, month: 10, day: 11, hour: 11, minute: 11 },
      Time { year: 1001, month: 11, day: 10, hour: 11, minute: 11 },
      Time { year: 1001, month: 11, day: 11, hour: 10, minute: 11 },
      Time { year: 1001, month: 11, day: 11, hour: 11, minute: 10 }
    ].to_vec();

    let actual = {
      let mut x = input.clone();
      x.sort();
      x
    };

    assert_eq!(actual, expected);

  }

  #[test]
  pub fn record_parser_test() {
    assert_eq!([
       Record { time: Time { year: 1518, month: 11, day: 1, hour: 0, minute: 0 }, event: RecordEvent::BeginShift(10) },
       Record { time: Time { year: 1518, month: 11, day: 1, hour: 0, minute: 5 }, event: RecordEvent::FallsAsleep },
       Record { time: Time { year: 1518, month: 11, day: 1, hour: 0, minute: 25 }, event: RecordEvent::WakesUp },
    ],
    record_parser(
      "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up"
    ).as_ref());
  }

  #[test]
  pub fn part1_test() {
    let records = record_parser("[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up");
    assert_eq!(240, part1(&records));
  }
}