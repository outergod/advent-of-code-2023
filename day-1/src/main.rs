use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> u16 {
    s.lines()
        .into_iter()
        .map(|s| {
            let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let s: String = [digits.first().unwrap(), digits.last().unwrap()]
                .into_iter()
                .collect();
            s.parse::<u16>().unwrap()
        })
        .sum()
}

fn solve2(s: &str) -> u16 {
    lazy_static! {
        static ref RE_FIRST: Regex =
            Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        static ref RE_LAST: Regex =
            Regex::new(r"(\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    }

    s.lines()
        .into_iter()
        .map(|s| {
            let first = RE_FIRST.find(s).unwrap().as_str().to_string();
            let last = RE_LAST
                .find(s.chars().rev().collect::<String>().as_str())
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect();

            let digits: Vec<u16> = [first, last]
                .into_iter()
                .map(|s| match s.as_str() {
                    "1" | "one" => 1,
                    "2" | "two" => 2,
                    "3" | "three" => 3,
                    "4" | "four" => 4,
                    "5" | "five" => 5,
                    "6" | "six" => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine" => 9,
                    _ => 0,
                })
                .collect();

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = include_str!("example-1");
    const EXAMPLE2: &'static str = include_str!("example-2");

    #[test]
    fn test_example1() {
        assert_eq!(solve1(EXAMPLE1), 142);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solve2(EXAMPLE2), 281);
    }
}
