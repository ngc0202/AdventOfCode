use crate::utils::sgrid::Dir;

use super::Solution;

day!(run 16);

type Grid = crate::utils::sgrid::Grid<Tile>;

struct Day16 {
    grid: Grid,
    start: usize,
    end: usize,
}

struct Tile {
    wall: bool,
    score: Option<u64>,
}

#[derive(Copy, Clone)]
struct State {
    idx: usize,
    score: u64,
    dir: Dir,
}

const TURN_PENALTY: u64 = 1_000;

impl<'i> Solution<'i> for Day16 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, parse::Error16> {
        parse::parse(input)
    }

    fn part1(&mut self) -> u64 {
        let mut stack = vec![State {
            idx: self.start,
            score: 0,
            dir: Dir::Right,
        }];

        while let Some(state) = stack.pop() {
            // Update score for tile, skipping walls or better scores
            let tile = &mut self.grid[state.idx];
            if tile.wall || !tile.update_score(state.score) || state.idx == self.end {
                continue;
            }

            // Continue next steps
            let dirs = [
                (state.dir, 0),
                (state.dir.right(), TURN_PENALTY),
                (state.dir.left(), TURN_PENALTY),
            ];
            for (dir, penalty) in dirs {
                if let Some(next_idx) = self.grid.dir_index(state.idx, dir) {
                    let new_score = state.score + penalty + 1;
                    stack.push(State {
                        idx: next_idx,
                        score: new_score,
                        dir,
                    });
                }
            }
        }

        // Get best score from start
        self.grid[self.end].score.expect("No path to end found")
    }

    fn part2(&mut self) -> usize {
        let mut best = vec![false; self.grid.len()];
        let mut stack = vec![State {
            idx: self.start,
            score: 0,
            dir: Dir::Right,
        }];
        let mut revisit = Vec::<(usize, Dir)>::new();

        while let Some(state) = stack.last().copied() {
            // Checks if index is an intersection
            let intersect = || {
                Dir::ALL
                .into_iter()
                .filter_map(|d| self.grid.dir_index(state.idx, d))
                .filter(|&i| self.grid[i].score.is_some())
                .count() > 2
            };

            // Check if still on best path
            let tscore = self.grid[state.idx].score;
            let isbest = tscore.is_some_and(|ts| state.score <= ts);
            let atend = state.idx == self.end;
            if tscore.is_none() || atend || (!isbest && !intersect()) {
                // If reached end, add all to best
                if atend && isbest {
                    for st in &stack {
                        best[st.idx] = true;
                    }
                }

                // Go to next revisit
                if let Some(revis) = revisit.pop() {
                    let rscore = loop {
                        let pop = stack.last().unwrap();
                        if pop.idx == revis.0 {
                            break pop.score;
                        }
                        stack.pop();
                    };
                    let nidx = self.grid.dir_index(revis.0, revis.1).unwrap();
                    stack.push(State {
                        idx: nidx,
                        score: rscore + TURN_PENALTY + 1,
                        dir: revis.1,
                    });
                } else {
                    break;
                }

                continue;
            }

            // Continue straight
            if let Some(nidx) = self.grid.dir_index(state.idx, state.dir) {
                stack.push(State {
                    idx: nidx,
                    score: state.score + 1,
                    dir: state.dir,
                });
            }

            // Revisit turns
            revisit.extend_from_slice(&[
                (state.idx, state.dir.right()),
                (state.idx, state.dir.left()),
            ]);
        }

        best.into_iter().filter(|&b| b).count()
    }
}

impl Tile {
    pub fn new(wall: bool) -> Self {
        Self { wall, score: None }
    }

    /// Updates the score for this tile to the lower value,
    /// returning whether `n` is the new best
    pub fn update_score(&mut self, n: u64) -> bool {
        match self.score {
            Some(v) if n >= v => false,
            _ => {
                self.score = Some(n);
                true
            }
        }
    }
}

mod parse {
    use super::{Day16, Grid, Tile};
    use crate::utils::{sgrid::GridParseErr, Coord};
    use snafu::{OptionExt, ResultExt, Snafu};

    pub fn parse(input: &[u8]) -> Result<Day16, Error16> {
        let mut start = None;
        let mut end = None;
        let grid = Grid::parse_co(input, |b, x, y| {
            let wall = match b {
                b'.' => false,
                b'#' => true,
                b'S' => {
                    start = Some(Coord { x, y });
                    false
                }
                b'E' => {
                    end = Some(Coord { x, y });
                    false
                }
                _ => return None,
            };
            Some(Tile::new(wall))
        })
        .context(GridSnafu)?
        .1;

        Ok(Day16 {
            start: start.context(NoStartSnafu)?.to_idx(grid.width()),
            end: end.context(NoEndSnafu)?.to_idx(grid.width()),
            grid,
        })
    }

    #[derive(Debug, Snafu)]
    pub enum Error16 {
        #[snafu(display("Failed to parse grid"))]
        Grid { source: GridParseErr },
        #[snafu(display("Missing start in grid"))]
        NoStart,
        #[snafu(display("Missing end in grid"))]
        NoEnd,
    }
}
