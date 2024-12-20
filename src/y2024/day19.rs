use std::collections::{HashMap, HashSet};

use super::Solution;
use crate::utils::NomFail;

day!(run 19 <'i>);

struct Day19<'i> {
    trie: trie::Trie,
    designs: Vec<&'i [u8]>,
}

impl<'i> Solution<'i> for Day19<'i> {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        parse::parse(input)
    }

    fn part1(&mut self) -> usize {
        self.designs.iter().filter(|d| self.is_possible(d)).count()
    }

    fn part2(&mut self) -> u64 {
        let mut cache = HashMap::new();
        self.designs
            .iter()
            .map(|d| self.count_possible(d, 0, &mut cache))
            .sum()
    }
}

impl<'i> Day19<'i> {
    pub fn is_possible(&self, design: &[u8]) -> bool {
        #[derive(Hash, PartialEq, Eq, Copy, Clone)]
        struct State {
            idx: usize,
            base: usize,
        }

        if design.is_empty() {
            return true;
        }

        let mut stack = vec![State { idx: 0, base: 0 }];
        let mut seen = HashSet::new();

        while let Some(state) = stack.pop() {
            // Check end terminates
            if state.idx == design.len() {
                if self.trie.terminal(state.base) {
                    return true;
                }
                continue;
            }

            // Skip seen
            if !seen.insert(state) {
                continue;
            }

            // Fork
            let fork = self.trie.fork(design[state.idx], state.base);
            let nidx = state.idx + 1;
            stack.extend(
                fork.into_iter()
                    .flatten()
                    .map(|base| State { idx: nidx, base }),
            );
        }

        false
    }

    pub fn count_possible(
        &self,
        design: &'i [u8],
        base: usize,
        cache: &mut HashMap<(&'i [u8], usize), u64>,
    ) -> u64 {
        // Check cache
        if let Some(&n) = cache.get(&(design, base)) {
            return n;
        }

        // Split input
        let Some((&c, tail)) = design.split_first() else {
            // End must terminate
            return self.trie.terminal(base) as u64;
        };

        // Fork
        let fork = self.trie.fork(c, base);
        let sum = fork
            .into_iter()
            .flatten()
            .map(|base| self.count_possible(tail, base, cache))
            .sum();

        // Add to cache
        cache.insert((design, base), sum);
        sum
    }
}

mod trie {
    use super::color;

    // terminator + 5(wubrg)
    const ALPHA_SIZE: usize = 6;

    #[derive(Debug, Default)]
    pub struct Trie {
        num_bases: usize,
        data: Vec<usize>,
    }

    impl Trie {
        pub fn insert(&mut self, mut s: &[u8]) {
            let mut base = 0;
            loop {
                self.resize(base);
                let offset = if let Some((&c, tail)) = s.split_first() {
                    // Got color
                    s = tail;
                    usize::from(color(c)) + 1
                } else {
                    // Got terminator
                    0
                };

                // Do insertion
                let slot = &mut self.data[base * ALPHA_SIZE + offset];
                if offset == 0 {
                    *slot = usize::MAX;
                    break;
                }

                // Add new base
                if *slot == 0 {
                    *slot = self.num_bases;
                }

                base = *slot;
            }
        }

        /// Advances the cursor, forking if a terminator is available
        pub fn fork(&self, c: u8, base: usize) -> [Option<usize>; 2] {
            let offset = usize::from(color(c)) + 1;

            // Check continuing
            let cont_base = self.data[base * ALPHA_SIZE + offset];
            let cont = (cont_base > 0).then_some(cont_base);

            // Check terminating
            let mut term = None;
            if self.terminal(base) {
                let term_base = self.data[offset];
                if term_base > 0 {
                    term = Some(term_base);
                }
            }

            [cont, term]
        }

        pub fn terminal(&self, base: usize) -> bool {
            self.data[base * ALPHA_SIZE] > 0
        }

        fn resize(&mut self, base: usize) {
            if base >= self.num_bases {
                self.num_bases = base + 1;
                let new_len = ALPHA_SIZE * self.num_bases;
                self.data.resize(new_len, 0);
            }
        }
    }
}

fn color(b: u8) -> u8 {
    match b {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => panic!("Invalid color {b}"),
    }
}

mod parse {
    use super::{trie::Trie, Day19};
    use crate::utils::{parser::line, NomFail};
    use nom::{
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::{all_consuming, iterator, opt},
        error::ErrorKind,
        multi::many1,
        sequence::{separated_pair, terminated},
        Finish, IResult, InputTakeAtPosition,
    };

    fn pattern(input: &[u8]) -> IResult<&[u8], &[u8]> {
        input.split_at_position1_complete(
            |b| !matches!(b, b'w' | b'u' | b'b' | b'r' | b'g'),
            ErrorKind::Alpha,
        )
    }

    fn trie(input: &[u8]) -> IResult<&[u8], Trie> {
        let mut trie = Trie::default();
        let mut iter = iterator(input, terminated(pattern, opt(tag(", "))));
        for pat in &mut iter {
            trie.insert(pat);
        }
        iter.finish().map(|(i, ())| (i, trie))
    }

    pub fn parse(input: &[u8]) -> Result<Day19<'_>, NomFail> {
        let designs = many1(line(pattern));
        let (_, (t, d)) =
            all_consuming(separated_pair(line(trie), line_ending, designs))(input).finish()?;
        Ok(Day19 {
            trie: t,
            designs: d,
        })
    }
}
