use super::Solution;
use crate::utils::{sgrid::{Dir, Grid}, Coord, NomFail};

day!(run 18);

struct Day18 {
    grid: Grid<bool>,
    coords: Vec<Coord<u8>>,
}

impl<'i> Solution<'i> for Day18 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        let width = if crate::get_small() { 7 } else { 71 };
        Ok(Self {
            grid: Grid::new_default(width * width, width),
            coords: parse::parse(input)?,
        })
    }

    fn part1(&mut self) -> usize {
        // Corrupt the grid
        let ncorrupt = if crate::get_small() { 12 } else { 1024 };
        self.corrupt(ncorrupt);

        // State
        let end = self.grid.len() - 1;
        let mut stack = vec![[0usize; 2]];
        let mut dists = vec![None; self.grid.len()];

        // Traverse
        while let Some([idx, dist]) = stack.pop() {
            // Skip walls
            if self.grid[idx] {
                continue
            }

            // Update dist, stop if worse or at end
            if !min(&mut dists[idx], dist) || idx == end {
                continue
            }

            // Traverse
            let ndist = dist + 1;
            for dir in Dir::ALL {
                if let Some(nidx) = self.grid.dir_index(idx, dir) {
                    stack.push([nidx, ndist]);
                }
            }
        }

        dists.last().copied().flatten().expect("No path found")
    }

    fn part2(&mut self) -> Coord<u8> {
        let mut stack = Vec::new();
        let mut seen = vec![false; self.grid.len()];
    
        let mut low = if crate::get_small() { 12 } else { 1024 };
        let mut hi = self.coords.len();

        while low + 1 < hi {
            let mid = (low + hi) / 2;
            self.reset();
            self.corrupt(mid);
            let hpath = self.has_path(&mut stack, &mut seen);
            if hpath {
                low = mid;
            } else {
                hi = mid;
            }
        }

        self.coords[hi-1]
    }
}

fn min(opt: &mut Option<usize>, n: usize) -> bool {
    match opt {
        Some(v) if *v <= n => false,
        _ => {
            *opt = Some(n);
            true
        },
    }
}

impl Day18 {
    fn has_path(&self, stack: &mut Vec<usize>, seen: &mut [bool]) -> bool {
        let end = self.grid.len() - 1;
        stack.clear();
        stack.push(0);
        seen.fill(false);
        while let Some(idx) = stack.pop() {
            if idx == end {
                return true
            }
            if self.grid[idx] || std::mem::replace(&mut seen[idx], true) {
                continue
            }
            for dir in Dir::ALL {
                if let Some(nidx) = self.grid.dir_index(idx, dir) {
                    stack.push(nidx);
                }
            }
        }
        false
    }

    fn corrupt(&mut self, n: usize) {
        self.coords[..n]
            .into_iter()
            .for_each(|&i| self.grid[i] = true);
    }

    fn reset(&mut self) {
        self.grid.fill(false);
    }
}

mod parse {
    use crate::utils::{parser::line, Coord, NomFail};
    use nom::{
        bytes::complete::tag, character::complete::u8, combinator::all_consuming, multi::many1,
        sequence::separated_pair, Finish, IResult, Parser,
    };

    fn coord(input: &[u8]) -> IResult<&[u8], Coord<u8>> {
        separated_pair(u8, tag(","), u8)
            .map(|(x, y)| Coord { x, y })
            .parse(input)
    }

    pub fn parse(input: &[u8]) -> Result<Vec<Coord<u8>>, NomFail> {
        Ok(all_consuming(many1(line(coord)))(input).finish()?.1)
    }
}
