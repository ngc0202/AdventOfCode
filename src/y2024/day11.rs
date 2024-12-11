use nom::Finish;

use super::Solution;
use crate::utils::{Day, NomFail};
use snafu::Whatever;

day!(run 11);

type Memo = std::cell::RefCell<std::collections::HashMap<(u64, usize), u64>>;

struct Day11 {
    nums: Vec<u64>,
    memo: Memo,
}

impl Solution for Day11 {
    const DAY: Day = DAY;

    fn parse(input: Vec<u8>) -> Result<Self, NomFail> {
        Ok(Self {
            nums: parse::parse(&input).finish()?.1,
            memo: Memo::default(),
        })
    }

    fn part1(&mut self) -> u64 {
        self.count_stones(25)
    }

    fn part2(&mut self) -> u64 {
        self.count_stones(75)
    }
}

impl Day11 {
    pub fn count_stones(&self, blinks: usize) -> u64 {
        self.nums
            .iter()
            .map(|&n| count_stones(n, blinks, &self.memo))
            .sum()
    }
}

fn count_stones(n: u64, blinks: usize, memo: &Memo) -> u64 {
    // Stop at zero blinks
    let Some(blinks) = blinks.checked_sub(1) else {
        return 1;
    };

    // Retrieve from cache if available
    if let Some(&c) = memo.borrow().get(&(n, blinks)) {
        return c;
    }

    // Calculate new number
    let c = if n == 0 {
        count_stones(1, blinks, memo)
    } else if let Some([a, b]) = split(n) {
        count_stones(a, blinks, memo) + count_stones(b, blinks, memo)
    } else {
        count_stones(n * 2024, blinks, memo)
    };

    // Add value to cache
    memo.borrow_mut().insert((n, blinks), c);
    c
}

/// Splits n in half if there's an even number of digits
fn split(n: u64) -> Option<[u64; 2]> {
    let len = n.checked_ilog10().filter(|p| p % 2 == 1)? + 1;
    let pow = 10u64.pow(len / 2);
    Some([n / pow, n % pow])
}

mod parse {
    use crate::utils::parser::line;
    use nom::{
        character::complete::{space1, u64},
        combinator::all_consuming,
        multi::separated_list1,
        IResult,
    };

    pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<u64>> {
        all_consuming(line(separated_list1(space1, u64)))(input)
    }
}
