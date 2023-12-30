use crate::{prelude::*, utils::NomFail, y2023::day15::parser::fold_opers};

day!(15);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let part1 = part1(&input);
    println!("Part 1: {part1}");

    let part2 = whatever!(part2(&input), "Failed to parse input");
    println!("Part 2: {part2}");

    Ok(())
}

fn part2(input: &[u8]) -> Result<u64, NomFail> {
    let mut map = Hashmap::default();
    fold_opers(
        input,
        || (),
        |(), oper| match oper {
            Oper::Removal { label } => map.remove(label),
            Oper::Insertion { label, focus } => map.insert(label, focus),
        },
    )
    .finish()?;
    Ok(map.power())
}

struct Lens<'i> {
    label: &'i [u8],
    focus: u8,
}

enum Oper<'i> {
    Removal { label: &'i [u8] },
    Insertion { label: &'i [u8], focus: u8 },
}

struct Hashmap<'i> {
    slots: [Vec<Lens<'i>>; 256],
}

impl Default for Hashmap<'_> {
    fn default() -> Self {
        Self {
            slots: [Self::EMPTY; 256],
        }
    }
}

impl<'i> Hashmap<'i> {
    const EMPTY: Vec<Lens<'i>> = Vec::new();

    fn get_slot<'a>(&mut self, label: &'a [u8]) -> &mut Vec<Lens<'i>> {
        &mut self.slots[usize::from(hash(label))]
    }

    pub fn remove(&mut self, label: &'i [u8]) {
        let slot = self.get_slot(label);
        slot.retain(|l| l.label != label);
    }

    pub fn insert(&mut self, label: &'i [u8], focus: u8) {
        let slot = self.get_slot(label);
        let mbox = slot.iter_mut().find(|l| l.label == label);
        match mbox {
            Some(l) => l.focus = focus,
            None => slot.push(Lens { label, focus }),
        }
    }

    pub fn power(&self) -> u64 {
        self.slots
            .iter()
            .zip(1u64..)
            .flat_map(|(s, sn)| {
                s.iter()
                    .zip(1u64..)
                    .map(move |(l, ln)| u64::from(l.focus) * ln * sn)
            })
            .sum()
    }
}

fn part1(input: &[u8]) -> u64 {
    input
        .split(|b| *b == b',')
        .fold(0u64, |acc, s| acc + u64::from(hash(s)))
}

fn hash(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .fold(0u8, |acc, &b| acc.wrapping_add(b).wrapping_mul(17))
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::{complete::char, is_alphabetic, is_digit},
        combinator::{eof, verify},
        multi::fold_many1,
        sequence::{preceded, terminated},
        IResult, Parser,
    };

    use super::Oper;

    pub fn fold_opers<'i, F, H, R>(input: &'i [u8], init: H, f: F) -> IResult<&'i [u8], R>
    where
        H: FnMut() -> R,
        F: FnMut(R, Oper<'i>) -> R,
    {
        let oper = terminated(oper, alt((eof, tag(","))));
        fold_many1(oper, init, f)(input)
    }

    fn focus(input: &[u8]) -> IResult<&[u8], u8> {
        verify(nom::number::complete::u8, |&b| is_digit(b))
            .map(|b| b - b'0')
            .parse(input)
    }

    fn oper(mut input: &[u8]) -> IResult<&[u8], Oper<'_>> {
        let label;
        (input, label) = take_while1(is_alphabetic)(input)?;

        alt((
            char('-').map(|_| Oper::Removal { label }),
            preceded(char('='), focus).map(|focus| Oper::Insertion { label, focus }),
        ))(input)
    }
}
