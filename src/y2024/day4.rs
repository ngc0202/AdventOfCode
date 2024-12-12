use crate::{
    prelude::*,
    utils::sgrid::{Dir, GridParseErr},
};

use super::Solution;

day!(run 4);

type Grid = crate::utils::sgrid::Grid<u8>;

struct Day4 {
    grid: Grid,
}

impl Solution for Day4 {
    const DAY: Day = DAY;

    fn parse(input: &mut Vec<u8>) -> Result<Self, GridParseErr> {
        let (_, grid) = Grid::parse(input, |b| match b {
            b'X' | b'M' | b'A' | b'S' => Some(b),
            _ => None,
        })?;

        Ok(Self { grid })
    }

    fn part1(&mut self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'X')
            .cartesian_product([Some(Dir::Up), Some(Dir::Down), None])
            .cartesian_product([Some(Dir::Left), Some(Dir::Right), None])
            .filter(|&(((idx, _), vert), horiz)| check_dir(&self.grid, idx, vert, horiz))
            .count()
    }

    fn part2(&mut self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|&(i, &c)| c == b'A' && is_xmas(&self.grid, i))
            .count()
    }
}

/// Checks if MAS is spelled in the given direction from `idx`
fn check_dir(grid: &Grid, mut idx: usize, vert: Option<Dir>, horiz: Option<Dir>) -> bool {
    // Don't allow no movement
    if vert.is_none() && horiz.is_none() {
        return false;
    }

    // Move the index in the given directions
    let mut do_dir = |dir: Option<Dir>| {
        if let Some(dir) = dir {
            match grid.dir_index(idx, dir) {
                Some(i) => idx = i,
                None => return None,
            }
        }
        Some(idx)
    };

    // Search for each letter in the given direction
    for next in [b'M', b'A', b'S'] {
        let valid = do_dir(vert)
            .and_then(|_| do_dir(horiz))
            .is_some_and(|i| grid[i] == next);
        if !valid {
            return false;
        }
    }

    true
}

/// Checks if the given index is the center of a doubly crossed MAS
fn is_xmas(grid: &Grid, idx: usize) -> bool {
    // Gets the element at the diagonal
    let diag = |vert, horiz| {
        grid.dir_index(idx, vert)
            .and_then(|vidx| grid.dir_index(vidx, horiz))
            .map(|i| grid[i])
    };

    // Check if opposite corners have MS
    let is_cross = move |v, h| {
        matches!(
            [diag(v, h), diag(v.opposite(), h.opposite())],
            [Some(b'M'), Some(b'S')] | [Some(b'S'), Some(b'M')]
        )
    };

    // Check both crosses for MAS
    is_cross(Dir::Up, Dir::Right) && is_cross(Dir::Up, Dir::Left)
}
