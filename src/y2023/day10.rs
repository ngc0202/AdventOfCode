use std::collections::VecDeque;

use crate::prelude::*;

day!(10);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let grid = whatever!(parser::parse_grid(&input), "Failed to parse input");
    let mut dists = distances(&grid);

    let part1 = dists.iter().flat_map(|s| s.dist).max().unwrap_or(0);
    println!("Part 1: {part1}");

    let part2 = enclosed(&grid, &mut dists);
    println!("Part 2: {part2}");

    Ok(())
}

#[derive(Clone, Default)]
struct State {
    dist: Option<usize>,
    visited: bool,
}

fn distances(grid: &Grid) -> Box<[State]> {
    // Init state
    let mut queue = VecDeque::<usize>::new();
    let mut states = vec![State::default(); grid.nodes.len()].into_boxed_slice();

    // Handle start
    let start = grid.start;
    states[start].dist = Some(0);
    queue.push_back(start);

    // Process queue
    while let Some(node) = queue.pop_front() {
        let state = &mut states[node];
        state.visited = true;
        let dist = state.dist.expect("Missing dist").saturating_add(1);

        // Update neighbors
        let tile = grid.nodes[node];
        for &dir in tile.conns() {
            if let Some(nbr) = grid.try_traverse(node, dir) {
                let nstate = &mut states[nbr];
                if !nstate.visited {
                    nstate.dist = Some(dist);
                    queue.push_back(nbr);
                }
            }
        }
    }

    states
}

fn enclosed(grid: &Grid, dists: &mut [State]) -> u64 {
    let iter = grid.nodes.iter().zip(dists).chunks(grid.width);

    let mut count = 0u64;
    for row in &iter {
        let mut inner = false;
        let mut prev = None;
        for (&node, state) in row {
            state.visited = true;
            if state.dist.is_some() {
                use Tile::*;
                match (node, prev) {
                    (Start, _) => prev = Some(SouthEast),
                    (NorthSouth, _)
                    | (NorthWest, Some(SouthEast))
                    | (SouthWest, Some(NorthEast)) => inner ^= true,
                    (NorthEast | SouthEast, _) => prev = Some(node),
                    _ => (),
                }
            } else if inner {
                count += 1;
                state.visited = false;
            }
        }
    }

    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Ground,
    Start,
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

type Coord = [isize; 2];

impl Tile {
    pub const fn conns(&self) -> &'static [Coord] {
        const N: Coord = [0, -1];
        const S: Coord = [0, 1];
        const W: Coord = [-1, 0];
        const E: Coord = [1, 0];

        match self {
            Tile::Ground => &[],
            Tile::Start => &[N, S, W, E],
            Tile::NorthSouth => &[N, S],
            Tile::WestEast => &[W, E],
            Tile::NorthEast => &[N, E],
            Tile::NorthWest => &[N, W],
            Tile::SouthWest => &[S, W],
            Tile::SouthEast => &[S, E],
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Failed to parse tile from byte {byte:#04b}"))]
struct TileParseError {
    byte: u8,
}

impl TryFrom<u8> for Tile {
    type Error = TileParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Tile::*;
        Ok(match value {
            b'.' => Ground,
            b'S' => Start,
            b'|' => NorthSouth,
            b'-' => WestEast,
            b'L' => NorthEast,
            b'J' => NorthWest,
            b'7' => SouthWest,
            b'F' => SouthEast,
            _ => return Err(TileParseError { byte: value }),
        })
    }
}

#[derive(Debug)]
struct Grid {
    nodes: Box<[Tile]>,
    start: usize,
    width: usize,
}

impl Grid {
    pub fn try_traverse(&self, loc: usize, dir: Coord) -> Option<usize> {
        let width = self.width as isize;
        let diff = dir[1] * width + dir[0];
        let negdir = [-dir[0], -dir[1]];
        loc.checked_add_signed(diff)
            .filter(|&l| l < self.nodes.len() && self.nodes[l].conns().contains(&negdir))
    }
}

mod parser {
    use super::{Grid, Tile};
    use crate::utils::{parser::line, NomFail};
    use nom::{
        combinator::{all_consuming, map, map_opt, map_res, success},
        multi::{fold_many_m_n, many0_count, many1_count},
        number::complete::u8 as take_byte,
        Finish, IResult,
    };

    pub fn parse_grid(input: &[u8]) -> Result<Grid, NomFail> {
        Ok(parse_grid_(input).finish()?.1)
    }

    fn parse_grid_(input: &[u8]) -> IResult<&[u8], Grid> {
        // Tile parser
        let mut ptile = map_res(take_byte, Tile::try_from);

        // State
        let mut nodes = Vec::new();
        let mut start = None;

        // Appending tile line parser
        let mut add_tile = |t: Tile| {
            let idx = nodes.len();
            nodes.push(t);
            if let Tile::Start = t {
                start = Some(idx);
            }
        };

        // Parse first line and find width
        let (input, width) = line(many1_count(map(&mut ptile, &mut add_tile)))(input)?;

        // Parse rest of input
        let pline = line(fold_many_m_n(
            width,
            width,
            &mut ptile,
            || (),
            |(), t| add_tile(t),
        ));
        let (input, _) = all_consuming(many0_count(pline))(input)?;

        // Verify start
        let (_, start) = map_opt(success(()), |()| start)(&[][..])?;

        let grid = Grid {
            nodes: nodes.into_boxed_slice(),
            start,
            width,
        };

        Ok((input, grid))
    }
}
