use std::{ops::{Range, RangeFrom, RangeTo}, mem::MaybeUninit};

    use nom::{
        branch::alt, character::complete::line_ending, combinator::{eof, recognize}, error::ParseError, sequence::terminated, Compare, IResult, InputIter, InputLength, InputTake, Offset, Parser, Slice
    };

    /// Matches the given parser followed by either eof or a line ending.
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

    /// Runs `parser` on the input `N` times, storing the result in an array.
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

    /// Retries `parser` until it succeeds or eof, taking a byte off the input on error.
    pub fn retry<P, I, E, O>(mut parser: P) -> impl FnMut(I) -> IResult<I, O, E>
    where
        I: InputLength + InputTake + Clone,
        E: ParseError<I>,
        P: Parser<I, O, E>,
    {
        move |mut input: I| {
            while input.input_len() > 0 {
                match parser.parse(input.clone()) {
                    Ok(v) => return Ok(v),
                    Err(nom::Err::Error(_)) => (input, _) = input.take_split(1),
                    Err(err) => return Err(err),
                }
            }

            Err(nom::Err::Error(E::from_error_kind(input, nom::error::ErrorKind::Eof)))
        }
    }