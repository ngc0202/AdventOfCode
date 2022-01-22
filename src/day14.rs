
use crate::prelude::*;
use std::ops::{Add, AddAssign};
use std::iter::Sum;
use std::sync::Arc;
use cached::proc_macro::cached;

const DAY: u8 = 14;

type Template = ([u8; 2], u8);

#[derive(Clone, Debug)]
struct Info {
    polymer: Vec<u8>,
    templates: Vec<Template>,
}

impl FromStr for Info {
    type Err = ();

    fn from_str(s: &str) -> Result<Info, ()> {
        let (p, t) = s.split_once("\n\n").ok_or(())?;
        t.trim()
            .split('\n')
            .map(|s| s.split_once(" -> ").and_then(|(a, b)|
               b.chars()
                .exactly_one()
                .ok()
                .and_then(|b| a.bytes().collect_tuple().map(|(x, y)| ([x, y], b as u8)))))
            .collect::<Option<Vec<_>>>()
            .ok_or(())
            .map(|templates|
                Info {
                    polymer: p.into(),
                    templates
                })
    }
}

#[derive(Clone, Debug, Default)]
struct Counts([u64; 27]);

impl Add for Counts {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += &rhs;
        self
    }
}

impl AddAssign<&Self> for Counts {
    fn add_assign(&mut self, rhs: &Self) {
        for (a, b) in self.0.iter_mut().zip(rhs.0) {
            *a += b;
        }
    }
}

impl Sum<Self> for Counts {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Counts {
        iter.fold(Counts::default(), Add::add)
    }
}

impl Counts {
    // Most common - least common
    pub fn value(&self) -> u64 {
        self.0
            .iter()
            .filter(|&&b| b != 0)
            .minmax()
            .into_option()
            .map(|(min, max)| max - min)
            .unwrap_or(0)
    }

    pub fn inc(&mut self, c: u8) {
        assert!((b'A'..=b'Z').contains(&c));
        let idx = usize::from(c - b'A');
        self.0[idx] += 1;
    }
}

fn get_output<'a, I: IntoIterator<Item=&'a Template>>(left: u8, right: u8, templates: I) -> Option<u8> {
    let arr = [left, right];
    templates
        .into_iter()
        .find_map(|t| (t.0 == arr).then(|| t.1))
}

#[cached]
fn recurse_solve(left: u8, right: u8, depth: usize, templates: Arc<[Template]>) -> Counts {
    let mut counts = Counts::default();
    if let Some(repl) = get_output(left, right, &*templates) {
        counts.inc(repl);
        if depth > 0 {
            counts += &recurse_solve(left, repl, depth - 1, Arc::clone(&templates));
            counts += &recurse_solve(repl, right, depth - 1, templates);
        }
    }
    counts
}

fn solve(info: &Info, steps: usize) -> u64 {
    let templates = info.templates.as_slice().into();
    info.polymer
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| recurse_solve(a, b, steps-1, Arc::clone(&templates)))
        .sum::<Counts>()
        .value()
}

pub fn run() -> GenResult {
    let input = load_input_string(DAY)?;
    let info: Info = input.as_str()
                          .parse()
                          .map_err(|_| "parse fail")?;

    let part1 = solve(&info, 10);
    println!("Part 1: {}", part1);

    let part2 = solve(&info, 40);
    println!("Part 2: {}", part2);

    Ok(())
}