
use crate::{prelude::*, y2022::day5::parsing::commands_iter};
use nom::Finish;
use smallvec::SmallVec;

use self::parsing::parse_dock;

day!(5);

type DockInner = SmallVec<[Vec<u8>; 9]>;

#[derive(Debug, Clone)]
struct Dock {
    stacks: DockInner,
}

impl Dock {
    pub fn parse_from(s: &[u8]) -> Result<(Self, &[u8]), nom::Err<nom::error::Error<Vec<u8>>>> {
        let (s, d) = parse_dock(s)
            .finish()
            .map_err(|e| nom::Err::Error(e).to_owned())?;
        Ok((d, s))
    }

    pub fn get_two_mut(
        &mut self,
        idx1: usize,
        idx2: usize
    ) -> Option<(&mut Vec<u8>, &mut Vec<u8>)> {
        let stacks = &mut self.stacks;
        if idx1 == idx2 || idx1 >= stacks.len() || idx2 >= stacks.len() {
            return None;
        }

        Some(if idx1 < idx2 {
            let (sl1, sl2) = stacks.split_at_mut(idx2);
            (sl1.get_mut(idx1)?, sl2.get_mut(0)?)
        } else {
            let (sl1, sl2) = stacks.split_at_mut(idx1);
            (sl2.get_mut(0)?, sl1.get_mut(idx2)?)
        })
    }

    pub fn exec_cmd(
        &mut self,
        Command { amt, from, to }: Command,
        part: Part
    ) -> Result<(), SimError> {
        if from == to {
            return Ok(());
        }

        let (from, to) = (from - 1, to - 1);

        let stack_len = self.stacks.len();

        let (src, dst) = 
            self.get_two_mut(usize::from(from), usize::from(to))
                .ok_or(SimError::BadIndex {
                    idx: from,
                    len: stack_len 
                })?;

        let src_len = src.len();
        let uamt = usize::from(amt);
        if src_len < uamt {
            return Err(SimError::TooSmallStack {
                num: amt,
                len: src_len 
            });
        }

        match part {
            Part::One => dst.extend(src.drain((src_len - uamt)..src_len).rev()),
            Part::Two => dst.extend(src.drain((src_len - uamt)..src_len)),
        }

        Ok(())
    }
}

impl std::fmt::Display for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.stacks.iter().map(Vec::len).max().unwrap_or(0);
        if max == 0 {
            return Ok(());
        }

        let line_fmt = |i: usize| {
            self.stacks
                .iter()
                .format_with(" ", move |stack, f| match stack.get(i) {
                    Some(&c) => f(&format_args!("[{}]", char::from(c))),
                    None => f(&"   "),
                })
        };

        writeln!(f)?;
        for i in (0..max).rev() {
            writeln!(f, "{}", line_fmt(i))?;
        }

        let nums = (1..=self.stacks.len()).format("   ");
        writeln!(f, " {nums}")?;

        Ok(())
    }
}

mod parsing {
    use crate::utils::{eof_iterator, EofParserIterator};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till},
        character::{
            complete::{line_ending, multispace1, u8 as nom_u8},
            is_newline,
        },
        combinator::{iterator, map, opt},
        number::complete::le_u8,
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    use super::{Command, Dock, DockInner};

    fn crane_slot(s: &[u8]) -> IResult<&[u8], Option<u8>> {
        alt((
            map(tag(&b"   "[..]), |_| None),
            map(delimited(tag(&b"["[..]), le_u8, tag(&b"]"[..])), Some)
        ))(s)
    }

    pub(super) fn parse_dock(s: &[u8]) -> IResult<&[u8], Dock> {
        let mut stacks = DockInner::new();

        let mut it = iterator(
            s,
            terminated(
                |s| {
                    let mut it = iterator(s, preceded(opt(tag(&b" "[..])), crane_slot));

                    for (idx, slot) in it.enumerate() {
                        if stacks.len() < idx + 1 {
                            stacks.push(Vec::new());
                        }

                        if let Some(val) = slot {
                            stacks[idx].insert(0, val);
                        }
                    }

                    it.finish()
                },
                line_ending
            ),
        );

        () = it.collect();
        let (s, ()) = it.finish()?;

        let (s, _) = take_till(is_newline)(s)?;
        let (s, _) = multispace1(s)?;

        IResult::Ok((s, Dock { stacks }))
    }

    fn command(s: &[u8]) -> IResult<&[u8], Command> {
        map(
            terminated(
                tuple((
                    preceded(tag(&b"move "[..]), nom_u8),
                    preceded(tag(&b" from "[..]), nom_u8),
                    preceded(tag(&b" to "[..]), nom_u8),
                )),
                opt(line_ending)
            ),
            |(amt, from, to)| Command { amt, from, to }
        )(s)
    }

    pub(super) fn commands_iter(
        s: &[u8]
    ) -> EofParserIterator<
        &[u8],
        nom::error::Error<&[u8]>,
        impl FnMut(&[u8]) -> IResult<&[u8], Command>>
    {
        eof_iterator(s, command)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Command {
    amt: u8,
    from: u8,
    to: u8,
}

#[derive(Debug, Snafu)]
enum SimError {
    #[snafu(display("Attempted to move {num} crates from a stack with only {len}"))]
    TooSmallStack { num: u8, len: usize },
    #[snafu(display("Attempted to index at {idx} from a dock with only {len} stacks"))]
    BadIndex { idx: u8, len: usize },
    #[snafu(display("Stack {idx} is empty at the end of simulation"))]
    EmptyStack { idx: usize },
}

#[derive(Copy, Clone, Debug)]
enum Part { One, Two }

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::One => "Part 1",
            Self::Two => "Part 2",
        })
    }
}

fn run_sim(mut dock: Dock, cmds: &[u8], part: Part) -> Result<(), Whatever> {
    let mut cmds = commands_iter(cmds);
    for cmd in &mut cmds {
        whatever!(dock.exec_cmd(cmd, part), "Failed executing command");
    }

    if let Err(err) = cmds.finish() {
        whatever!(Err(err.to_string()), "Failed parsing dock instructions");
    }

    print!("{part}: ");

    for (idx, stack) in dock.stacks.iter().enumerate() {
        let &top = whatever!(
            stack.last().ok_or(SimError::EmptyStack { idx }),
            "Failed grabbing top of stack"
        );
        print!("{}", char::from(top));
    }
    println!();

    Ok(())
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed reading input");

    let (dock, commands) = whatever!(Dock::parse_from(&input), "Failed parsing dock");
    let dock2 = dock.clone();

    whatever!(run_sim(dock,  commands, Part::One), "Failed running part 1");
    whatever!(run_sim(dock2, commands, Part::Two), "Failed running part 2");

    Ok(())
}
