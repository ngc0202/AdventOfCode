use super::Solution;
use crate::utils::{
    opt_min,
    sgrid::{Dir, Grid},
    Coord,
};

day!(run 20);

struct Day20 {
    grid: Grid<bool>,
    dists: Box<[Option<usize>]>,
    start: usize,
}

impl<'i> Solution<'i> for Day20 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, parse::Error20> {
        parse::parse(input)
    }

    fn part1(&mut self) -> usize {
        self.calc_dists();
        (0..self.grid.len())
            .map(|i| self.time_saving(i))
            .filter(|&n| n >= 100)
            .count()
    }

    fn part2(&mut self) -> usize {
        let width = self.grid.width();
        self.dists
            .iter()
            .enumerate()
            .filter_map(|(i, d)| d.map(|d| (i, d)))
            .map(|(idx, dist)| {
                self.iter_dist(idx, 20)
                    .filter_map(|(c, diff)| {
                        let d2 = self.dists[c.to_idx(width)]?;
                        Some(dist.saturating_sub(d2 + diff))
                    })
                    .filter(|&n| n >= 100)
                    .count()
            })
            .sum()
    }
}

impl Day20 {
    /// Iterates over the coordinates within `dist` distance from `idx`
    fn iter_dist(&self, idx: usize, dist: usize) -> impl Iterator<Item = (Coord, usize)> {
        let width = self.grid.width();
        let height = self.grid.height();
        let coord = Coord::of_idx(idx, width);

        let ylo = coord.y.saturating_sub(dist);
        let yhi = height.min(coord.y + dist + 1);

        (ylo..yhi).flat_map(move |y| {
            let xdiff = dist - y.abs_diff(coord.y);
            let xlo = coord.x.saturating_sub(xdiff);
            let xhi = width.min(coord.x + xdiff + 1);
            (xlo..xhi).map(move |x| (Coord { x, y }, x.abs_diff(coord.x) + y.abs_diff(coord.y)))
        })
    }

    fn calc_dists(&mut self) {
        let mut idx = self.start;
        let mut dist = 0;
        let mut last_dir = None;
        loop {
            self.dists[idx] = Some(dist);
            dist += 1;
            let mut next = None;
            for dir in Dir::ALL {
                if last_dir.is_none_or(|ld| dir != ld) {
                    if let Some(nidx) = self.grid.dir_index(idx, dir) {
                        if !self.grid[nidx] {
                            next = Some(nidx);
                            last_dir = Some(dir.opposite());
                            break;
                        }
                    }
                }
            }
            match next {
                Some(i) => idx = i,
                None => break,
            }
        }
    }

    fn time_saving(&self, idx: usize) -> usize {
        if !self.grid[idx] {
            return 0;
        }

        let mut min = None;
        for pair in [[Dir::Up, Dir::Down], [Dir::Left, Dir::Right]] {
            let pair = pair.map(|d| self.grid.dir_index(idx, d).and_then(|i| self.dists[i]));
            if let [Some(a), Some(b)] = pair {
                opt_min(&mut min, a.abs_diff(b));
            }
        }

        min.unwrap_or(0).saturating_sub(2)
    }
}

mod parse {
    use super::{Day20, Grid};
    use crate::utils::{sgrid::GridParseErr, Coord};
    use snafu::{OptionExt, ResultExt, Snafu};

    pub fn parse(input: &[u8]) -> Result<Day20, Error20> {
        let mut start = None;
        let grid = Grid::parse_co(input, |b, x, y| {
            Some(match b {
                b'.' => false,
                b'#' => true,
                b'S' => {
                    start = Some(Coord { x, y });
                    false
                }
                _ => return None,
            })
        })
        .context(GridSnafu)?
        .1;

        Ok(Day20 {
            start: start.context(StartSnafu)?.to_idx(grid.width()),
            dists: vec![None; grid.len()].into(),
            grid,
        })
    }

    #[derive(Debug, Snafu)]
    pub enum Error20 {
        #[snafu(display("Failed to parse grid"))]
        Grid { source: GridParseErr },
        #[snafu(display("Start not found in grid"))]
        Start,
    }
}
