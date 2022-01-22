#[macro_use]
mod macros;

#[allow(dead_code)]
mod utils;

use crate::prelude::*;
use paste::paste;

// Main Method
days!(4, 5, 6, 7, 8, 9, 10, 14*, 16, 21);

mod prelude {

    pub type GenError = Box<dyn Error>;
    pub type GenResult = Result<(), GenError>;

    pub use std::io::{self, BufRead, Read};
    pub use std::iter;
    pub use std::str::FromStr;

    pub use itertools::Itertools;
    use itertools::{process_results, ProcessResults};

    use std::error::Error;
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use std::path::PathBuf;

    pub fn load_input(day: u8) -> io::Result<BufReader<File>> {
        let mut path = PathBuf::new();
        path.push("inputs");
        path.push(format!("input{:02}.txt", day));
        println!("Reading from {}", path.display());
        File::open(&path).map(BufReader::new)
    }

    pub fn load_input_string(day: u8) -> io::Result<String> {
        let mut input = String::new();
        load_input(day)?.read_to_string(&mut input)?;
        Ok(input)
    }

    pub fn process_inputs<F, R>(day: u8, f: F) -> io::Result<R>
    where
        F: FnOnce(ProcessResults<Lines<BufReader<File>>, io::Error>) -> R,
    {
        process_results(load_input(day)?.lines(), f)
    }
}
