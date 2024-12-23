use std::{
    collections::{hash_map::Entry, HashMap},
    ops::RangeInclusive,
};

use crate::utils::NomFail;

use super::Solution;

day!(run 22);

struct Day22 {
    nums: Vec<u64>,
}

impl<'i> Solution<'i> for Day22 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        Ok(Self {
            nums: parse::parse(input)?,
        })
    }

    fn part1(&mut self) -> u64 {
        self.nums.iter().map(|&s| nth_secret(s, 2000)).sum()
    }

    fn part2(&mut self) -> u64 {
        let diffs: Vec<_> = self.nums.iter().map(|&n| calc_diffs(n)).collect();
        let mut max = 0u64;
        const DS: RangeInclusive<i8> = -10i8..=10i8;

        for a in DS {
            for b in DS {
                for c in DS {
                    for d in DS {
                        let key = [a, b, c, d];
                        let c = banana_count(&key, &diffs);
                        if c > max {
                            max = c;
                        }
                    }
                }
            }
        }

        max
    }
}

fn banana_count(key: &[i8; 4], maps: &[HashMap<[i8; 4], u8>]) -> u64 {
    maps.iter()
        .map(|m| m.get(key).copied().unwrap_or(0) as u64)
        .sum()
}

fn calc_diffs(mut n: u64) -> HashMap<[i8; 4], u8> {
    let mut map = HashMap::new();
    let digit = |v| (v % 10) as i8;
    let mut last;
    let mut diffs = [0; 4];
    for i in 0..2000 {
        last = n;
        n = next_secret(n);
        let d = digit(n);
        let diff = d - digit(last);
        diffs[0] = diff;
        diffs.rotate_left(1);
        if i >= 3 {
            if let Entry::Vacant(slot) = map.entry(diffs) {
                slot.insert(d as u8);
            }
        }
    }
    map
}

fn nth_secret(mut start: u64, n: usize) -> u64 {
    for _ in 0..n {
        start = next_secret(start);
    }
    start
}

fn next_secret(mut s: u64) -> u64 {
    s ^= s * 64;
    s %= 16777216;
    s ^= s / 32;
    s %= 16777216;
    s ^= s * 2048;
    s %= 16777216;
    s
}

mod parse {
    use crate::utils::{parser::line, NomFail};
    use nom::{
        character::complete::u64, combinator::all_consuming, error::Error, multi::many1, Finish,
    };

    pub fn parse(input: &[u8]) -> Result<Vec<u64>, NomFail> {
        Ok(all_consuming(many1(line(u64::<_, Error<_>>)))(input)
            .finish()?
            .1)
    }
}
