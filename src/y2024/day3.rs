use crate::{prelude::*, utils::NomFail};

use super::Solution;

day!(run 3);

struct Day3 {
    p1: u64,
    p2: u64,
}

impl Solution for Day3 {
    const DAY: Day = DAY;

    fn parse(input: &mut Vec<u8>) -> Result<Self, NomFail> {
        let (p1, p2) = parse::solve(input)?;
        Ok(Self { p1, p2 })
    }

    fn part1(&mut self) -> u64 {
        self.p1
    }

    fn part2(&mut self) -> u64 {
        self.p2
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, u64},
        combinator::value,
        multi::fold_many1,
        sequence::{delimited, separated_pair},
        Finish, IResult, Parser,
    };

    use crate::utils::{parser::retry, NomFail};

    #[derive(Copy, Clone)]
    enum Insn {
        Enable(bool),
        Mul(u64),
    }

    fn mul(input: &[u8]) -> IResult<&[u8], (u64, u64)> {
        delimited(tag("mul("), separated_pair(u64, char(','), u64), char(')'))(input)
    }

    fn insn(input: &[u8]) -> IResult<&[u8], Insn> {
        alt((
            value(Insn::Enable(true), tag("do()")),
            value(Insn::Enable(false), tag("don't()")),
            mul.map(|(a, b)| Insn::Mul(a * b)),
        ))(input)
    }

    pub fn solve(input: &[u8]) -> Result<(u64, u64), NomFail> {
        let (_, (_, p1, p2)) = fold_many1(
            retry(insn),
            || (true, 0, 0),
            |(en, p1, p2), insn| match insn {
                Insn::Enable(e) => (e, p1, p2),
                Insn::Mul(n) if en => (en, p1 + n, p2 + n),
                Insn::Mul(n) => (en, p1 + n, p2),
            },
        )(input)
        .finish()?;

        Ok((p1, p2))
    }
}
