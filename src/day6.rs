
use crate::GenResult;
use std::io::BufRead;
use itertools::Itertools;

const DAY: u8 = 6;

#[derive(Clone, Debug, Default)]
struct FishTank {
	fish: [u64; 7],
	head: usize,
	day7: u64,
	day8: u64
}

impl FishTank {
	pub fn tick(&mut self) {
		let babies = self.fish[self.head];
		self.fish[self.head] += self.day7;
		self.head = (self.head + 1) % 7;
		self.day7 = self.day8;
		self.day8 = babies;
	}
	
	pub fn count_fish(&self) -> u64 {
		self.fish.iter().sum::<u64>() + self.day7 + self.day8
	}
	
	pub fn simulate(&mut self, days: usize) -> u64 {
		for _ in 0..days {
			self.tick();
		}
		self.count_fish()
	}
}

impl FromIterator<u8> for FishTank {
	fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
		let mut tank = FishTank::default();
		for n in iter {
			tank.fish[usize::from(n)] += 1;
		}
		tank
	}
}

pub fn run() -> GenResult {
	let line = crate::load_input(DAY)?
	                 .lines()
					 .exactly_one()?
					 .unwrap();

	let mut fish: FishTank =
			line.split(',')
		        .map(|n| n.parse())
		        .try_collect()?;

	let part1 = fish.simulate(80);
	println!("Part 1: {}", part1); // 390011
	
	let part2 = fish.simulate(256-80);
	println!("Part 2: {}", part2);

	Ok(())
}