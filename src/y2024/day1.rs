use crate::utils::NomFail;

use super::Solution;

day!(run 1);

struct Day1 {
    v1: Vec<u64>,
    v2: Vec<u64>,
}

impl<'i> Solution<'i> for Day1 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        let [v1, v2] = parse::parse(input)?;
        Ok(Self { v1, v2 })
    }

    fn part1(&mut self) -> u64 {
        self.v1
            .iter()
            .zip(self.v2.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum()
    }

    fn part2(&mut self) -> u64 {
        let mut v2 = &*self.v2;
        self.v1
            .iter()
            .map(|&v| {
                let idx = v2.partition_point(|&n| n < v);
                (_, v2) = v2.split_at(idx);
                let c = v2.iter().take_while(|&&n| n == v).count();
                v * (c as u64)
            })
            .sum()
    }
}

mod parse {
    use nom::{
        character::complete::{space1, u64},
        sequence::separated_pair,
        Finish, IResult,
    };

    use crate::utils::{eof_iterator, parser::line, NomFail};

    fn row(input: &[u8]) -> IResult<&[u8], (u64, u64)> {
        line(separated_pair(u64, space1, u64))(input)
    }

    pub fn parse(input: &[u8]) -> Result<[Vec<u64>; 2], NomFail> {
        // Parse vecs
        let mut it = eof_iterator(input, row);
        let (mut v1, mut v2): (Vec<u64>, Vec<u64>) = it.by_ref().unzip();
        it.finish().finish()?;

        // Sort vecs
        v1.sort_unstable();
        v2.sort_unstable();

        Ok([v1, v2])
    }
}
