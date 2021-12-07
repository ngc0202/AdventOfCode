use crate::GenResult;

use itertools::Itertools;
use std::io::BufRead;
use std::ops::Sub;

const DAY: u8 = 7;

pub fn run() -> GenResult {
    let nums: Vec<u16> = crate::load_input(DAY)?
        .lines()
        .exactly_one()??
        .split(',')
        .map(|s| s.parse())
        .try_collect()?;

    let part1: usize = (0..nums.len())
        .map(|i| 
			nums.iter()
				.map(|&n| abs_diff(i, usize::from(n)))
				.sum())
        .min()
        .unwrap();

    println!("Part 1: {}", part1);

	let part2: usize = (0..nums.len())
        .map(|i| 
			nums.iter()
				.flat_map(|&n| (1..=abs_diff(i, usize::from(n))))
				.sum())
        .min()
        .unwrap();
		
	println!("Part 2: {}", part2);

    Ok(())
}

fn abs_diff<T>(a: T, b: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}
