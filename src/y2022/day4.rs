use std::num::ParseIntError;

use crate::prelude::*;

day!(4);

type Section = std::ops::RangeInclusive<u8>;

#[derive(Debug, Snafu)]
enum LineParseError {
    #[snafu(display("Failed to parse int ranges"))]
    IntParse { source: ParseIntError },
    #[snafu(display("Missing comma in input"))]
    MissingComma,
    #[snafu(display("Missing hyphen in range"))]
    MissingHyphen,
}

fn parse_line(line: &str) -> Result<(Section, Section), LineParseError> {
    let (one, two) = line.split_once(',').ok_or(LineParseError::MissingComma)?;

    fn parse_section(s: &str) -> Result<Section, LineParseError> {
        let (start, end) = s.split_once('-').ok_or(LineParseError::MissingHyphen)?;
        Ok(Section::new(
            start.parse().context(IntParseSnafu)?,
            end.parse().context(IntParseSnafu)?,
        ))
    }

    Ok((parse_section(one)?, parse_section(two)?))
}

fn sections_contained(a: &Section, b: &Section) -> bool {
    let [small, big] = {
        let mut arr = [a, b];
        arr.sort_by_key(|s| s.len());
        arr
    };

    (big.start() <= small.start()) && (big.end() >= small.end())
}

fn sections_overlap(a: &Section, b: &Section) -> bool {
    let [small, big] = {
        let mut arr = [a, b];
        arr.sort_by_key(|s| s.len());
        arr
    };

    big.contains(small.start()) || big.contains(small.end())
}

pub fn run() -> Result<(), Whatever> {
    let (part1, part2) = whatever!(load_input_string(DAY), "Failed reading input file")
        .lines()
        .map(parse_line)
        .fold_ok((0u64, 0u64), |(acc1, acc2), (ref s1, ref s2)| {
            (
                acc1 + sections_contained(s1, s2) as u64,
                acc2 + sections_overlap(s1, s2) as u64,
            )
        })
        .whatever_context("Failed parsing input")?;

    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}
