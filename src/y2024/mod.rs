#![allow(refining_impl_trait_internal)]

use std::{error::Error, fmt::Display};

days!(1*);

const YEAR: u16 = 2024;

// -- Extras for 2024 --

/// A trait for solvers for a day of AoC
trait Solution: Sized {
    const DAY: Day;

    /// Parse the input string
    fn parse(input: Vec<u8>) -> Result<Self, impl Error + 'static>;

    /// Solve part 1
    fn part1(&mut self) -> impl Display;

    /// Solve part 2
    fn part2(&mut self) -> impl Display;
}

/// Runs the given solver on its input
fn solve<S: Solution>() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(S::DAY), "Failed to load input");
    let mut solver = whatever!(S::parse(input), "Failed to parse input");
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
    Ok(())
}