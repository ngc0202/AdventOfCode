use std::ops::Range;

use nom::Finish;
use smallvec::SmallVec;

use crate::{prelude::*, utils::NomFail};

day!(5);

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

    let start = std::time::Instant::now();
    let part2 = whatever!(part2(&input), "Failed part 2");
    let dur = start.elapsed();
    println!("Part 2: {part2} in {dur:?}");

    Ok(())
}

mod parser {
    use std::ops::Range;

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::{
            complete::{line_ending, multispace0, multispace1, space1, u64 as take_u64},
            is_alphabetic,
        },
        combinator::{eof, iterator, map, recognize, ParserIterator},
        error::Error,
        multi::many1,
        sequence::{delimited, preceded, terminated, tuple, separated_pair},
        IResult
    };

    use super::{Mapping, Mappable};

    pub fn part1(input: &[u8]) -> IResult<&[u8], u64> {
        // Parse initial seeds
        let (mut input, mut seeds) = delimited(
            tag("seeds:"),
            many1(preceded(space1, take_u64)),
            multispace1,
        )(input)?;

        // Process maps
        (input, ()) = process_maps(input, &mut seeds)?;

        // Return minimum
        let &min = seeds.iter().min().unwrap();
        Ok((input, min))
    }

    pub fn part2(input: &[u8]) -> IResult<&[u8], u64> {
        // Parse initial seeds
        let (mut input, mut seeds) = delimited(
            tag("seeds:"),
            collect_ranges,
            multispace1,
        )(input)?;

        // Process maps
        (input, ()) = process_maps(input, &mut seeds)?;

        // Return minimum
        let min = seeds.iter().map(|r| r.start).min().unwrap();
        Ok((input, min))
    }

    fn process_maps<'i, T: Mappable>(mut input: &'i [u8], seeds: &mut Vec<T>) -> IResult<&'i [u8], ()> {
        let mut maps = Vec::new();
        
        // Process maps
        while !input.is_empty() {
            // Throw away map name
            (input, _) = map_name(input)?;

            // Collect maps
            let mut it = map_iter(input);
            maps.clear();
            maps.extend(&mut it);
            (input, ()) = it.finish()?;

            // Apply maps
            let mut idx = 0;
            while idx < seeds.len() {
                for map in &maps {
                    if let Some(addl) = seeds[idx].apply(map) {
                        seeds.extend(addl);
                        break;
                    }
                }
                idx += 1;
            }

            // Throw away trailing whitespace
            (input, _) = multispace0(input)?;
        }

        // Ensure input has been consumed
        (input, _) = eof(input)?;

        Ok((input, ()))
    }

    fn map_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
        recognize(tuple((
            take_while1(|b| b == b'-' || is_alphabetic(b)),
            tag(" map:"),
            line_ending,
        )))(input)
    }

    fn map_iter<'a>(
        input: &'a [u8],
    ) -> ParserIterator<&'a [u8], Error<&'a [u8]>, impl FnMut(&'a [u8]) -> IResult<&'a [u8], Mapping>>
    {
        let eol = alt((eof, recognize(line_ending)));
        let pmap = terminated(tuple((take_u64, space1, take_u64, space1, take_u64)), eol);
        let pmap = map(pmap, |(dst, _, src, _, len)| Mapping { dst, src, len });

        iterator(input, pmap)
    }

    fn collect_ranges(input: &[u8]) -> IResult<&[u8], Vec<Range<u64>>> {
        many1(preceded(space1, map(separated_pair(take_u64, space1, take_u64), |(s, l)| s..s+l)))(input)
    }
}

#[derive(Debug)]
struct Mapping {
    dst: u64,
    src: u64,
    len: u64,
}

impl Mapping {
    pub fn map(&self, n: u64) -> Option<u64> {
        n.checked_sub(self.src)
            .filter(|&d| d < self.len)
            .map(|diff| self.dst + diff)
    }
}

trait Mappable {
    type Iter: IntoIterator<Item = Self>;

    // Applies the mapping
    fn apply(&mut self, map: &Mapping) -> Option<Self::Iter>;
}

impl Mappable for u64 {
    type Iter = std::iter::Empty<Self>;

    fn apply(&mut self, map: &Mapping) -> Option<Self::Iter> {
        map.map(*self).map(|upd| {
            *self = upd;
            std::iter::empty()
        })
    }
}

impl Mappable for Range<u64> {
    type Iter = SmallVec<[Self; 2]>;

    fn apply(&mut self, map: &Mapping) -> Option<Self::Iter> {
        let union = self.start.max(map.src)..self.end.min(map.src+map.len);
        if union.is_empty() {
            return None;
        }

        let mut addl = SmallVec::new();

        // Check left
        let left = self.start..union.start;
        if !left.is_empty() {
            addl.push(left);
        }

        // Check right
        let right = union.end..self.end;
        if !right.is_empty() {
            addl.push(right);
        }

        // Apply difference
        let diff = map.dst.wrapping_sub(map.src);
        let diff = |n| diff.wrapping_add(n);
        *self = diff(union.start)..diff(union.end);

        Some(addl)
    }
}
