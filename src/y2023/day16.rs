use bitvec::BitArr;

use crate::{prelude::*, utils::sgrid::Dir};

day!(16);

type Grid = crate::utils::sgrid::Grid<Tile>;

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let (_, mut grid) = whatever!(Grid::parse(&input, parse_elem), "Failed to parse input");
    println!("Parsed grid of size {}x{}", grid.width(), grid.height());

    println!("Part 1: {}", part1(&mut grid));
    println!("Part 2: {}", part2(&mut grid));

    Ok(())
}

fn part2(grid: &mut Grid) -> usize {
    fn inner(
        grid: &mut Grid,
        max: &mut usize,
        dir: Dir,
        indices: impl IntoIterator<Item = usize>,
    ) {
        // For each starting point
        for idx in indices {
            // Clear state
            grid.iter_mut().for_each(|t| t.power.clear());

            // Run simulation
            let val = solve(grid, idx, dir);
            if val > *max {
                *max = val;
            }
        }
    }

    let mut max = 0_usize;
    let width = grid.width();
    let len = grid.len();

    // Top row
    inner(grid, &mut max, Dir::Down, 0..width);

    // Bottom row
    inner(grid, &mut max, Dir::Up, (len - width)..len);

    // Left row
    inner(grid, &mut max, Dir::Right, (0..len).step_by(width));

    // Right row
    inner(grid, &mut max, Dir::Left, (width - 1..len).step_by(width));

    max
}

fn part1(grid: &mut Grid) -> usize {
    solve(grid, 0, Dir::Right)
}

fn solve(grid: &mut Grid, start_idx: usize, start_dir: Dir) -> usize {
    use Dir::*;

    let mut beams = vec![Beam::new(start_idx, start_dir)];

    while let Some(Beam { idx, dir }) = beams.pop() {
        // Retrieve tile the beam is on
        let Some(tile) = grid.get_mut(idx) else {
            continue;
        };

        // Mark tile as powered
        if tile.power.set(dir) {
            continue;
        }

        // Determine next direction for the beam
        let cur_dir = &[dir];
        let dirs: &[Dir] = match tile.kind {
            Kind::Empty => cur_dir,
            Kind::LeftMirror => match dir {
                Up => &[Right],
                Down => &[Left],
                Left => &[Down],
                Right => &[Up],
            },
            Kind::RightMirror => match dir {
                Up => &[Left],
                Down => &[Right],
                Left => &[Up],
                Right => &[Down],
            },
            Kind::VertSplit => match dir {
                Up | Down => cur_dir,
                Left | Right => &[Up, Down],
            },
            Kind::HorizSplit => match dir {
                Left | Right => cur_dir,
                Up | Down => &[Left, Right],
            },
        };

        // Process directions
        for &dir in dirs {
            if let Some(idx) = grid.dir_index(idx, dir) {
                beams.push(Beam { idx, dir });
            }
        }
    }

    grid.iter().filter(|x| x.power.any_set()).count()
}

struct Beam {
    idx: usize,
    dir: Dir,
}

impl Beam {
    pub fn new(idx: usize, dir: Dir) -> Self {
        Self { idx, dir }
    }
}

fn parse_elem(b: u8) -> Option<Tile> {
    use Kind::*;
    let kind = match b {
        b'.' => Empty,
        b'/' => LeftMirror,
        b'\\' => RightMirror,
        b'|' => VertSplit,
        b'-' => HorizSplit,
        _ => return None,
    };

    Some(Tile {
        kind,
        power: Power::default(),
    })
}

struct Tile {
    kind: Kind,
    power: Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Empty,
    LeftMirror,
    RightMirror,
    VertSplit,
    HorizSplit,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Power {
    inner: BitArr!(for 4, in u8),
}

impl Power {
    pub fn set(&mut self, dir: Dir) -> bool {
        self.inner.replace(dir as usize, true)
    }

    pub fn clear(&mut self) {
        self.inner.fill(false);
    }

    pub fn any_set(&self) -> bool {
        self.inner.any()
    }
}
