use std::{ops::Add, marker::PhantomData};

use crate::prelude::*;
use bitvec::prelude::*;
use itertools::process_results;

day!(3);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Rucksack(BitArr!(for 52, in u64));

impl FromStr for Rucksack {
    type Err = WrongCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = BitArray::ZERO;
        for c in s.chars() {
            let idx: u8 = char_to_idx(c)?;
            bits.set(usize::from(idx), true);
        }
        Ok(Self(bits))
    }
}

impl Rucksack {
    pub const ONE: Self = Self(BitArray {
        _ord: PhantomData,
        data: [u64::MAX],
    });

    pub fn intersect(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
}

#[derive(Debug, Clone, Snafu)]
#[snafu(display("Invalid char ({c})"))]
struct WrongCharError {
    c: char,
}

impl From<char> for WrongCharError {
    fn from(c: char) -> Self {
        Self { c }
    }
}

fn char_to_idx(chr: char) -> Result<u8, WrongCharError> {
    let c: u8 = chr.try_into().map_err(|_| WrongCharError::from(chr))?;
    Ok(match c {
        b'a'..=b'z' => c - b'a',
        b'A'..=b'Z' => c - b'A' + b'z' - b'a' + 1,
        _ => return Err(WrongCharError::from(chr)),
    })
}

fn score(left: &str, right: &str) -> Result<u64, WrongCharError> {
    let left: Rucksack = left.parse()?;
    let right: Rucksack = right.parse()?;

    Ok(left
        .intersect(&right)
        .0
        .iter_ones()
        .map(|i| (i as u64) + 1)
        .sum())
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_string(DAY), "Failed opening input file");

    let part1: u64 = input
        .lines()
        .map(|s| s.trim().split_at(s.len() / 2))
        .map(|(l, r)| score(l, r))
        .fold_ok(0u64, Add::add)
        .whatever_context("Invalid input string")?;

    println!("Part 1: {part1}");

    let part2: u64 = process_results(
        input
            .lines()
            .map(|s| s.parse().whatever_context("Invalid input string")),
        |rucks| {
            rucks
                .chunks(3)
                .into_iter()
                .map(|triple| {
                    triple
                        .fold(Rucksack::ONE, |acc, v| acc.intersect(&v))
                        .0
                        .iter_ones()
                        .exactly_one()
                        .ok()
                        .whatever_context("Wrong number of badges")
                })
                .fold_ok(0u64, |acc, v| 1 + acc + v as u64)
        },
    )??;

    println!("Part 2: {part2}");

    Ok(())
}
