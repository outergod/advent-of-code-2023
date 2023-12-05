use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

#[derive(Debug, Clone)]
struct Range {
    source_range_start: u128,
    destination_range_start: u128,
    range_length: u128,
}

impl Range {
    pub fn map(&self, n: u128) -> Option<u128> {
        if self.source_range_start <= n && n < self.source_range_start + self.range_length {
            Some(self.destination_range_start + (n - self.source_range_start))
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u128>,
    maps: HashMap<(String, String), Vec<Range>>,
}

impl Almanac {
    fn map(&self, source: &str, destination: &str, number: u128) -> u128 {
        self.maps
            .get(&(source.to_string(), destination.to_string()))
            .unwrap()
            .iter()
            .find_map(|range| range.map(number))
            .unwrap_or(number)
    }

    pub fn closest(&self, groups: bool) -> u128 {
        let seeds: Vec<u128> = if groups {
            self.seeds
                .chunks(2)
                .filter_map(|pair| match pair {
                    [start, length] => Some((start, length)),
                    _ => None,
                })
                .flat_map(|(start, length)| *start..(start + length))
                .collect()
        } else {
            self.seeds.clone()
        };

        seeds
            .into_iter()
            .map(|seed| self.map("seed", "soil", seed))
            .map(|soil| self.map("soil", "fertilizer", soil))
            .map(|fertilizer| self.map("fertilizer", "water", fertilizer))
            .map(|water| self.map("water", "light", water))
            .map(|light| self.map("light", "temperature", light))
            .map(|temperature| self.map("temperature", "humidity", temperature))
            .map(|humidity| self.map("humidity", "location", humidity))
            .min()
            .unwrap()
    }
}

fn parse_seeds(lines: &mut VecDeque<&str>, result: &mut Almanac) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^seeds: (.+)").unwrap();
    }

    let seeds = match lines.front().and_then(|line| RE.captures(line)) {
        Some(captures) => captures.get(1).unwrap(),
        None => return false,
    };

    lines.pop_front();

    result.seeds = seeds
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    true
}

fn parse_map(lines: &mut VecDeque<&str>, almanac: &mut Almanac) -> bool {
    lazy_static! {
        static ref RE_MAP: Regex = Regex::new(r"^([^-]+)-to-([^ ]+) map:").unwrap();
        static ref RE_RANGE: Regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    }

    let (_, [source, destination]) = match lines.front().and_then(|line| RE_MAP.captures(line)) {
        Some(captures) => captures.extract(),
        None => return false,
    };

    let mut ranges = Vec::new();

    lines.pop_front();

    while let Some(captures) = lines.front().and_then(|line| RE_RANGE.captures(line)) {
        lines.pop_front();
        let (_, [destination, source, length]) = captures.extract();
        ranges.push(Range {
            destination_range_start: destination.parse::<u128>().unwrap(),
            source_range_start: source.parse::<u128>().unwrap(),
            range_length: length.parse::<u128>().unwrap(),
        })
    }

    almanac
        .maps
        .insert((source.to_string(), destination.to_string()), ranges);

    true
}

fn parse(s: &str) -> Almanac {
    let mut almanac: Almanac = Default::default();
    let mut lines: VecDeque<_> = s.lines().collect();

    while let Some(line) = lines.front().cloned() {
        if line.trim().is_empty() {
            lines.pop_front();
            continue;
        }

        if parse_seeds(&mut lines, &mut almanac) {
            continue;
        }

        if parse_map(&mut lines, &mut almanac) {
            continue;
        }

        panic!("Couldn't parse {}", line);
    }

    almanac
}

fn solve1(s: &str) -> u128 {
    let almanac = parse(s);
    almanac.closest(false)
}

fn solve2(s: &str) -> u128 {
    let almanac = parse(s);
    almanac.closest(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE), 35);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE), 46);
    }
}
