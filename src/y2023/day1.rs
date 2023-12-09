use nom::character::is_digit;

use crate::prelude::*;

day!(1);

pub fn part1_line(input: &[u8]) -> Result<u8, Whatever> {
    let mut iter = input.iter().filter(|&&b| is_digit(b)).map(|&d| d - b'0');

    let first = iter.next().whatever_context("No digit in input")?;

    let last = iter.next_back().unwrap_or(first);

    Ok(first * 10 + last)
}

pub fn part1(input: &[u8]) -> Result<u64, Whatever> {
    let line_iter = input
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(part1_line);

    itertools::process_results(line_iter, |it| it.map(u64::from).sum())
}

pub fn part2(input: &[u8]) -> Result<u64, Whatever> {
    let mut sum = 0u64;
    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }

        let num = parser::parse_line(line)?;
        sum += u64::from(num);
    }

    Ok(sum)
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::u8 as take_u8,
        combinator::{map_parser, value},
        IResult,
    };
    use snafu::{OptionExt, Whatever};

    fn text_digit(input: &[u8]) -> IResult<&[u8], u8> {
        alt((
            value(0, tag("zero")),
            value(1, tag("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
        ))(input)
    }

    pub fn any_digit(input: &[u8]) -> IResult<&[u8], u8> {
        alt((map_parser(take(1u8), take_u8), text_digit))(input)
    }

    pub fn parse_line(input: &[u8]) -> Result<u8, Whatever> {
        let first = {
            let mut input = input;
            loop {
                if let Ok((_, num)) = any_digit(input) {
                    break num;
                }

                input = input.get(1..).whatever_context("No digit in input")?;
            }
        };

        let last = 'last: {
            for i in (0..input.len()).rev() {
                if let Ok((_, num)) = any_digit(&input[i..]) {
                    break 'last num;
                }
            }

            first
        };

        Ok(first * 10 + last)
    }
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed opening input file");

    println!("Day 1: {}", part1(&input)?);
    println!("Day 2: {}", part2(&input)?);

    Ok(())
}
