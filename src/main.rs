#[macro_use]
mod macros;

use crate::prelude::*;
use paste::paste;

// Main Method
days!(4, 5*, 6, 7);

mod prelude {

    pub type GenError = Box<dyn Error>;
    pub type GenResult = Result<(), GenError>;

    pub use std::io::{self, BufRead};
    pub use std::iter;
    pub use std::str::FromStr;

    pub use itertools::Itertools;

    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    pub fn load_input(day: u8) -> io::Result<BufReader<File>> {
        let mut path = PathBuf::new();
        path.push("inputs");
        path.push(format!("input{:02}.txt", day));
        println!("Reading from {}", path.display());
        File::open(&path).map(BufReader::new)
    }
}
