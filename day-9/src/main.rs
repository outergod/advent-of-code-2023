const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

fn solve1(s: &str) -> i32 {
    s.lines()
        .map(|line| {
            let mut nums: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let mut sequences = Vec::new();

            while !nums.iter().all(|n| *n == 0) {
                sequences.push(nums.clone());
                nums = nums.windows(2).map(|nums| nums[1] - nums[0]).collect();
            }

            sequences
                .into_iter()
                .map(|seq| seq.last().cloned().unwrap())
                .sum::<i32>()
        })
        .sum()
}

fn solve2(s: &str) -> i32 {
    s.lines()
        .map(|line| {
            let mut nums: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let mut sequences = Vec::new();

            while !nums.iter().all(|n| *n == 0) {
                sequences.push(nums.clone());
                nums = nums.windows(2).map(|nums| nums[1] - nums[0]).collect();
            }

            sequences
                .into_iter()
                .map(|seq| seq.first().cloned().unwrap())
                .rev()
                .reduce(|acc, n| n - acc)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE), 114);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE), 2);
    }
}
