use smallvec::SmallVec;
use crate::{prelude::*, utils::NomFail};
use super::Solution;

day!(run 7);

struct Eqn {
    result: u64,
    nums: SmallVec<[u16; 12]>,
}

struct Day7 {
    vec: Vec<Eqn>,
}

impl Solution for Day7 {
    const DAY: Day = DAY;

    fn parse(input: &mut Vec<u8>) -> Result<Self, NomFail> {
        let vec = parse::parse(input)?;
        Ok(Self { vec })
    }

    fn part1(&mut self) -> u64 {
        self.solve(false)
    }

    fn part2(&mut self) -> u64 {
        self.solve(true)
    }
}

impl Day7 {
    fn solve(&self, concat: bool) -> u64 {
        self.vec
            .iter()
            .filter_map(|e| e.is_solvable(concat).then_some(e.result))
            .sum()
    }
}

impl Eqn {
    fn is_solvable(&self, concat: bool) -> bool {
        let it = self.nums.iter().copied();
        solve(self.result, 0u64, concat, it)
    }
}

fn solve<I>(res: u64, cur: u64, use_concat: bool, mut nums: I) -> bool
where
    I: Clone + Iterator<Item = u16>,
{
    // Pull next number
    let Some(num) = nums.next().map(u64::from) else {
        // Reached the end, check target
        return cur == res
    };

    // Process addition
    let add = cur.checked_add(num).is_some_and(|n| solve(res, n, use_concat, nums.clone()));
    if add {
        return true
    }

    // Process multiplication
    let mul = cur.checked_mul(num).is_some_and(|n| solve(res, n, use_concat, nums.clone()));
    if mul {
        return true
    }

    // Process concatenation
    if use_concat {
        concat(cur, num).is_some_and(|n| solve(res, n, use_concat, nums))
    } else {
        false
    }
}

/// Checked concatenation of two numbers, concat(12, 345) == Some(12345)
fn concat(a: u64, b: u64) -> Option<u64> {
    let blen = b.checked_ilog10().unwrap_or(0) + 1;
    10u64.pow(blen).checked_mul(a)?.checked_add(b)
}


mod parse {
    use nom::{bytes::complete::tag, character::complete::{space1, u16, u64}, combinator::all_consuming, multi::many1, sequence::{preceded, terminated, tuple}, Finish, IResult};
    use smallvec::SmallVec;
    use crate::utils::{parser::{for_each, line}, NomFail};
    use super::Eqn;

    fn eqn(input: &[u8]) -> IResult<&[u8], Eqn> {
        let mut nums = SmallVec::new();
        let (i, result) = line(terminated(u64, tuple((tag(":"), for_each(preceded(space1, u16), |n| nums.push(n))))))(input)?;
        Ok((i, Eqn { result, nums }))
    }

    pub fn parse(input: &[u8]) -> Result<Vec<Eqn>, NomFail> {
        Ok(all_consuming(many1(eqn))(input).finish()?.1)
    }
}