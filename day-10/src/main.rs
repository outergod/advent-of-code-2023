#![feature(int_roundings)]

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

const INPUT: &'static str = include_str!("input");

fn main() {
    println!("{}", solve1(INPUT));
    println!("{}", solve2(INPUT));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn counterpart(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn next(&self) -> Position {
        match self {
            Direction::North => (0, -1).into(),
            Direction::East => (1, 0).into(),
            Direction::South => (0, 1).into(),
            Direction::West => (-1, 0).into(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    None,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            _ => Self::None,
        }
    }
}

impl Pipe {
    fn connectors(&self) -> HashSet<Direction> {
        match self {
            Pipe::Vertical => HashSet::from([Direction::North, Direction::South]),
            Pipe::Horizontal => HashSet::from([Direction::East, Direction::West]),
            Pipe::NorthEast => HashSet::from([Direction::North, Direction::East]),
            Pipe::NorthWest => HashSet::from([Direction::North, Direction::West]),
            Pipe::SouthEast => HashSet::from([Direction::South, Direction::East]),
            Pipe::SouthWest => HashSet::from([Direction::South, Direction::West]),
            Pipe::None => HashSet::new(),
        }
    }
}

struct Game {
    map: HashMap<Position, Pipe>,
    start: Position,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = None;

        for (line, y) in value.lines().zip(0i32..) {
            for (c, x) in line.chars().zip(0i32..) {
                if c == 'S' {
                    start = Some((x, y));
                } else {
                    map.insert((x, y).into(), Pipe::from(c));
                }
            }
        }

        if let Some((x, y)) = start {
            let west = map
                .get(&((x - 1, y).into()))
                .unwrap_or(&Pipe::None)
                .connectors()
                .contains(&Direction::East);
            let east = map
                .get(&((x + 1, y).into()))
                .unwrap_or(&Pipe::None)
                .connectors()
                .contains(&Direction::West);
            let north = map
                .get(&((x, y - 1).into()))
                .unwrap_or(&Pipe::None)
                .connectors()
                .contains(&Direction::South);
            let south = map
                .get(&((x, y + 1).into()))
                .unwrap_or(&Pipe::None)
                .connectors()
                .contains(&Direction::North);

            let pipe = match (north, east, south, west) {
                (true, true, false, false) => Pipe::NorthEast,
                (false, true, true, false) => Pipe::SouthEast,
                (true, false, false, true) => Pipe::NorthWest,
                (false, false, true, true) => Pipe::SouthWest,
                (true, false, true, false) => Pipe::Vertical,
                (false, true, false, true) => Pipe::Horizontal,
                _ => panic!(),
            };

            map.insert((x, y).into(), pipe);
        }

        Self {
            map,
            start: start.unwrap().into(),
        }
    }
}

impl Game {
    pub fn solve(&self) -> Vec<Position> {
        let (mut next, mut direction) = self
            .map
            .get(&self.start)
            .unwrap()
            .connectors()
            .iter()
            .find_map(|dir| {
                let pos = self.start + dir.next();

                self.map
                    .get(&pos)
                    .map(|pipe| pipe.connectors())
                    .and_then(|dirs| {
                        if dirs.contains(&dir.counterpart()) {
                            dirs.into_iter()
                                .filter(|d| *d != dir.counterpart())
                                .next()
                                .map(|dir| (pos, dir))
                        } else {
                            None
                        }
                    })
            })
            .unwrap();

        let mut track = vec![self.start];

        while next != self.start {
            track.push(next);
            next = next + direction.next();
            let dirs = self.map.get(&next).unwrap().connectors();

            assert!(dirs.contains(&direction.counterpart()));
            direction = dirs
                .into_iter()
                .filter(|d| *d != direction.counterpart())
                .next()
                .unwrap();
        }

        track
    }

    pub fn area(&self) -> usize {
        let track: HashSet<_> = self.solve().into_iter().collect();
        let space: HashSet<_> = self.map.keys().cloned().into_iter().collect();

        space
            .difference(&track)
            .into_iter()
            .filter(|pos| {
                let mut inside = false;
                let mut bend: Option<&Pipe> = None;

                for y in (0..pos.y).rev() {
                    let pos: Position = (pos.x, y).into();
                    if !track.contains(&pos) {
                        continue;
                    }
                    let pipe = self.map.get(&pos).unwrap();

                    match (pipe, bend) {
                        (&Pipe::Horizontal, _) => {
                            inside = !inside;
                        }
                        (&Pipe::NorthEast, _) | (&Pipe::NorthWest, _) => {
                            bend = Some(pipe);
                        }
                        (&Pipe::SouthEast, Some(Pipe::NorthWest))
                        | (&Pipe::SouthWest, Some(Pipe::NorthEast)) => {
                            inside = !inside;
                            bend = None;
                        }
                        (&Pipe::SouthEast, Some(Pipe::NorthEast))
                        | (&Pipe::SouthWest, Some(Pipe::NorthWest)) => {
                            bend = None;
                        }
                        _ => {}
                    }
                }

                inside
            })
            .count()
    }
}

fn solve1(s: &str) -> usize {
    let game: Game = s.into();
    game.solve().len().div_floor(2)
}

fn solve2(s: &str) -> usize {
    let game: Game = s.into();
    game.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = include_str!("example-1");
    const EXAMPLE2: &'static str = include_str!("example-2");
    const EXAMPLE3: &'static str = include_str!("example-3");
    const EXAMPLE4: &'static str = include_str!("example-4");
    const EXAMPLE5: &'static str = include_str!("example-5");
    const EXAMPLE6: &'static str = include_str!("example-6");
    const EXAMPLE7: &'static str = include_str!("example-7");

    #[test]
    fn test_example1() {
        assert_eq!(solve1(EXAMPLE1), 4);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solve1(EXAMPLE2), 4);
    }

    #[test]
    fn test_example3() {
        assert_eq!(solve1(EXAMPLE3), 8);
    }

    #[test]
    fn test_example4() {
        assert_eq!(solve1(EXAMPLE4), 8);
    }

    #[test]
    fn test_example5() {
        assert_eq!(solve2(EXAMPLE5), 4);
    }

    #[test]
    fn test_example6() {
        assert_eq!(solve2(EXAMPLE6), 8);
    }

    #[test]
    fn test_example7() {
        assert_eq!(solve2(EXAMPLE7), 10);
    }
}
