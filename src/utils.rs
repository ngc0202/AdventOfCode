#[allow(dead_code)]
mod grid;
pub use grid::Grid;

mod wrap;
use itertools::{process_results, ProcessResults};
use snafu::Snafu;
pub use wrap::WrapUsize;

mod eof_iterator;
pub use eof_iterator::{eof_iterator, EofParserIterator};

use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
    path::PathBuf,
};

use crate::get_small;

pub fn load_input(Day { day, year }: Day) -> io::Result<BufReader<File>> {
    let filename = if get_small() {
        format!("input{day:02}_small.txt")
    } else {
        format!("input{day:02}.txt")
    };

    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(format!("{}", year));
    path.push(filename);
    println!("Reading from {}", path.display());
    File::open(&path).map(BufReader::new)
}

pub fn load_input_string(day: Day) -> io::Result<String> {
    let mut input = String::new();
    load_input(day)?.read_to_string(&mut input)?;
    Ok(input)
}

pub fn load_input_bytes(day: Day) -> io::Result<Vec<u8>> {
    let mut input = Vec::new();
    load_input(day)?.read_to_end(&mut input)?;
    Ok(input)
}

pub fn process_inputs<F, R>(day: Day, f: F) -> io::Result<R>
where
    F: FnOnce(ProcessResults<'_, Lines<BufReader<File>>, io::Error>) -> R,
{
    process_results(load_input(day)?.lines(), f)
}

#[derive(Debug)]
pub struct NoInputError;

impl fmt::Display for NoInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No input provided")
    }
}

impl std::error::Error for NoInputError {}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Day {
    pub day: u8,
    pub year: u16,
}

#[allow(unused)]
pub fn dbg_dmp<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a [u8]) -> nom::IResult<&'a [u8], O, E>
where
    F: FnMut(&'a [u8]) -> nom::IResult<&'a [u8], O, E>,
{
    use nom::HexDisplay;
    move |i: &'a [u8]| match f(i) {
        Err(e) => {
            println!("{}: Error({:?}) at:\n{}", context, e, i.to_hex(8));
            Err(e)
        }
        a => a,
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Parse error ({code:?}): {input:?}"))]
pub struct NomFail {
    code: nom::error::ErrorKind,
    input: String,
}

impl<I: nom::AsBytes> From<nom::error::Error<I>> for NomFail {
    fn from(inner: nom::error::Error<I>) -> Self {
        Self {
            code: inner.code,
            input: String::from_utf8_lossy(inner.input.as_bytes()).into_owned(),
        }
    }
}

pub mod parser {
    use std::{ops::{Range, RangeFrom, RangeTo}, mem::MaybeUninit};

    use nom::{
        branch::alt,
        character::complete::line_ending,
        combinator::{eof, recognize},
        error::ParseError,
        sequence::terminated,
        Compare, IResult, InputIter, InputLength, Offset, Parser, Slice,
    };

    pub fn line<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
    where
        I: Clone
            + Offset
            + InputLength
            + InputIter
            + Slice<RangeFrom<usize>>
            + Slice<Range<usize>>
            + Slice<RangeTo<usize>>
            + Compare<&'static str>,
        E: ParseError<I>,
        F: Parser<I, O, E>,
    {
        terminated(parser, alt((eof, recognize(line_ending))))
    }

    pub fn many_array<const N: usize, I, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, [O; N], E>
    where
        E: ParseError<I>,
        F: Parser<I, O, E>,
        O: Copy,
    {
        move |mut input: I| {
            let mut arr = std::array::from_fn(|_| MaybeUninit::uninit());
            for item in &mut arr {
                let val;
                (input, val) = parser.parse(input)?;
                item.write(val);
            }
            Ok((input, arr.map(|u| unsafe { u.assume_init() } )))
        }
    }
}
