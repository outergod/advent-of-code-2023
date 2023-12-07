use std::{cmp::Ordering, collections::HashMap};

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

impl From<char> for JokerCard {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand<T>([T; 5])
where
    T: From<char>
        + std::fmt::Debug
        + std::hash::Hash
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord,
    Hand<T>: HandChooser;

impl<T> Ord for Hand<T>
where
    T: From<char>
        + std::fmt::Debug
        + std::hash::Hash
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord,
    Hand<T>: HandChooser,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => self
                .0
                .iter()
                .zip(other.0.iter())
                .find_map(|(this, other)| match this.cmp(other) {
                    Ordering::Equal => None,
                    x => Some(x),
                })
                .unwrap_or(Ordering::Equal),
            x => x,
        }
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: From<char>
        + std::fmt::Debug
        + std::hash::Hash
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord,
    Hand<T>: HandChooser,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> From<&str> for Hand<T>
where
    T: From<char>
        + std::fmt::Debug
        + std::hash::Hash
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord,
    Hand<T>: HandChooser,
{
    fn from(value: &str) -> Self {
        let cards: Vec<T> = value.chars().map(|c| T::from(c)).collect();
        Self(cards.try_into().unwrap())
    }
}

trait HandChooser {
    fn kind(&self) -> HandKind;
}

impl HandChooser for Hand<Card> {
    fn kind(&self) -> HandKind {
        let buckets = self.0.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        let mut powers: Vec<_> = buckets.values().collect();
        powers.sort();
        match powers[..] {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 1, 2] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

impl HandChooser for Hand<JokerCard> {
    fn kind(&self) -> HandKind {
        let mut buckets = self.0.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        let jokers = buckets.remove(&JokerCard::Joker).unwrap_or(0);

        let mut powers: Vec<_> = buckets.into_values().collect();
        powers.sort();
        if let Some(n) = powers.last_mut() {
            *n += jokers;
        } else {
            powers.push(jokers);
        }

        match powers[..] {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 1, 2] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

fn solve1(s: &str) -> u32 {
    let mut camel: Vec<_> = s
        .lines()
        .map(|line| {
            let mut fields = line.split_whitespace();
            let hand: Hand<Card> = fields.next().unwrap().into();
            let bid = fields.next().unwrap().parse::<u32>().unwrap();

            (hand, bid)
        })
        .collect();

    camel.sort_unstable_by_key(|(hand, _)| *hand);

    camel
        .into_iter()
        .zip(1..)
        .fold(0, |acc, ((_, bid), i)| acc + i * bid)
}

fn solve2(s: &str) -> u32 {
    let mut camel: Vec<_> = s
        .lines()
        .map(|line| {
            let mut fields = line.split_whitespace();
            let hand: Hand<JokerCard> = fields.next().unwrap().into();
            let bid = fields.next().unwrap().parse::<u32>().unwrap();

            (hand, bid)
        })
        .collect();

    camel.sort_unstable_by_key(|(hand, _)| *hand);

    camel
        .into_iter()
        .zip(1..)
        .fold(0, |acc, ((_, bid), i)| acc + i * bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = include_str!("example");

    #[test]
    fn test_solution1() {
        assert_eq!(solve1(EXAMPLE), 6440);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solve2(EXAMPLE), 5905);
    }
}
