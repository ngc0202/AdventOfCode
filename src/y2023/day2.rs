use crate::prelude::*;

day!(2);

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::u8 as take_u8,
        character::complete::{newline, space1},
        combinator::{all_consuming, eof, iterator, recognize, value},
        error::Error,
        multi::fold_many1,
        sequence::{delimited, separated_pair, terminated},
        Finish, IResult,
    };

    use super::{MAX_BLUE, MAX_GREEN, MAX_RED};

    #[derive(Copy, Clone)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    impl Color {
        pub const fn max(&self) -> u8 {
            match self {
                Color::Red => MAX_RED,
                Color::Green => MAX_GREEN,
                Color::Blue => MAX_BLUE,
            }
        }
    }

    struct Draw {
        amt: u8,
        color: Color,
    }

    impl Draw {
        /// Is valid under part 1
        pub const fn is_valid(&self) -> bool {
            self.amt <= self.color.max()
        }
    }

    fn parse_draw(input: &[u8]) -> IResult<&[u8], Draw> {
        let pcolor = alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        ));

        let (rest, (amt, color)) = terminated(
            separated_pair(take_u8, space1, pcolor),
            alt((eof, recognize(newline), tag(", "), tag("; "))),
        )(input)?;

        let draw = Draw { amt, color };

        Ok((rest, draw))
    }

    fn calculate_line(input: &[u8]) -> IResult<&[u8], (u8, u64)> {
        let (input, game) = delimited(tag("Game "), take_u8, tag(": "))(input)?;

        let mut draw_iter = iterator(input, parse_draw);

        // Part 1 state
        let mut valid = true;

        // Part 2 state
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for draw in &mut draw_iter {
            // Part 1
            if valid && !draw.is_valid() {
                valid = false;
            }

            // Part 2
            match draw.color {
                Color::Red => max_red = max_red.max(draw.amt),
                Color::Green => max_green = max_green.max(draw.amt),
                Color::Blue => max_blue = max_blue.max(draw.amt),
            }
        }

        let (input, ()) = draw_iter.finish()?;

        let id = valid.then_some(game).unwrap_or_default();
        let power = u64::from(max_red) * u64::from(max_green) * u64::from(max_blue);

        Ok((input, (id, power)))
    }

    pub fn calculate_input(input: &[u8]) -> Result<(u64, u64), Error<&[u8]>> {
        all_consuming(fold_many1(
            calculate_line,
            || (0u64, 0u64),
            |acc, val| (acc.0 + u64::from(val.0), acc.1 + val.1),
        ))(input)
        .finish()
        .map(|(_, v)| v)
    }
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed opening input file");

    let start = std::time::Instant::now();
    let (part1, part2) = whatever!(
        parser::calculate_input(&input).map_err(|e| format!(
            "ParseError({:?}): {:?}",
            e.code,
            String::from_utf8_lossy(e.input)
        )),
        "Failed to parse input"
    );
    println!("Time: {:?}", start.elapsed());

    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}
