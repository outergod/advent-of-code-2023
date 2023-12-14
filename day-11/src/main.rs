use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn parse(s: &str, age: u32) -> HashSet<(i128, i128)> {
    let mut galaxies = HashSet::new();
    let mut cols = HashSet::new();
    let mut rows = HashSet::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x as i128, y as i128));
                cols.insert(x as i128);
                rows.insert(y as i128);
            }
        }
    }

    let mut cols: Vec<_> = cols.into_iter().collect();
    cols.sort();
    let gaps_x: HashMap<_, _> = cols
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, (x - i as i128) * (age - 1) as i128))
        .collect();
    let mut rows: Vec<_> = rows.into_iter().collect();
    rows.sort();
    let gaps_y: HashMap<_, _> = rows
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, (x - i as i128) * (age - 1) as i128))
        .collect();

    galaxies
        .into_iter()
        .map(|(x, y)| (x + gaps_x.get(&x).unwrap(), y + gaps_y.get(&y).unwrap()))
        .collect()
}

fn solve(s: &str, age: u32) -> u128 {
    let mut galaxies: Vec<_> = parse(s, age).into_iter().collect();
    let mut pairs: Vec<_> = Vec::new();
    while let Some(left) = galaxies.pop() {
        for right in &galaxies {
            pairs.push((left, *right));
        }
    }

    pairs
        .into_iter()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .reduce(|x, y| x + y)
        .unwrap()
}

fn solve1(s: &str) -> u128 {
    solve(s, 2)
}

fn solve2(s: &str) -> u128 {
    solve(s, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution() {
        assert_eq!(solve1(EXAMPLE), 374);
    }
}
