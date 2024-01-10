use indexmap::IndexMap;
use smallvec::SmallVec;

use crate::{prelude::*, utils::NomFail};

day!(19);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let (_, system) = whatever!(parser::system(&input).finish().map_err(NomFail::from), "Failed to parse input");

    let part1 = solve(&system);
    println!("Part 1: {part1}");

    let part2 = count_states(&system);
    println!("Part 2: {part2}");

    Ok(())
}

fn count_states(system: &System<'_>) -> u64 {
    let mut count = 0u64;
    let init = PartRanges::new(1..4001);
    let start = system.flows.get_index_of(&b"in"[..]).unwrap();
    let mut stack = vec![(init, start)];

    while let Some((mut ranges, label)) = stack.pop() {
        let cur = system.flows[label].as_ref().unwrap();
        for rule in &cur.conds {
            let pass;
            [pass, ranges] = ranges.split(&rule.comp);

            match rule.insn {
                Insn::Reject => (),
                Insn::Accept => count += pass.num_states(),
                Insn::Goto(idx) => stack.push((pass, idx)),
            }
        }

        match cur.term {
            Insn::Reject => (),
            Insn::Accept => count += ranges.num_states(),
            Insn::Goto(idx) => stack.push((ranges, idx)),
        }
    }

    count
}

type Range = std::ops::Range<u16>;

#[derive(Debug, Clone)]
struct PartRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRanges {
    pub fn new(range: Range) -> Self {
        Self {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range,
        }
    }

    pub fn num_states(&self) -> u64 {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .map(|r| u64::from(r.end - r.start))
            .product()
    }

    pub fn split(&self, cond: &Compare) -> [Self; 2] {
        let mut ranges = [self.clone(), self.clone()];
        let [pass, fail] = &mut ranges;
    
        let (pass, fail) = match cond.category {
            Category::Cool => (&mut pass.x, &mut fail.x),
            Category::Music => (&mut pass.m, &mut fail.m),
            Category::Aero => (&mut pass.a, &mut fail.a),
            Category::Shiny => (&mut pass.s, &mut fail.s),
        };

        if cond.greater {
            let amt = cond.amt + 1;
            pass.start = pass.start.max(amt);
            fail.end = fail.end.min(amt);
        } else {
            pass.end = pass.end.min(cond.amt);
            fail.start = fail.start.max(cond.amt);
        }

        ranges
    }
}

fn solve(system: &System<'_>) -> u64 {
    system
        .parts
        .iter()
        .filter(|p| system.validate(p))
        .map(|p| u64::from(p.total()))
        .sum()
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    pub fn total(&self) -> u32 {
        [self.x, self.m, self.a, self.s]
            .iter()
            .map(|&n| u32::from(n))
            .sum()
    }
}

#[derive(Debug)]
struct Workflow {
    conds: SmallVec<[Rule; 6]>,
    term: Insn,
}

#[derive(Debug)]
struct Rule {
    comp: Compare,
    insn: Insn,
}

#[derive(Debug, Copy, Clone)]
enum Insn {
    Goto(usize),
    Accept,
    Reject,
}

#[derive(Debug)]
struct Compare {
    category: Category,
    greater: bool,
    amt: u16,
}

impl Compare {
    pub fn check(&self, part: &Part) -> bool {
        let cat = match self.category {
            Category::Cool => part.x,
            Category::Music => part.m,
            Category::Aero => part.a,
            Category::Shiny => part.s,
        };

        if self.greater {
            cat > self.amt
        } else {
            cat < self.amt
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Category {
    Cool,
    Music,
    Aero,
    Shiny,
}

#[derive(Debug)]
struct System<'l> {
    flows: IndexMap<&'l [u8], Option<Workflow>>,
    parts: Vec<Part>,
}

impl System<'_> {
    fn validate(&self, part: &Part) -> bool {
        let mut cur = self.flows[&b"in"[..]].as_ref().expect("Missing start node `in`");
        loop {
            let insn = cur.conds
                .iter()
                .filter(|c| c.comp.check(part))
                .map(|c| c.insn)
                .next()
                .unwrap_or(cur.term);

            let idx = match insn {
                Insn::Accept => return true,
                Insn::Reject => return false,
                Insn::Goto(idx) => idx,
            };

            cur = self.flows[idx].as_ref().expect("Missing referenced node");
        }
    }
}

mod parser {
    use crate::utils::parser::line;
    use indexmap::{IndexMap, map::Entry};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::{is_alphabetic, complete::{u16 as take_u16, line_ending}},
        combinator::{value, map, opt, eof},
        sequence::{tuple, terminated, delimited, preceded},
        IResult, Parser, multi::many1,
    };
    use smallvec::SmallVec;

    use super::{Category, Compare, Rule, Insn, System, Workflow, Part};

    fn parse_label(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while1(is_alphabetic)(input)
    }

    fn compare(input: &[u8]) -> IResult<&[u8], Compare> {
        tuple((
            alt((
                value(Category::Aero, tag("a")),
                value(Category::Cool, tag("x")),
                value(Category::Shiny, tag("s")),
                value(Category::Music, tag("m")),
            )),
            alt((value(false, tag("<")), value(true, tag(">")))),
            take_u16,
        ))
        .map(|(c, g, n)| Compare {
            category: c,
            greater: g,
            amt: n,
        })
        .parse(input)
    }

    fn part(input: &[u8]) -> IResult<&[u8], Part> {
        delimited(
            tag("{x="),
            tuple((
                take_u16,
                preceded(tag(",m="), take_u16),
                preceded(tag(",a="), take_u16),
                preceded(tag(",s="), take_u16),
            )),
            tag("}")
        ).map(|(x, m, a, s)| Part { x, m, a, s })
        .parse(input)
    }

    pub fn system(mut input: &[u8]) -> IResult<&[u8], System<'_>> {
        let mut flows = IndexMap::new();
    
        let mut insert_label = |label, workflow: Option<Workflow>| -> usize {
            match flows.entry(label) {
                Entry::Vacant(slot) => {
                    let idx = slot.index();
                    slot.insert(workflow);
                    idx
                }
                Entry::Occupied(mut entry) => {
                    if workflow.is_some() {
                        entry.insert(workflow);
                    }
                    entry.index()
                },
            }
        };

        loop {
            let mut conds = SmallVec::new();
    
            let label;
            (input, label) = terminated(parse_label, tag("{"))(input)?;

            let workflow = loop {
                let comp;
                (input, comp) = opt(terminated(compare, tag(":")))(input)?;

                let insn;
                (input, insn) = alt((
                    map(tag("A"), |_| Insn::Accept),
                    map(tag("R"), |_| Insn::Reject),
                    map(parse_label, |l| Insn::Goto(insert_label(l, None))),
                ))(input)?;

                match comp {
                    Some(comp) => {
                        (input, _) = tag(",")(input)?;
                        conds.push(Rule { comp, insn });
                    },
                    None => {
                        break Workflow {
                            conds,
                            term: insn,
                        }
                    }
                }
            };

            insert_label(label, Some(workflow));
            (input, _) = tuple((tag("}"), line_ending))(input)?;

            if let Ok((inp, _)) = line_ending::<_, ()>(input) {
                input = inp;
                break;
            }
        }

        let parts;
        (input, parts) = terminated(many1(line(part)), eof)(input)?;

        Ok((input, System {
            flows,
            parts,
        }))
    }
}
