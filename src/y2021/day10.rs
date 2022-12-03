
use crate::prelude::*;

use std::error::Error;
use std::fmt;
use itertools::process_results;

day!(10);

#[derive(Clone, Debug)]
struct NotBracketError;

impl Error for NotBracketError {}

impl fmt::Display for NotBracketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotBracketError")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle
}

impl Bracket {
    pub fn value_error(&self) -> u16 {
        match *self {
            Bracket::Round  => 3,
            Bracket::Square => 57,
            Bracket::Curly  => 1197,
            Bracket::Angle  => 25137
        }
    }

    pub fn value_completion(&self) -> u8 {
        match *self {
            Bracket::Round  => 1,
            Bracket::Square => 2,
            Bracket::Curly  => 3,
            Bracket::Angle  => 4
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = NotBracketError;

    fn try_from(c: char) -> Result<Bracket, NotBracketError> {
        match c {
            '(' | ')' => Ok(Bracket::Round),
            '[' | ']' => Ok(Bracket::Square),
            '{' | '}' => Ok(Bracket::Curly),
            '<' | '>' => Ok(Bracket::Angle),
            _ => Err(NotBracketError)
        }
    }
}

#[derive(Clone, Debug)]
struct BracketStack {
    stack: Vec<Bracket>,
}

impl BracketStack {
    pub fn new() -> Self {
        BracketStack { stack: Vec::new() }
    }

    pub fn feed(&mut self, c: char) -> Result<(), Bracket> {
        let b = c.try_into().unwrap();
        if let '(' | '[' | '{' | '<' = c {
            Ok(self.push(b))
        } else {
            self.pop(b)
        }
    }

    fn push(&mut self, b: Bracket) {
        self.stack.push(b);
    }

    fn pop(&mut self, b: Bracket) -> Result<(), Bracket> {
        self.stack
            .pop()
            .and_then(|p|
                (p==b).then(|| ()))
            .ok_or(b)
    }
}

// None = no error, Some = error value
fn find_error(s: &str) -> Option<u16> {
    let mut stack = BracketStack::new();
    s.chars()
        .try_for_each(|c| stack.feed(c))
        .map_err(|b| b.value_error())
        .err()
}

pub fn run() -> GenResult {
    let part1 = load_input(DAY)?
        .lines()
        .map_ok(|ref s| find_error(s))
        .fold_ok(0u64, |acc, v| v.map(u64::from).unwrap_or(0) + acc)?;

    println!("Part 1: {}", part1);

    let mut part2_vec: Vec<u64> = process_results(
        load_input(DAY)?
        .lines(), |it| it
        .filter_map(|ref s| s.chars().
            try_fold(BracketStack::new(), |mut stack, c| -> Result<BracketStack, Bracket> {
                stack.feed(c)?;
                Ok(stack)
            }).ok())
        .map(|BracketStack { stack }| stack.iter().rev().fold(0u64, |acc, b| acc * 5 + u64::from(b.value_completion())))
        .collect()
    )?;

    part2_vec.sort();
    dbg!(&part2_vec);

    let &part2 = part2_vec.get(part2_vec.len() / 2 ).unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}