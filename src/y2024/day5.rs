use std::{cmp::Ordering, collections::HashMap, ops::Range};

use crate::utils::{NomFail, Pair};

use super::Solution;

day!(run 5);

// Expects array to be sorted, true indicates a precedes b for [a,b]
type NumMap = HashMap<[u8; 2], bool>;
// Indices into NumMap
type Spans = Vec<Range<usize>>;

struct Day5 {
    map: NumMap,
    nums: Vec<u8>,
    spans: Spans,
}

impl<'i> Solution<'i> for Day5 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        parse::parse(input)
    }

    // 2: 9419 too high
    fn part1(&mut self) -> Pair<u64, u64> {
        let nums = &mut *self.nums;

        let mut sum1 = 0u64;
        let mut sum2 = 0u64;

        let center = |s: &[u8]| u64::from(s[s.len() / 2]);

        for range in self.spans.iter().cloned() {
            let span = &mut nums[range];
            let valid = span.is_sorted_by(|&a, &b| is_ordered(&self.map, a, b));
            if valid {
                sum1 += center(span);
            } else {
                span.sort_unstable_by(|&a, &b| {
                    if is_ordered(&self.map, a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                sum2 += center(span);
            }
        }

        Pair(sum1, sum2)
    }
}

fn is_ordered(map: &NumMap, a: u8, b: u8) -> bool {
    let (key, ch) = if b > a {
        ([a, b], false)
    } else {
        ([b, a], true)
    };

    ch ^ *map
        .get(&key)
        .unwrap_or_else(|| panic!("{key:?} not in map"))
}

mod parse {
    use std::cmp::Ordering;

    use super::{Day5, NumMap, Spans};
    use crate::utils::{parser::line, NomFail};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, u8},
        combinator::{all_consuming, iterator, opt, value},
        error::Error,
        multi::fold_many1,
        sequence::{preceded, separated_pair},
        Finish, IResult,
    };

    fn ord(input: &[u8]) -> IResult<&[u8], ([u8; 2], bool)> {
        let (i, (a, b)) = line(separated_pair(u8, tag([b'|']), u8))(input)?;
        let r = match a.cmp(&b) {
            Ordering::Less => ([a, b], true),
            Ordering::Greater => ([b, a], false),
            Ordering::Equal => {
                return Err(nom::Err::Failure(Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )))
            }
        };
        Ok((i, r))
    }

    fn map(input: &[u8]) -> IResult<&[u8], NumMap> {
        let mut map = NumMap::new();
        let (i, ()) = fold_many1(ord, || (), |(), (k, v)| _ = map.insert(k, v))(input)?;
        Ok((i, map))
    }

    fn nums(input: &[u8]) -> IResult<&[u8], (Vec<u8>, Spans)> {
        let mut nums = Vec::new();
        let mut spans = Spans::new();

        let pnum = preceded(opt(tag([b','])), alt((u8, value(0, line_ending))));
        let mut it = iterator(input, pnum);
        let mut start = 0;
        for n in (&mut it).chain([0]) {
            if n == 0 {
                let i = nums.len();
                if start < i {
                    spans.push(start..i);
                    start = i;
                }
            } else {
                nums.push(n);
            }
        }

        it.finish().map(|(i, ())| (i, (nums, spans)))
    }

    pub fn parse(input: &[u8]) -> Result<Day5, NomFail> {
        let (_, (map, (nums, spans))) =
            all_consuming(separated_pair(map, line_ending, nums))(input).finish()?;
        Ok(Day5 { map, nums, spans })
    }
}
