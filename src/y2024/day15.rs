use super::Solution;
use crate::utils::sgrid::{Dir, Grid};

day!(run 15);

struct Day15 {
    grid1: Grid<Tile1>,
    grid2: Grid<Tile2>,
    robot: usize,
    moves: Vec<Dir>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile1 {
    Empty,
    Wall,
    Box,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile2 {
    Empty,
    Wall,
    LBox,
    RBox,
}

type Stack = indexmap::IndexMap<usize, usize>;

impl<'i> Solution<'i> for Day15 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, parse::Error15> {
        parse::parse(input)
    }

    fn part1(&mut self) -> usize {
        let start_idx = self.robot;
        for i in 0..self.moves.len() {
            self.move_robot_p1(self.moves[i]);
        }

        self.robot = start_idx;
        score(&self.grid1, &Tile1::Box)
    }

    fn part2(&mut self) -> usize {
        self.robot *= 2;
        let mut stack = Stack::new();
        for i in 0..self.moves.len() {
            self.move_robot_p2(self.moves[i], &mut stack);
        }
        score(&self.grid2, &Tile2::LBox)
    }
}

fn score<T: PartialEq>(grid: &Grid<T>, elem: &T) -> usize {
    let w = grid.width();
    grid.iter()
        .enumerate()
        .filter(|&(_, t)| t == elem)
        .map(|(i, _)| (i / w * 100) + (i % w))
        .sum()
}

// Part 1 methods
impl Day15 {
    fn move_robot_p1(&mut self, dir: Dir) {
        let next_idx = self.grid1.dir_index(self.robot, dir).unwrap();
        let mut idx = next_idx;
        loop {
            idx = match self.grid1[idx] {
                Tile1::Empty => break,
                Tile1::Wall => return,
                Tile1::Box => self.grid1.dir_index(idx, dir).unwrap(),
            }
        }

        if idx != next_idx {
            self.grid1[next_idx] = Tile1::Empty;
            self.grid1[idx] = Tile1::Box;
        }

        self.robot = next_idx;
    }
}

// Part 2 methods
impl Day15 {
    fn move_robot_p2(&mut self, dir: Dir, stack: &mut Stack) {
        stack.clear();
        let next_idx = self.grid2.dir_index(self.robot, dir).unwrap();
        let can_move = self.can_move(next_idx, dir, stack);
        if can_move {
            self.sort_stack(dir, stack);
            self.do_moves(stack);
            self.robot = next_idx;
        }
    }

    fn sort_stack(&self, dir: Dir, stack: &mut Stack) {
        stack.sort_unstable_by(|a, _, b, _| match dir {
            Dir::Right | Dir::Down => a.cmp(b),
            Dir::Left | Dir::Up => b.cmp(a),
        });
    }

    fn can_move(&self, idx: usize, dir: Dir, stack: &mut Stack) -> bool {
        if stack.contains_key(&idx) {
            return true;
        }

        // Return whether clear or for box, get idx to second half
        let oidx = match self.grid2[idx] {
            Tile2::Empty => return true,
            Tile2::Wall => return false,
            Tile2::LBox => idx + 1,
            Tile2::RBox => idx - 1,
        };

        let nidx = self.grid2.dir_index(idx, dir).unwrap();
        let onidx = self.grid2.dir_index(oidx, dir).unwrap();

        stack.extend([(idx, nidx), (oidx, onidx)]);

        self.can_move(nidx, dir, stack) && self.can_move(onidx, dir, stack)
    }

    fn do_moves(&mut self, stack: &mut Stack) {
        for (idx, nidx) in stack.drain(..).rev() {
            self.grid2.swap(idx, nidx);
        }
    }
}

// Doubles the width of every tile
fn grow(grid: &Grid<Tile1>) -> Grid<Tile2> {
    let mut elems = Vec::with_capacity(grid.len() * 2);
    for &tile in grid.iter() {
        let upd = match tile {
            Tile1::Empty => [Tile2::Empty; 2],
            Tile1::Wall => [Tile2::Wall; 2],
            Tile1::Box => [Tile2::LBox, Tile2::RBox],
        };
        elems.extend_from_slice(&upd);
    }

    Grid::new(elems, 2 * grid.width())
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::{all_consuming, opt, value},
        multi::many1,
        sequence::preceded,
        Finish, IResult,
    };
    use snafu::{OptionExt, ResultExt, Snafu};

    use super::{Day15, Grid, Tile1};
    use crate::utils::{
        parser::line,
        sgrid::{Dir, GridParseErr},
        Coord, NomFail,
    };

    #[allow(clippy::type_complexity)]
    fn grid(input: &[u8]) -> Result<(&[u8], (Grid<Tile1>, usize)), Error15> {
        let mut robot = None;
        let (i, grid) = Grid::parse_co(input, |b, x, y| match b {
            b'#' => Some(Tile1::Wall),
            b'O' => Some(Tile1::Box),
            b'.' => Some(Tile1::Empty),
            b'@' => {
                robot = Some(Coord { x, y });
                Some(Tile1::Empty)
            }
            _ => None,
        })
        .context(GridSnafu)?;

        let robot = robot.context(NoRobotSnafu)?.to_idx(grid.width());
        Ok((i, (grid, robot)))
    }

    fn dirs(input: &[u8]) -> IResult<&[u8], Vec<Dir>> {
        many1(preceded(
            opt(line_ending),
            alt((
                value(Dir::Up, tag("^")),
                value(Dir::Right, tag(">")),
                value(Dir::Down, tag("v")),
                value(Dir::Left, tag("<")),
            )),
        ))(input)
    }

    pub fn parse(input: &[u8]) -> Result<Day15, Error15> {
        let (input, (grid1, robot)) = grid(input)?;
        let (_, moves) = all_consuming(line(dirs))(input)
            .finish()
            .map_err(NomFail::from)
            .context(DirsSnafu)?;
        let grid2 = super::grow(&grid1);
        Ok(Day15 {
            grid1,
            grid2,
            robot,
            moves,
        })
    }

    #[derive(Debug, Snafu)]
    pub enum Error15 {
        #[snafu(display("Failed to parse grid"))]
        Grid { source: GridParseErr },
        #[snafu(display("Failed to parse directions"))]
        Dirs { source: NomFail },
        #[snafu(display("No robot found in grid"))]
        NoRobot,
    }
}
