use std::ops::Add;

use crate::{prelude::*, utils::NomFail};

day!(9);

fn predict(nums: &[i64]) -> Option<(i64, i64)> {
    let mut ends = Vec::with_capacity(nums.len());
    let mut cur = nums.to_vec();
    let mut last = Vec::with_capacity(cur.len());
    loop {
        ends.push((*cur.first()?, *cur.last()?));
        std::mem::swap(&mut cur, &mut last);
        cur.clear();

        let mut done = true;
        cur.extend(
            last.iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .inspect(|n| done &= *n == 0),
        );

        if done {
            break;
        }
    }

    ends.into_iter()
        .rev()
        .reduce(|acc, val| (val.0 - acc.0, acc.1 + val.1))
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let mut iter = parser::input_iter(&input);

    let Some((part2, part1)) = iter
        .map(|x| dbg!(predict(&x)))
        .fold_options((0i64, 0i64), tuple_add)
    else {
        whatever!("Failed to predict");
    };

    whatever!(
        iter.finish().finish().map_err(NomFail::from),
        "Failed to parse input"
    );

    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}

fn tuple_add<O, U, T: Add<U, Output = O>>(a: (T, T), b: (U, U)) -> (O, O) {
    (a.0 + b.0, a.1 + b.1)
}

mod parser {
    use nom::{character::complete::space1, error::Error, multi::separated_list1, IResult};

    use crate::utils::{eof_iterator, parser::line, EofParserIterator};

    pub fn input_iter<'i>(
        input: &'i [u8],
    ) -> EofParserIterator<
        &'i [u8],
        Error<&'i [u8]>,
        impl FnMut(&'i [u8]) -> IResult<&'i [u8], Vec<i64>>,
    > {
        eof_iterator(
            input,
            line(separated_list1(space1, nom::character::complete::i64)),
        )
    }
}
