use std::mem;

use crate::utils::{sgrid::{Dir, GridParseErr}, Pair};

use super::Solution;

day!(run 10);

type Grid = crate::utils::sgrid::Grid<u8>;

struct Day10 {
    grid: Grid,
}

impl<'i> Solution<'i> for Day10 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, GridParseErr> {
        let (_, grid) = Grid::parse(input, |b| b.is_ascii_digit().then_some(b - b'0'))?;
        Ok(Self { grid })
    }

    fn part1(&mut self) -> Pair<usize, usize> {
        let mut stack = Vec::new();
        let mut seen = vec![false; self.grid.len()];
        let mut count1 = 0; // Part 1
        let mut count2 = 0; // Part 2

        for start_idx in 0..self.grid.len() {
            // Check for peak
            if self.grid[start_idx] != 9 {
                continue;
            }

            stack.push(Item {
                idx: start_idx,
                height: 9,
            });

            // Take steps
            while let Some(Item { idx, height }) = stack.pop() {
                // Check height and seen
                if self.grid[idx] != height {
                    continue;
                }

                // Check trailhead
                if let Some(next_hgt) = height.checked_sub(1) {
                    // Continue steps
                    for dir in Dir::ALL {
                        if let Some(next_idx) = self.grid.dir_index(idx, dir) {
                            stack.push(Item {
                                idx: next_idx,
                                height: next_hgt,
                            });
                        }
                    }
                } else {
                    // Found it
                    count2 += 1;
                    if !mem::replace(&mut seen[idx], true) {
                        count1 += 1;
                    }
                }
            }

            // Reset seen
            seen.iter_mut().for_each(|s| *s = false);
        }

        Pair(count1, count2)
    }
}

struct Item {
    idx: usize,
    height: u8,
}
