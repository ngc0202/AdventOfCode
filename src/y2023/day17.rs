use std::{cmp::Ordering, collections::BinaryHeap};

use bitvec::BitArr;

use crate::prelude::*;

day!(17);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let grid = whatever!(Grid::parse(&input), "Failed to parse input");

    let part1 = grid.find_path(0, 3);
    println!("Part 1: {part1}");

    let part2 = grid.find_path(4, 10);
    println!("Part 2: {part2}");

    Ok(())
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    nodes: Box<[T]>,
}

#[derive(Debug, Clone, Default)]
struct DState {
    dist: Option<u64>,
    steps: BitArr!(for 64),
}

#[derive(Debug, Clone, Default)]
struct PerDir<T> {
    north: T,
    south: T,
    east: T,
    west: T,
}

impl<T> std::ops::Index<Dir> for PerDir<T> {
    type Output = T;

    fn index(&self, dir: Dir) -> &T {
        match dir {
            Dir::North => &self.north,
            Dir::South => &self.south,
            Dir::East => &self.east,
            Dir::West => &self.west,
        }
    }
}

impl<T> std::ops::IndexMut<Dir> for PerDir<T> {
    fn index_mut(&mut self, dir: Dir) -> &mut T {
        match dir {
            Dir::North => &mut self.north,
            Dir::South => &mut self.south,
            Dir::East => &mut self.east,
            Dir::West => &mut self.west,
        }
    }
}

impl Grid<u8> {
    pub fn find_path(&self, min_steps: u8, max_steps: u8) -> u64 {
        if self.nodes.is_empty() {
            return 0;
        }

        let nodes = &self.nodes[..];

        let mut dists: Box<[PerDir<DState>]> =
            vec![Default::default(); self.nodes.len()].into_boxed_slice();

        {
            let lastd = dists.last_mut().unwrap();
            lastd.north = Default::default();
            lastd.west = Default::default();
        }

        let mut heap = BinaryHeap::new();
        let last_pos = dists.len() - 1;
        heap.push(State {
            position: last_pos,
            cost: 0,
            extra: DirState {
                dir: Dir::North,
                steps: 0,
            },
        });

        heap.push(State {
            position: last_pos,
            cost: 0,
            extra: DirState {
                dir: Dir::West,
                steps: 0,
            },
        });

        while let Some(State {
            cost,
            position,
            extra,
        }) = heap.pop()
        {
            let next_cost = cost.saturating_add(u64::from(nodes[position]));

            // Check adjacencies
            let mut check_adj = |adj: usize, adj_dir: Dir| {
                if let Some(extra) = extra.adv_dir(adj_dir, min_steps, max_steps) {
                    let dstate = &mut dists[adj][adj_dir];
                    let stepn = usize::from(extra.steps);
                    if !dstate.steps[stepn] {
                        dstate.steps.set(stepn, true);

                        if extra.steps >= min_steps {
                            dstate.dist = Some(match dstate.dist {
                                Some(d) => d.min(next_cost),
                                None => next_cost,
                            });
                        }

                        let next = State {
                            cost: next_cost,
                            position: adj,
                            extra,
                        };

                        heap.push(next);
                    }
                }
            };

            // Check north
            if let Some(next_pos) = position.checked_sub(self.width) {
                check_adj(next_pos, Dir::North);
            }

            // Check south
            if let Some(next_pos) = position
                .checked_add(self.width)
                .filter(|&x| x < nodes.len())
            {
                check_adj(next_pos, Dir::South);
            }

            let rem = position % self.width;

            // Check east
            if rem + 1 < self.width {
                check_adj(position + 1, Dir::East);
            }

            // Check west
            if rem > 0 {
                check_adj(position - 1, Dir::West);
            }
        }

        let min = Dir::ALL.iter().filter_map(|&d| dists[0][d].dist).min();
        min.expect("Target node was never reached")
    }

    fn parse(input: &[u8]) -> Result<Self, BadChar> {
        let mut nodes = Vec::new();
        let mut width = 0_usize;
        let mut first_done = false;
        for &byte in input {
            match byte {
                b'0'..=b'9' => {
                    nodes.push(byte - b'0');
                    if !first_done {
                        width += 1;
                    }
                }
                b'\r' | b'\n' => {
                    if !first_done {
                        first_done = true;
                    }
                }
                _ => return Err(BadChar { val: byte }),
            }
        }

        Ok(Self {
            width,
            nodes: nodes.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Encountered invalid char {val:#04x}"))]
struct BadChar {
    val: u8,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub const ALL: [Self; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

    pub fn reverse(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

struct DirState {
    dir: Dir,
    steps: u8,
}

impl DirState {
    pub fn adv_dir(&self, dir: Dir, min_steps: u8, max_steps: u8) -> Option<Self> {
        if dir == self.dir {
            (self.steps < max_steps).then_some(Self {
                dir,
                steps: self.steps + 1,
            })
        } else if dir == self.dir.reverse() {
            None
        } else {
            (self.steps >= min_steps).then_some(Self { dir, steps: 1 })
        }
    }
}

struct State<E> {
    cost: u64,
    position: usize,
    extra: E,
}

impl<E> Eq for State<E> {}

impl<E> PartialEq for State<E> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

impl<E> Ord for State<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl<E> PartialOrd for State<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
