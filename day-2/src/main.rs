use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> u16 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Game (\d+): (.+)").unwrap();
    }

    s.lines()
        .into_iter()
        .map(|s| {
            let game = RE.captures(s).unwrap();
            let id = game.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let sets: Vec<_> = game.get(2).unwrap().as_str().split("; ").collect();

            if sets.iter().all(|set| {
                set.split(", ").all(|cubes| {
                    let mut s = cubes.split_whitespace();
                    let n = s.next().unwrap().parse::<u16>().unwrap();
                    match s.next() {
                        Some("red") => n <= 12,
                        Some("green") => n <= 13,
                        Some("blue") => n <= 14,
                        Some(_) | None => panic!(),
                    }
                })
            }) {
                id
            } else {
                0
            }
        })
        .sum()
}

#[derive(Default)]
struct Cubes {
    red: u16,
    green: u16,
    blue: u16,
}

impl Cubes {
    pub fn power(&self) -> u16 {
        self.red * self.green * self.blue
    }
}

fn solve2(s: &str) -> u16 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Game (\d+): (.+)").unwrap();
        static ref RE_SPLIT: Regex = Regex::new(r"[;,] ").unwrap();
    }

    s.lines()
        .into_iter()
        .map(|s| {
            let game = RE.captures(s).unwrap();
            let mut result: Cubes = Default::default();

            RE_SPLIT
                .split(game.get(2).unwrap().as_str())
                .for_each(|cubes| {
                    let mut s = cubes.split_whitespace();
                    let n = s.next().unwrap().parse::<u16>().unwrap();
                    match s.next() {
                        Some("red") => {
                            result.red = result.red.max(n);
                        }
                        Some("green") => {
                            result.green = result.green.max(n);
                        }
                        Some("blue") => {
                            result.blue = result.blue.max(n);
                        }
                        Some(_) | None => panic!(),
                    }
                });

            result.power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = include_str!("example-1");

    #[test]
    fn test_example1() {
        assert_eq!(solve1(EXAMPLE1), 8);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solve2(EXAMPLE1), 2286);
    }
}
