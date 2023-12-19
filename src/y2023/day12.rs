use std::collections::{hash_map::Entry, HashMap};

use crate::{prelude::*, utils::NomFail};

day!(12);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spring {
    Unknown,
    Oper,
    Inop,
}

#[derive(Debug, Snafu)]
#[snafu(display("Invalid spring character with value {value:#04X}"))]
pub struct InvalidSpring {
    value: u8,
}

impl TryFrom<u8> for Spring {
    type Error = InvalidSpring;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'?' => Self::Unknown,
            b'.' => Self::Oper,
            b'#' => Self::Inop,
            _ => return Err(InvalidSpring { value }),
        })
    }
}

type Range = std::ops::Range<usize>;

fn ranges_into(groups: &[u8], spots: usize, ranges: &mut Vec<Range>) {
    // Allocate ranges
    ranges.extend(iter::repeat(0..0).take(groups.len()));

    // Set starts
    ranges
        .iter_mut()
        .map(|x| &mut x.start)
        .set_from(groups.iter().scan(0, |st, &g| {
            let v = *st;
            *st += usize::from(g) + 1;
            Some(v)
        }));

    // Set ends
    ranges
        .iter_mut()
        .map(|x| &mut x.end)
        .rev()
        .set_from(groups.iter().rev().scan(spots + 1, |st, &g| {
            let ret = st.saturating_sub(usize::from(g));
            *st = ret.saturating_sub(1);
            Some(ret)
        }));
}

struct Position<'a> {
    groups: &'a [u8],
    spots: &'a [Spring],
    ranges: &'a [Range],
    offset: usize,
}

enum State<'a> {
    PopStack,
    Position(Position<'a>),
}

impl<'a> From<Position<'a>> for State<'a> {
    fn from(pos: Position<'a>) -> Self {
        State::Position(pos)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Frame {
    spot_idx: usize,
    group_idx: usize,
}

fn count_arrs(ospots: &[Spring], ogroups: &[u8], oranges: &[Range]) -> u64 {
    let init = Position {
        groups: ogroups,
        spots: ospots,
        ranges: oranges,
        offset: 0,
    };

    let mut tasks = vec![init.into()];
    let mut stack = Vec::new();
    let mut cache = HashMap::new();

    while let Some(task) = tasks.pop() {
        // Pop next task
        let state = match task {
            State::Position(p) => p,
            State::PopStack => {
                let f = stack.pop().unwrap();
                let val = cache.get(&f).copied().unwrap_or(0);
                if let Some(tkey) = stack.last() {
                    let top = cache.entry(*tkey).or_default();
                    *top += val;
                }
                continue;
            }
        };

        // Check cache
        let curframe = Frame {
            spot_idx: ptr_diff(ospots, state.spots),
            group_idx: ptr_diff(ogroups, state.groups),
        };

        let entry = match cache.entry(curframe) {
            Entry::Vacant(e) => e.insert(0),
            Entry::Occupied(e) => {
                let top = stack.last().unwrap();
                *cache.entry(*top).or_default() += *e.get();
                continue;
            }
        };

        // Add stack frame
        tasks.push(State::PopStack);
        stack.push(curframe);

        // Check for finished
        let Some((group, grest)) = state.groups.split_first() else {
            // Check tail is no INOP
            let tail_valid = state.spots.iter().all(|&s| s != Spring::Inop);
            if tail_valid {
                // Add cache entry
                *entry += 1;
            }
            continue;
        };

        let (range, rrest) = state.ranges.split_first().unwrap();

        let spots = state.spots;
        let gsize = usize::from(*group);

        // Check prefix
        let diff = range.start - state.offset;
        if spots.len() < diff {
            continue;
        }

        let offset = state.offset + diff;
        let (prefix, spots) = spots.split_at(diff);
        if prefix.iter().any(|&s| s == Spring::Inop) {
            continue;
        }

        for idx in 0..range.end - range.start {
            // Check length
            if spots.len() < idx + gsize {
                continue;
            }

            // Split the spots
            let (prefix, spots) = spots.split_at(idx);
            let (spots, mut srest) = spots.split_at(gsize);

            // Check prefix has no INOP
            let prefix_invalid = prefix.iter().any(|&s| s == Spring::Inop);
            if prefix_invalid {
                continue;
            }

            // Check for either an INOP or the of the slice
            let last_invalid = srest.first().copied() == Some(Spring::Inop);
            if last_invalid {
                continue;
            }

            // Check no OPER in spots
            let valid = spots.iter().all(|&s| s != Spring::Oper);
            if !valid {
                continue;
            }

            let mut offset = offset + gsize;
            if let Some((_, r)) = srest.split_first() {
                srest = r;
                offset += 1;
            }

            let pos = Position {
                groups: grest,
                spots: srest,
                ranges: rrest,
                offset,
            };

            tasks.push(pos.into());
        }
    }

    cache[&Frame::default()]
}

fn ptr_diff<T: ?Sized>(a: *const T, b: *const T) -> usize {
    (a.cast::<()>() as usize).abs_diff(b.cast::<()>() as usize)
}

fn solve(mut input: &[u8]) -> Result<(u64, u64), NomFail> {
    let mut part1 = 0u64;
    let mut part2 = 0u64;

    let mut springs = Vec::new();
    let mut groups = Vec::new();
    let mut ranges = Vec::new();

    while !input.is_empty() {
        springs.clear();
        groups.clear();
        ranges.clear();

        // Parse and fill vecs
        let res = parser::parse_into(input, &mut springs, &mut groups);
        (input, ()) = res.finish().map_err(NomFail::from)?;

        // Part 1
        ranges_into(&groups, springs.len(), &mut ranges);
        part1 += count_arrs(&springs, &groups, &ranges);

        // Part 2
        let slen = springs.len();
        let glen = groups.len();
        for _ in 0..4 {
            springs.push(Spring::Unknown);
            springs.extend_from_within(..slen);
            groups.extend_from_within(..glen);
        }

        ranges.clear();
        ranges_into(&groups, springs.len(), &mut ranges);
        part2 += count_arrs(&springs, &groups, &ranges);
    }

    Ok((part1, part2))
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let (part1, part2) = whatever!(solve(&input), "Failed parsing input");
    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, space1},
        combinator::{eof, map_res},
        multi::fold_many1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    use super::Spring;

    pub fn parse_into<'i>(
        input: &'i [u8],
        springs: &mut Vec<Spring>,
        groups: &mut Vec<u8>,
    ) -> IResult<&'i [u8], ()> {
        separated_pair(
            fold_many1(
                map_res(nom::number::complete::u8, Spring::try_from),
                || (),
                |(), v| springs.push(v),
            ),
            space1,
            fold_many1(
                terminated(
                    nom::character::complete::u8,
                    alt((tag(","), line_ending, eof)),
                ),
                || (),
                |(), v| groups.push(v),
            ),
        )(input)
        .map(|(i, ((), ()))| (i, ()))
    }
}
