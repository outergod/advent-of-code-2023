use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("input");

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
}

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> u16 {
    let mut table = HashMap::new();
    let mut lines = s.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    lines.next();

    while let Some(line) = lines.next() {
        let (_, [key, left, right]) = RE.captures(line).unwrap().extract();
        table.insert(key, (left, right));
    }

    let mut pointer = "AAA";
    for (instruction, i) in instructions.zip(0..) {
        if pointer == "ZZZ" {
            return i;
        }
        pointer = match instruction {
            'R' => table.get(pointer).unwrap().1,
            'L' => table.get(pointer).unwrap().0,
            _ => panic!(),
        }
    }

    unreachable!()
}

fn solve2(s: &str) -> u64 {
    let mut table = HashMap::new();
    let mut lines = s.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    lines.next();

    while let Some(line) = lines.next() {
        let (_, [key, left, right]) = RE.captures(line).unwrap().extract();
        table.insert(key, (left, right));
    }

    let mut pointers: Vec<_> = table
        .keys()
        .cloned()
        .filter(|key| key.ends_with('A'))
        .collect();
    let mut goals = Vec::new();

    for (instruction, i) in instructions.zip(0..) {
        if goals.len() == pointers.len() {
            break;
        }

        for pointer in pointers.iter_mut() {
            if pointer.ends_with('Z') {
                goals.push(i);
            }
            *pointer = match instruction {
                'R' => table.get(pointer).unwrap().1,
                'L' => table.get(pointer).unwrap().0,
                _ => panic!(),
            }
        }
    }

    goals.into_iter().reduce(num::integer::lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = include_str!("example-1");
    const EXAMPLE2: &'static str = include_str!("example-2");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE1), 6);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE2), 6);
    }
}
