use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> u32 {
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();
    let mut parts: HashMap<(usize, usize), u32> = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        let mut num: Option<(usize, u32)> = None;

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = c.to_digit(10).unwrap();

                match num {
                    None => {
                        num = Some((x, n));
                    }
                    Some((x, sum)) => num = Some((x, sum * 10 + n)),
                }
            } else {
                if c != '.' {
                    symbols.insert((x, y));
                }

                if let Some((x, n)) = num {
                    parts.insert((x, y), n);
                    num = None;
                }
            }
        }

        if let Some((x, n)) = num {
            parts.insert((x, y), n);
        }
    }

    parts
        .into_iter()
        .filter_map(|((x, y), n)| {
            let digits = n.ilog10() as usize + 1;

            for y in (y.saturating_sub(1))..=(y + 1) {
                for x in (x.saturating_sub(1))..=(x + digits) {
                    if symbols.contains(&(x, y)) {
                        return Some(n);
                    }
                }
            }

            None
        })
        .sum()
}

fn solve2(s: &str) -> u32 {
    let mut gears: HashSet<(usize, usize)> = HashSet::new();
    let mut parts: HashMap<(usize, usize), u32> = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        let mut num: Option<(usize, u32)> = None;

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let n = c.to_digit(10).unwrap();

                match num {
                    None => {
                        num = Some((x, n));
                    }
                    Some((x, sum)) => num = Some((x, sum * 10 + n)),
                }
            } else {
                if c == '*' {
                    gears.insert((x, y));
                }

                if let Some((x, n)) = num {
                    let digits = n.ilog10() as usize + 1;
                    for x in x..(x + digits) {
                        parts.insert((x, y), n);
                    }
                    num = None;
                }
            }
        }

        if let Some((x, n)) = num {
            let digits = n.ilog10() as usize + 1;
            for x in x..(x + digits) {
                parts.insert((x, y), n);
            }
        }
    }

    gears
        .into_iter()
        .filter_map(|(x, y)| {
            let mut found = Vec::new();

            for y in (y.saturating_sub(1))..=(y + 1) {
                for x in (x.saturating_sub(1))..=(x + 1) {
                    if let Some(n) = parts.get(&(x, y)) {
                        found.push(*n);
                    }
                }
            }

            found.sort();
            found.dedup();

            if found.len() == 2 {
                return Some(found.get(0).unwrap() * found.get(1).unwrap());
            } else {
                return None;
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_example1() {
        assert_eq!(solve1(EXAMPLE), 4361);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solve2(EXAMPLE), 467835);
    }
}
