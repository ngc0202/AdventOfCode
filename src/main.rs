
use std::error::Error;
use std::io::{self, BufReader};
use std::fs::File;
use std::path::PathBuf;

mod day4;

pub type GenResult = Result<(), Box<dyn Error>>;


fn main() -> GenResult {
    day4::run()?;

    Ok(())
}

pub fn load_input(day: u8) -> io::Result<BufReader<File>> {
    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(format!("input{:02}.txt", day));
    println!("Reading from {}", path.display());
    File::open(&path).map(BufReader::new)
}