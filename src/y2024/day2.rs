use arrayvec::ArrayVec;

use super::Solution;
use crate::{prelude::*, utils::NomFail};

day!(run 2);

type Row = ArrayVec<u8, 8>;

struct Day2 {
    levels: Vec<Row>,
}

impl Solution for Day2 {
    const DAY: Day = DAY;

    fn parse(input: Vec<u8>) -> Result<Self, NomFail> {
        let levels = parse::parse(&input)?;
        Ok(Self { levels })
    }

    fn part1(&mut self) -> usize {
        self.levels.iter().filter(|l| is_safe1(l)).count()
    }

    fn part2(&mut self) -> usize {
        self.levels.iter().filter(|l| is_safe2(l)).count()
    }
}

fn is_safe1(levels: &[u8]) -> bool {
    let mut prev = levels[0];
    let inc = levels[1] > levels[0];
    for &n in &levels[1..] {
        // Check unsafe conditions
        if n == prev || (n > prev) != inc || n.abs_diff(prev) > 3 {
            return false;
        }
        prev = n;
    }
    true
}

fn is_safe2(levels: &Row) -> bool {
    // Test no removals
    if is_safe1(levels) {
        return true;
    }

    // Test each removal
    for i in 0..levels.len() {
        let mut l = levels.clone();
        l.remove(i);
        if is_safe1(&l) {
            return true;
        }
    }

    false
}

mod parse {
    use crate::utils::{parser::line, NomFail};
    use nom::{
        character::complete::{space1, u8},
        combinator::{all_consuming, opt},
        multi::{fold_many1, many1},
        sequence::preceded,
        Finish, IResult,
    };

    use super::Row;

    fn parse_row(input: &[u8]) -> IResult<&[u8], Row> {
        line(fold_many1(
            preceded(opt(space1), u8),
            Row::new,
            |mut vec, n| {
                vec.push(n);
                vec
            },
        ))(input)
    }

    pub fn parse(input: &[u8]) -> Result<Vec<Row>, NomFail> {
        Ok(all_consuming(many1(parse_row))(input).finish()?.1)
    }
}
