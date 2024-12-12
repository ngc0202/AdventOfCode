#![allow(refining_impl_trait_internal)]

use std::{
    error::Error,
    fmt::Display,
    time::{Duration, Instant},
};

use crate::utils::input_path;

days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11*);

const YEAR: u16 = 2024;

// -- Extras for 2024 --

/// A trait for solvers for a day of AoC
trait Solution: Sized {
    const DAY: Day;

    /// Parse the input string
    fn parse(input: &mut Vec<u8>) -> Result<Self, impl Error + 'static>;

    /// Solve part 1
    fn part1(&mut self) -> impl Display {
        "<unimplemented>"
    }

    /// Solve part 2
    fn part2(&mut self) -> impl Display {
        "<unimplemented>"
    }
}

/// Runs the given solver on its input
fn solve<S: Solution>() -> Result<(), Whatever> {
    // Load input
    let path = input_path(S::DAY);
    println!("Reading from {}", path.display());
    let (dur, mut input) = whatever!(try_timeit(|| std::fs::read(&path)), "Failed to load input");
    println!("Loaded input file in {dur:?}");

    // Parse input
    let (dur, mut solver) = whatever!(try_timeit(|| S::parse(&mut input)), "Failed to parse input");
    println!("Parsed input in {dur:?}");

    // Part 1
    {
        let (dur, ans) = timeit(|| solver.part1());
        println!("Part 1 ({dur:?}): {ans}");
    }

    // Part 2
    {
        let (dur, ans) = timeit(|| solver.part2());
        println!("Part 2 ({dur:?}): {ans}");
    }

    Ok(())
}

fn timeit<R, F: FnOnce() -> R>(f: F) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();
    let dur = start.elapsed();
    (dur, ret)
}

fn try_timeit<T, E, F>(f: F) -> Result<(Duration, T), E>
where
    F: FnOnce() -> Result<T, E>,
{
    let (dur, res) = timeit(f);
    Ok((dur, res?))
}
