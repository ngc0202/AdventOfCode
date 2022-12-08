use std::{cmp::Ordering, error::Error, fmt::Display, ops::Not};

use itertools::process_results;

use crate::prelude::*;

day!(2);

#[derive(Debug)]
enum ParseError {
    WrongNumber,
    WrongLetter(char),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLetter(c) => write!(f, "Wrong Letter ({c})"),
            Self::WrongNumber => f.write_str("Wrong Number of Letters"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        use RPS::*;
        match (self, other) {
            (Rock, Rock) => Ordering::Equal,
            (Paper, Paper) => Ordering::Equal,
            (Scissors, Scissors) => Ordering::Equal,
            (Rock, Scissors) => Ordering::Greater,
            (Paper, Rock) => Ordering::Greater,
            (Scissors, Paper) => Ordering::Greater,
            (Scissors, Rock) => Ordering::Less,
            (Rock, Paper) => Ordering::Less,
            (Paper, Scissors) => Ordering::Less,
        }
    }
}

impl FromStr for RPS {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().exactly_one() {
            Ok(c) => c.try_into(),
            Err(_) => Err(ParseError::WrongNumber),
        }
    }
}

impl TryFrom<char> for RPS {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'r' | 'R' => RPS::Rock,
            'p' | 'P' => RPS::Paper,
            's' | 'S' => RPS::Scissors,
            _ => return Err(ParseError::WrongLetter(c)),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum XYZ {
    X,
    Y,
    Z,
}

impl FromStr for XYZ {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().exactly_one() {
            Ok(c) => c.try_into(),
            Err(_) => Err(ParseError::WrongNumber),
        }
    }
}

impl TryFrom<char> for XYZ {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'x' | 'X' => XYZ::X,
            'y' | 'Y' => XYZ::Y,
            'z' | 'Z' => XYZ::Z,
            _ => return Err(ParseError::WrongLetter(c)),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ABC {
    A,
    B,
    C,
}

impl FromStr for ABC {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().exactly_one() {
            Ok(c) => c.try_into(),
            Err(_) => Err(ParseError::WrongNumber),
        }
    }
}

impl TryFrom<char> for ABC {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'a' | 'A' => ABC::A,
            'b' | 'B' => ABC::B,
            'c' | 'C' => ABC::C,
            _ => return Err(ParseError::WrongLetter(c)),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct InputParseError;

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid input")
    }
}

impl Error for InputParseError {}

fn score1(abc: ABC, xyz: XYZ) -> u8 {
    let a = match abc {
        ABC::A => RPS::Rock,
        ABC::B => RPS::Paper,
        ABC::C => RPS::Scissors
    };

    let b = match xyz {
        XYZ::X => RPS::Rock,
        XYZ::Y => RPS::Paper,
        XYZ::Z => RPS::Scissors,
    };

    (b as u8) + match b.cmp(&a) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

fn score2(abc: ABC, xyz: XYZ) -> u8 {
    let a = match abc {
        ABC::A => RPS::Rock,
        ABC::B => RPS::Paper,
        ABC::C => RPS::Scissors
    };

    let b = match xyz {
        XYZ::X => Outcome::Loss,
        XYZ::Y => Outcome::Draw,
        XYZ::Z => Outcome::Win,
    };

    (b as u8) + match (a, b) {
        (a, Outcome::Draw) => a,
        (RPS::Rock, Outcome::Loss) => RPS::Scissors,
        (RPS::Rock, Outcome::Win) => RPS::Paper,
        (RPS::Paper, Outcome::Loss) => RPS::Rock,
        (RPS::Paper, Outcome::Win) => RPS::Scissors,
        (RPS::Scissors, Outcome::Loss) => RPS::Paper,
        (RPS::Scissors, Outcome::Win) => RPS::Rock,
    } as u8
}

pub fn run() -> GenResult {
    let (part1, part2) = process_results(
        load_input_string(DAY)?
            .split('\n')
            .filter_map(|l| { let l = l.trim(); l.is_empty().not().then_some(l) })
            .map(|line| line.split_once(' ').ok_or(InputParseError)),
        |spl| -> Result<_, ParseError> {
            process_results(
                spl.map(|(l, r)| Ok((l.trim().parse()?, r.trim().parse()?))),
                |it| it.fold((0u64, 0u64), |(acc1, acc2), (rps, xyz)| (acc1 + u64::from(score1(rps, xyz)), (acc2 + u64::from(score2(rps, xyz)))))
            )
        },
    )??;

    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}
