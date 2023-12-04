use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Card +\d+: ([^|]+)\|(.+)").unwrap();
}

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> u16 {
    s.lines()
        .map(|line| {
            let groups = RE.captures(line).unwrap();
            let winning: HashSet<u16> = groups
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            let owned: HashSet<u16> = groups
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();

            let total = owned.intersection(&winning).count() as u32;
            if total == 0 {
                0
            } else {
                2_u16.pow(total - 1)
            }
        })
        .sum()
}

#[derive(Clone)]
struct Card {
    count: u32,
    matches: u16,
}

fn solve2(s: &str) -> u32 {
    let mut cards: Vec<Card> = s
        .lines()
        .map(|line| {
            let groups = RE.captures(line).unwrap();
            let winning: HashSet<u16> = groups
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            let owned: HashSet<u16> = groups
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();

            let matches = owned.intersection(&winning).count() as u16;
            Card { count: 1, matches }
        })
        .collect();

    for x1 in 0..cards.len() {
        let card = cards.get(x1).cloned().unwrap();

        for x2 in (x1 + 1)..(cards.len().min(x1 + 1 + card.matches as usize)) {
            cards.get_mut(x2).unwrap().count += card.count;
        }
    }

    cards.into_iter().fold(0, |acc, card| acc + card.count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE), 13);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE), 30);
    }
}
