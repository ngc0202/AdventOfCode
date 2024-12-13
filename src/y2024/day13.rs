use super::Solution;
use crate::utils::NomFail;

day!(run 13);

struct Day13 {
    games: Vec<Game>,
}

struct Game {
    a: [u64; 2],
    b: [u64; 2],
    prize: [u64; 2],
}

const ACOST: u64 = 3;
const BCOST: u64 = 1;
const P2DIFF: u64 = 10_000_000_000_000;

impl<'i> Solution<'i> for Day13 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        Ok(Self {
            games: parse::parse(input)?,
        })
    }

    fn part1(&mut self) -> u64 {
        self.games.iter().filter_map(|g| g.cost()).sum()
    }

    fn part2(&mut self) -> u64 {
        self.games
            .iter_mut()
            .filter_map(|g| {
                g.prize = g.prize.map(|n| n + P2DIFF);
                g.cost()
            })
            .sum()
    }
}

impl Game {
    /// Returns the cost to win, if possible
    pub fn cost(&self) -> Option<u64> {
        let bc = self.b[0] * self.a[1];
        let ad = self.a[0] * self.b[1];
        let bf = self.b[0] * self.prize[1];
        let de = self.b[1] * self.prize[0];

        if (ad > bc) != (de > bf) {
            return None;
        }

        let x = divide(bf.abs_diff(de), bc.abs_diff(ad))?;
        let y = divide(self.prize[0].checked_sub(self.a[0] * x)?, self.b[0])?;

        Some(x * ACOST + y * BCOST)
    }
}

fn divide(a: u64, b: u64) -> Option<u64> {
    a.checked_div(b).filter(|_| a % b == 0)
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, u64},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::{preceded, separated_pair, tuple},
        Finish, IResult, Parser,
    };

    use crate::utils::{parser::line, NomFail};

    use super::Game;

    fn button(label: u8) -> impl FnMut(&[u8]) -> IResult<&[u8], (u64, u64)> {
        move |input: &[u8]| {
            line(preceded(
                tuple((tag("Button "), tag([label]), tag(": X+"))),
                separated_pair(u64, tag(", Y+"), u64),
            ))(input)
        }
    }

    fn prize(input: &[u8]) -> IResult<&[u8], (u64, u64)> {
        line(preceded(
            tag("Prize: X="),
            separated_pair(u64, tag(", Y="), u64),
        ))(input)
    }

    fn game(input: &[u8]) -> IResult<&[u8], Game> {
        tuple((button(b'A'), button(b'B'), prize))
            .map(|(a, b, p)| Game {
                a: a.into(),
                b: b.into(),
                prize: p.into(),
            })
            .parse(input)
    }

    pub fn parse(input: &[u8]) -> Result<Vec<Game>, NomFail> {
        Ok(all_consuming(separated_list1(line_ending, game))(input)
            .finish()?
            .1)
    }
}
