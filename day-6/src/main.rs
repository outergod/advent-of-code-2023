use std::ops::Mul;

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn win(&self) -> usize {
        (0..=self.time)
            .filter(|i| (self.time - i) * i > self.record)
            .count()
    }
}

fn parse1(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let mut times = lines.next().unwrap().split_whitespace();
    let mut records = lines.next().unwrap().split_whitespace();

    assert_eq!(times.next(), Some("Time:"));
    assert_eq!(records.next(), Some("Distance:"));

    times
        .zip(records)
        .map(|(time, record)| Race {
            time: time.parse::<u64>().unwrap(),
            record: record.parse::<u64>().unwrap(),
        })
        .collect()
}

fn parse2(s: &str) -> Race {
    let mut lines = s.lines();
    let mut times = lines.next().unwrap().split_whitespace();
    let mut records = lines.next().unwrap().split_whitespace();

    assert_eq!(times.next(), Some("Time:"));
    assert_eq!(records.next(), Some("Distance:"));

    Race {
        time: times
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap(),
        record: records
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap(),
    }
}

fn solve1(s: &str) -> u64 {
    parse1(s)
        .into_iter()
        .map(|race| race.win() as u64)
        .reduce(Mul::mul)
        .unwrap()
}

fn solve2(s: &str) -> u64 {
    parse2(s).win() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE), 288);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE), 71503);
    }
}
