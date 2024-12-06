use std::collections::HashSet;

use crate::{prelude::*, utils::sgrid::Dir};

use super::Solution;

day!(run 6);

type Grid = crate::utils::sgrid::Grid<Tile>;

struct Day6 {
    grid: Grid,
    start: usize,
    end: usize,
}

impl Solution for Day6 {
    const DAY: Day = DAY;

    fn parse(input: Vec<u8>) -> Result<Self, Grid6Err> {
        let mut start = None;
        let (_, grid) = Grid::parse_co(&input, |b, x, y| {
            let block = match b {
                b'.' => false,
                b'#' => true,
                b'^' => {
                    start = Some([x, y]);
                    false
                }
                _ => return None,
            };

            Some(Tile { block, prev: None })
        })?;

        let [x, y] = start.context(NoStartSnafu)?;
        let start = y * grid.width() + x;

        Ok(Self {
            grid,
            start,
            end: usize::MAX,
        })
    }

    fn part1(&mut self) -> usize {
        let mut idx = self.start;
        let mut dir = Dir::Up;
        self.grid[self.start].prev = Some((usize::MAX, Dir::Up));
        let mut count = 1;

        loop {
            // Store prevs
            let (pidx, pdir) = (idx, dir);

            // Step
            idx = match take_step(&self.grid, idx, &mut dir) {
                Some(i) => i,
                None => {
                    self.end = idx;
                    break;
                }
            };

            // Mark position as visited
            let prev = &mut self.grid[idx].prev;
            if prev.is_none() {
                count += 1;
                *prev = Some((pidx, pdir));
            }
        }

        count
    }

    fn part2(&mut self) -> usize {
        let mut count = 0;
        let mut seen = HashSet::new();

        for bidx in 0..self.grid.len() {
            // Don't block start
            if bidx == self.start {
                continue;
            }

            // Set block and get previous tile
            let tile = &mut self.grid[bidx];
            let Some((mut idx, mut dir)) = tile.prev else {
                continue;
            };
            tile.block = true;

            loop {
                // Mark visited
                let is_loop = !seen.insert((idx, dir));
                if is_loop {
                    count += 1;
                    break;
                }

                // Take step
                idx = match take_step(&self.grid, idx, &mut dir) {
                    Some(i) => i,
                    None => break,
                };
            }

            // Go to next tile
            self.grid[bidx].block = false;
            seen.clear();
        }

        count
    }
}

/// Take next step, turning if necessary
fn take_step(grid: &Grid, idx: usize, dir: &mut Dir) -> Option<usize> {
    for _ in 0..4 {
        // Find next index in dir
        let next_idx = grid.dir_index(idx, *dir)?;

        // Check for obstacle and turn or continue
        if grid[next_idx].block {
            *dir = turn(*dir);
        } else {
            return Some(next_idx);
        }
    }

    panic!("Got stuck in one spot! (idx {idx})")
}

/// Turns right 90 degrees
const fn turn(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Right,
        Down => Left,
        Left => Up,
        Right => Down,
    }
}

#[derive(Debug, Snafu)]
enum Grid6Err {
    #[snafu(display("Missing starting point in grid"))]
    NoStart,
    #[snafu(transparent)]
    Other {
        source: crate::utils::sgrid::GridParseErr,
    },
}

struct Tile {
    block: bool,
    prev: Option<(usize, Dir)>,
}
