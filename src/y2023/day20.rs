use crate::prelude::*;

day!(20);


type Signal = bool;


pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");



    Ok(())
}


trait Component {
    // fn pulse(&mut self, inputs: &[], outputs: &mut []);
}

struct FlipFlop {
    state: bool,
}

struct Conjunction {
    state: bool,
}
