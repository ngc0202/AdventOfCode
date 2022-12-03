#[allow(dead_code)]
mod grid;
pub use grid::Grid;

mod wrap;
use itertools::{process_results, ProcessResults};
pub use wrap::WrapUsize;

// mod zorder;
// pub use zorder::ZOrderIter;

use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
    path::PathBuf,
};

use crate::get_small;

pub fn load_input(Day { day, year }: Day) -> io::Result<BufReader<File>> {
    let filename = if get_small() {
        format!("input{day:02}_small.txt")
    } else {
        format!("input{day:02}.txt")
    };

    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(format!("{}", year));
    path.push(filename);
    println!("Reading from {}", path.display());
    File::open(&path).map(BufReader::new)
}

pub fn load_input_string(day: Day) -> io::Result<String> {
    let mut input = String::new();
    load_input(day)?.read_to_string(&mut input)?;
    Ok(input)
}

pub fn process_inputs<F, R>(day: Day, f: F) -> io::Result<R>
where
    F: FnOnce(ProcessResults<Lines<BufReader<File>>, io::Error>) -> R,
{
    process_results(load_input(day)?.lines(), f)
}

#[derive(Debug)]
pub struct NoInputError;

impl fmt::Display for NoInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No input provided")
    }
}

impl std::error::Error for NoInputError {}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Day {
    pub day: u8,
    pub year: u16,
}
