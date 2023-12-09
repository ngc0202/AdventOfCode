use nom::Finish;

use crate::{prelude::*, utils::NomFail};

day!(4);

fn part1(input: &[u8]) -> Result<u64, NomFail> {
    let lines = input.split(|&b| b == b'\n');
    let mut sum = 0u64;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if let (_, n @ 1..) = parser::count_hits(line).finish()? {
            sum += 1 << (n - 1);
        }
    }

    Ok(sum)
}

fn part2(input: &[u8]) -> Result<u64, NomFail> {
    let mut cards = vec![0u64];
    let lines = input.split(|&b| b == b'\n');
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (_, matches) = parser::count_hits(line).finish()?;
        cards.push(u64::from(matches));
    }

    for idx in (1..cards.len()).rev() {
        let ([.., card], rest) = cards.split_at_mut(idx+1) else {
            panic!("Missing card");
        };

        *card = 1u64 + rest[..*card as usize].iter().sum::<u64>();
    }

    let sum = cards.into_iter().sum();

    Ok(sum)
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let part1 = whatever!(part1(&input), "Failed in part 1");
    println!("Part 1: {part1}");

    let part2 = whatever!(part2(&input), "Failed in part 2");
    println!("Part 2: {part2}");

    Ok(())
}

mod parser {
    use nom::{
        bytes::complete::{tag, take_while1},
        character::{
            complete::{space1, u8 as take_u8},
            is_digit,
        },
        error::Error,
        sequence::{preceded, Tuple},
        IResult,
    };

    pub fn count_hits(line: &[u8]) -> IResult<&[u8], u32> {
        // Remove card num
        let space1 = space1::<&[u8], Error<_>>;
        let (mut input, _) = (tag("Card"), space1, take_while1(is_digit), tag(":")).parse(line)?;

        // Take winners
        let mut pnum = preceded(space1, take_u8);
        let mut winners = [false; 100];
        while let Ok((rest, num)) = pnum(input) {
            input = rest;
            winners[usize::from(num)] = true;
        }

        (input, _) = (space1, tag("|")).parse(input)?;

        // Take have numbers
        let mut count = 0u32;
        while let Ok((rest, num)) = pnum(input) {
            input = rest;
            if winners[usize::from(num)] {
                count += 1;
            }
        }

        Ok((input, count))
    }
}
