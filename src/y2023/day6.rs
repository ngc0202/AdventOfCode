use crate::{prelude::*, utils::NomFail};

day!(6);

fn count_ways(time: u64, dist: u64) -> u64 {
    let ftime = time as f64;
    let fdist = dist as f64;

    let x = (ftime * ftime - 4. * fdist).sqrt();
    let low = (ftime - x) / 2. + 1.;
    let high = (ftime + x) / 2. - 1.;

    ((low.floor() as u64)..=(high.ceil() as u64)).size_hint().0 as u64
}

fn part1(input: &[u8]) -> Result<u64, NomFail> {
    Ok(parser::part1(input).finish()?.1)
}

fn part2(input: &[u8]) -> Result<u64, NomFail> {
    Ok(parser::part2(input).finish()?.1)
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let part1 = whatever!(part1(&input), "Failed part 1");
    println!("Part 1: {part1}");

    let part2 = whatever!(part2(&input), "Failed part 2");
    println!("Part 2: {part2}");

    Ok(())
}

mod parser {
    use nom::{
        bytes::complete::{tag, take_till},
        character::{
            complete::{line_ending, space0, space1},
            is_digit, is_newline,
        },
        combinator::{all_consuming, verify},
        multi::fold_many1,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use crate::y2023::day6::count_ways;

    pub fn part1(input: &[u8]) -> IResult<&[u8], u64> {
        let (rest, (mut stime, mut sdist)) = separated_pair(
            preceded(tag("Time:"), take_till(is_newline)),
            line_ending,
            preceded(tag("Distance:"), take_till(is_newline)),
        )(input)?;

        let mut take_u64 = preceded(space1, nom::character::complete::u64);

        let mut prod = 1u64;
        while !(stime.is_empty() || sdist.is_empty()) {
            let (time, dist);
            (stime, time) = take_u64(stime)?;
            (sdist, dist) = take_u64(sdist)?;
            prod *= count_ways(time, dist);
        }

        Ok((rest, prod))
    }

    pub fn part2(input: &[u8]) -> IResult<&[u8], u64> {
        let (rest, (stime, sdist)) = separated_pair(
            preceded(tag("Time:"), take_till(is_newline)),
            line_ending,
            preceded(tag("Distance:"), take_till(is_newline)),
        )(input)?;

        let mut make_num = all_consuming(fold_many1(
            preceded(space0, verify(nom::number::complete::u8, |&b| is_digit(b))),
            || 0u64,
            |acc, b| acc * 10 + u64::from(b - b'0'),
        ));
        let (_, time) = make_num(stime)?;
        let (_, dist) = make_num(sdist)?;

        let ways = count_ways(time, dist);
        Ok((rest, ways))
    }
}
