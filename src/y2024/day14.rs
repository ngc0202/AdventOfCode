use crate::utils::{Coord, sgrid::Grid, NomFail};
use super::Solution;
use std::{io::Write, sync::atomic::{AtomicBool, Ordering}};

day!(run 14);

struct Day14 {
    robots: Vec<Robot>,
    width: u8,
    height: u8,
}

struct Robot {
    pos: Coord<u8>,
    vel: Coord<i8>,
}

impl<'i> Solution<'i> for Day14 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        let (w, h) = if crate::get_small() {
            (11, 7)
        } else {
            (101, 103)
        };

        let robots = parse::parse(input)?;

        Ok(Self {
            robots,
            width: w,
            height: h,
        })
    }

    fn part1(&mut self) -> usize {
        let mut quads = [0usize; 4];
        for robot in &self.robots {
            let c = self.roam(robot, 100);
            self.assign_quad(&c, &mut quads);
        }
        quads.into_iter().product()
    }

    fn part2(&mut self) -> u64 {
        use rayon::prelude::*;

        let width = usize::from(self.width);
        let size = width * usize::from(self.height);
        let vec = (0..size).map(|_| AtomicBool::default()).collect();
        let grid = Grid::<AtomicBool>::new(vec, width);
        let mut input = String::new();
        let stdin = &mut std::io::stdin();
        let stdout = &mut std::io::stdout().lock();

        let square = |b: &AtomicBool| if b.load(Ordering::Relaxed) { '\u{25A0}' } else { ' ' };

        for i in 1u64.. {
            // Reset grid
            grid.par_iter().for_each(|b| b.store(false, Ordering::Relaxed));

            // Take step
            let ret = self.robots.par_iter_mut().try_for_each(|robot| {
                let c = robot.step(self.width, self.height).map(usize::from);
                if grid[c].swap(true, Ordering::Relaxed) {
                    return None;
                }
                Some(())
            });

            if ret.is_none() {
                continue
            }

            // Check if tree
            writeln!(stdout, "Step {i}: {}", grid.display_with(square)).unwrap();
            write!(stdout, "\nStop? ").unwrap();
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).expect("stdin fail");
            if input.trim().eq_ignore_ascii_case("stop") {
                return i;
            }
        }

        unreachable!()
    }
}

impl Day14 {
    /// Returns the coordinates of `robot` after roaming for `secs`
    pub fn roam(&self, robot: &Robot, secs: u32) -> Coord<u8> {
        let secs: i64 = secs.into();
        let vel = robot.vel.map(i64::from);
        let mut pos = robot.pos.map(i64::from);
        pos.x = (pos.x + (vel.x * secs)).rem_euclid(i64::from(self.width));
        pos.y = (pos.y + (vel.y * secs)).rem_euclid(i64::from(self.height));
        pos.map(|n| n as u8)
    }

    pub fn assign_quad(&self, coord: &Coord<u8>, quads: &mut [usize; 4]) {
        let midx = self.width / 2;
        let midy = self.height / 2;
        use std::cmp::Ordering::*;
        match (coord.x.cmp(&midx), coord.y.cmp(&midy)) {
            (Greater, Greater) => quads[0] += 1,
            (Less, Greater) => quads[1] += 1,
            (Less, Less) => quads[2] += 1,
            (Greater, Less) => quads[3] += 1,
            _ => (),
        }
    }
}

impl Robot {
    /// Takes one step
    pub fn step(&mut self, width: u8, height: u8) -> Coord<u8> {
        self.pos.x = self.pos.x.wrapping_add_signed(self.vel.x) % width;
        self.pos.y = self.pos.y.wrapping_add_signed(self.vel.y) % height;
        self.pos
    }
}

mod parse {
    use super::Robot;
    use crate::utils::{
        parser::{line, NomInt},
        Coord, NomFail,
    };
    use nom::{
        bytes::complete::tag,
        combinator::all_consuming,
        multi::many1,
        sequence::{preceded, separated_pair},
        Finish, IResult, Parser,
    };

    fn coord<T: NomInt>(input: &[u8]) -> IResult<&[u8], Coord<T>> {
        separated_pair(T::parse_int, tag([b',']), T::parse_int)
            .map(|(x, y)| Coord { x, y })
            .parse(input)
    }

    fn robot(input: &[u8]) -> IResult<&[u8], Robot> {
        line(separated_pair(
            preceded(tag("p="), coord),
            tag(" v="),
            coord,
        ))
        .map(|(pos, vel)| Robot { pos, vel })
        .parse(input)
    }

    pub fn parse(input: &[u8]) -> Result<Vec<Robot>, NomFail> {
        Ok(all_consuming(many1(robot))(input).finish()?.1)
    }
}
