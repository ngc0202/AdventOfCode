use std::mem;

use super::Solution;
use crate::utils::{
    sgrid::{Dir, GridParseErr},
    Pair,
};

day!(run 12);

type Grid = crate::utils::sgrid::Grid<Tile>;
type Stack = Vec<usize>;

struct Day12 {
    grid: Grid,
}

#[derive(Default, Copy, Clone)]
struct Tile {
    region: u8,
    seen: bool,
}

impl<'i> Solution<'i> for Day12 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, GridParseErr> {
        let grid = Grid::parse(input, |b| {
            b.is_ascii_alphabetic().then(|| Tile {
                region: b,
                ..Tile::default()
            })
        })?
        .1;
        Ok(Self { grid })
    }

    fn part1(&mut self) -> Pair<u64, u64> {
        let mut stack = Stack::new();
        let mut part1 = 0;
        let mut part2 = 0;
        for idx in 0..self.grid.len() {
            let [a, p, s] = self.calc_cost(idx, &mut stack);
            part1 += a * p;
            part2 += a * s;
        }

        Pair(part1, part2)
    }
}

impl Day12 {
    /// Finds the cost of the region containing `start`
    pub fn calc_cost(&mut self, idx: usize, stack: &mut Vec<usize>) -> [u64; 3] {
        let start = self.grid[idx];
        let region = start.region;
        if region == 0 || start.seen {
            return [0; 3];
        }

        let mut area = 0u64;
        let mut perimeter = 0u64;
        let mut sides = 0u64;

        stack.push(idx);
        while let Some(idx) = stack.pop() {
            let cur = &mut self.grid[idx];

            // Check inside or out
            if cur.region == region {
                if !mem::replace(&mut cur.seen, true) {
                    area += 1;
                    sides += u64::from(self.corners(idx, region));

                    // Add neighbors
                    for ndir in Dir::ALL {
                        if let Some(ni) = self.grid.dir_index(idx, ndir) {
                            stack.push(ni);
                        } else {
                            perimeter += 1;
                        }
                    }
                }
            } else {
                perimeter += 1;
            }
        }

        [area, perimeter, sides]
    }

    fn corners(&mut self, idx: usize, region: u8) -> u8 {
        let is_inside = |dir| {
            self.grid
                .dir_index(idx, dir)
                .is_some_and(|i| self.grid[i].region == region)
        };
        let regs = Dir::ALL.map(is_inside);
        let mut c = 0;
        for d1 in Dir::ALL {
            let d2 = d1.right();
            c += match [regs[d1 as usize], regs[d2 as usize]] {
                [false, false] => 1,
                [true, true] => {
                    self.grid
                        .dir_index(idx, d1)
                        .and_then(|idx| self.grid.dir_index(idx, d2))
                        .is_none_or(|i| self.grid[i].region == region) as u8
                }
                _ => 0,
            }
        }
        c
    }
}
