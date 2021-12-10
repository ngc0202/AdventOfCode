
use crate::prelude::*;
use std::io::Seek;

const DAY: u8 = 9;

#[derive(Clone, Debug)]
struct Cave {
	width: usize,
	heights: Vec<u8>
}

impl Cave {
	pub fn new(width: usize, heights: Vec<u8>) -> Cave {
		Cave { width, heights }
	}
}



pub fn run() -> GenResult {
	let cave: Cave = {
		let mut input = load_input(DAY)?;
		let width = {
			let mut line1 = String::new();
			input.read_line(&mut line1)?;
			input.rewind()?;
			line1.chars().count()
		};
		let heights = input.lines()
			.flat_map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap() as u8))
			.collect_vec();
		Cave { width, heights }
	};

	Ok(())
}
