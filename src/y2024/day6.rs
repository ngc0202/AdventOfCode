use std::collections::HashSet;

use crate::{prelude::*, utils::sgrid::Dir};

use super::Solution;

day!(run 6);

type Grid = crate::utils::sgrid::Grid<Tile>;

struct Day6 {
    grid: Grid,
    start: usize,
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

            Some(Tile {
                block,
                visit: false,
            })
        })?;

        let [x, y] = start.context(NoStartSnafu)?;
        let start = y * grid.width() + x;

        Ok(Self { grid, start })
    }

    fn part1(&mut self) -> u64 {
        let mut idx = self.start;
        let mut dir = Dir::Up;
        let mut count = 0;

        loop {
            // Mark position as visited
            let seen = set(&mut self.grid[idx].visit);
            if !seen {
                count += 1;
            }

            // Step
            idx = match take_step(&self.grid, idx, &mut dir) {
                Some(i) => i,
                None => return count,
            };
        }
    }

    fn part2(&mut self) -> u64 {
        let mut count = 0;
        let mut seen = HashSet::new();

        for block_idx in 0..self.grid.len() {
            // Skip start and existing blocks
            let tile = &mut self.grid[block_idx];
            if block_idx == self.start || !tile.visit || set(&mut tile.block) {
                continue;
            }

            // Take steps checking for loops
            let mut idx = self.start;
            let mut dir = Dir::Up;

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
                }
            }

            // Reset block and seen
            self.grid[block_idx].block = false;
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

    panic!("Got stuck in one spot!")
}

const fn set(b: &mut bool) -> bool {
    std::mem::replace(b, true)
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
    visit: bool,
}
