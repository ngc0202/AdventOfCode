
use crate::prelude::*;

day!(18);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let [plan1, plan2] = whatever!(parser::all_steps(&input), "Failed to parse input");

    println!("Part 1: {}", solve(&plan1));
    println!("Part 2: {}", solve(&plan2));

    Ok(())
}

fn solve(plan: &[Step]) -> u64 {
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut area = 0_i64;
    let mut borders = 0_u64;

    for step in plan {
        let (x0, y0) = (x, y);
        borders += u64::from(step.amt);
        let amt = i64::from(step.amt);
        match step.dir {
            Dir::Up => y -= amt,
            Dir::Down => y += amt,
            Dir::Left => x -= amt,
            Dir::Right => x += amt,
        }
        area += x0 * y - x * y0;
    }

    (area.abs_diff(0) + borders) / 2 + 1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    amt: u32,
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::space1,
        combinator::{map_parser, value},
        number::complete::hex_u32,
        sequence::{delimited, tuple},
        Finish, IResult, Parser,
    };

    use crate::utils::{parser::line, NomFail};

    use super::{Dir, Step};

    fn color(input: &[u8]) -> IResult<&[u8], Step> {
        tuple((
            map_parser(take(5_usize), hex_u32),
            alt((
                value(Dir::Right, tag("0")),
                value(Dir::Down, tag("1")),
                value(Dir::Left, tag("2")),
                value(Dir::Up, tag("3")),
            )),
        ))
        .map(|(amt, dir)| Step { dir, amt })
        .parse(input)
    }

    fn step(input: &[u8]) -> IResult<&[u8], [Step; 2]> {
        let dir = alt((
            value(Dir::Left, tag("L")),
            value(Dir::Right, tag("R")),
            value(Dir::Up, tag("U")),
            value(Dir::Down, tag("D")),
        ));

        let amt = delimited(space1, nom::character::complete::u32, space1);

        let color = delimited(tag("(#"), color, tag(")"));

        tuple((dir, amt, color))
            .map(|(dir, amt, color)| [Step { dir, amt }, color])
            .parse(input)
    }

    pub fn all_steps(mut input: &[u8]) -> Result<[Vec<Step>; 2], NomFail> {
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        while !input.is_empty() {
            match line(step)(input).finish() {
                Err(err) => return Err(err.into()),
                Ok((inp, [s1, s2])) => {
                    p1.push(s1);
                    p2.push(s2);
                    input = inp;
                }
            }
        }

        Ok([p1, p2])
    }
}
