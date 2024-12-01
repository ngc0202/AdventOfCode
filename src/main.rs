#![deny(rust_2018_idioms)]

use crate::prelude::*;

#[macro_use]
mod macros;

#[allow(unused)]
mod utils;

#[allow(warnings)]
mod y2021;
#[allow(warnings)]
mod y2022;
#[allow(warnings)]
mod y2023;
mod y2024;

use std::cell::Cell;
thread_local!(
    pub static SMALL: Cell<Option<bool>> = Cell::new(None);
);

#[snafu::report]
fn main() -> Result<(), Whatever> {
    y2024::main()
}

pub fn get_small() -> bool {
    SMALL.with(|small| {
        if let Some(small) = small.get() {
            return small;
        }

        let val = match std::env::args().nth(1) {
            Some(arg) => arg.eq_ignore_ascii_case("small"),
            None => false,
        };

        small.set(Some(val));
        val
    })
}

mod prelude {
    pub type GenError = Box<dyn std::error::Error>;
    pub type GenResult<T = ()> = Result<T, GenError>;

    pub use std::io::{self, BufRead, Read};
    pub use std::iter;
    pub use std::str::FromStr;

    pub use itertools::Itertools;

    pub use crate::utils::{
        load_input, load_input_bytes, load_input_string, process_inputs, Day, NoInputError,
    };

    pub use snafu::{prelude::*, Whatever};

    pub use nom::Finish;
}
