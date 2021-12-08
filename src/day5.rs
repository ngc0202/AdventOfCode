
use crate::prelude::*;

use std::iter;
use std::cmp::Ordering::*;
use std::ops::RangeInclusive;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};


const DAY: u8 = 5;


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct LineSeg {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl LineSeg {	
	pub fn is_diag(&self) -> bool {
		(self.x1 != self.x2) && (self.y1 != self.y2)
	}
	
	pub fn iter_pts(&self) -> Box<dyn Iterator<Item=(u16, u16)>> {
		let &LineSeg { x1, y1, x2, y2 } = self;
		match (x1.cmp(&x2), y1.cmp(&y2)) {
			(Equal, Equal) => Box::new(iter::once((x1, y1))),
			(Equal, Less|Greater) => Box::new(iter::repeat(x1).zip(Self::make_range(y1, y2))),
			(Less|Greater, Equal) => Box::new(Self::make_range(x1, x2).zip(iter::repeat(y1))),
			(Less, Less) | (Greater, Greater) => Box::new(Self::make_range(x1, x2).zip(Self::make_range(y1, y2))),
			_ => Box::new(Self::make_range(x1, x2).rev().zip(Self::make_range(y1, y2)))
		}
	}
	
	fn make_range<T: PartialOrd>(a: T, b: T) -> RangeInclusive<T> {
		if a <= b {
			a..=b
		} else {
			b..=a
		}
	}
	
	pub fn as_tuples(&self) -> ((u16, u16), (u16, u16)) {
		((self.x1, self.y1), (self.x2, self.x2))
	}
}

impl Display for LineSeg {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let &LineSeg { x1, y1, x2, y2 } = self;
		write!(f, "({}, {}) -> ({}, {})", x1, y1, x2, y2)
	}
}

impl From<(u16, u16, u16, u16)> for LineSeg {
    fn from((x1, y1, x2, y2): (u16, u16, u16, u16)) -> Self {
        LineSeg { x1, y1, x2, y2 }
    }
}

impl FromStr for LineSeg {
    type Err = ();

    // panics instead of err
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(s.split(" -> ")
            .flat_map(|t| t.split(','))
            .map(|pt| pt.parse::<u16>().unwrap())
            .collect_tuple::<(u16, u16, u16, u16)>()
            .unwrap()
            .into())
    }
}

const WIDTH: usize = 1000;

fn print_grid(grid: &[u16]) {
    println!(
        "Grid:\n{}",
        grid.into_iter()
            .copied()
            .chunks(WIDTH)
            .into_iter()
            .format_with("\n", |chunk, f| f(
                &chunk.format_with(" ", |v, g| g(&format_args!("{}", v)))
            ))
    )
}

fn print_counts(counts: &HashMap<usize, usize>) {
	let grid: Vec<u16> = (0..WIDTH*WIDTH).map(|i| counts.get(&i).copied().unwrap_or(0) as u16).collect();
	print_grid(&grid);
}

fn count_overlaps(diagonals: bool) -> u64 {
	crate::load_input(DAY)
		.unwrap()
        .lines()
        .map(|l| l.unwrap().parse::<LineSeg>().unwrap())
		.filter(|s| diagonals || !s.is_diag())
        .flat_map(|s| s.iter_pts())
        .map(|(x, y)| WIDTH * usize::from(y) + usize::from(x))
		.counts()
		.into_iter()
        .filter_map(|(_, c)| (c >= 2).then(|| 1))
        .sum()
}

pub fn run() -> GenResult {
	
	let part1 = count_overlaps(false);
    println!("Part 1: {}", part1);

    let part2: u64 = count_overlaps(true);
	println!("Part 2: {}", part2);

    Ok(())
}
