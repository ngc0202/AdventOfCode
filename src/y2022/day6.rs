
use crate::prelude::*;
use bitvec::prelude::*;
use nom::Offset;

day!(6);

type CharMap = BitArr!(for 26, in u32);

fn find_marker(input: &[u8], len: usize) -> Result<usize, Whatever> {
    'win: for window in input.windows(len) {
        let mut map = CharMap::ZERO;
        for c in window {
            ensure_whatever!((b'a'..=b'z').contains(c), "Invalid letter ({c})");
            let i = usize::from(c - b'a');
            let mut x = map.get_mut(i).unwrap();
            if *x {
                continue 'win;
            } else {
                x.set(true);
            }
        }

        let wl = window.len();
        return Ok(input.offset(&window[wl..wl]));
    }
    
    whatever!("Failed to find market of length {len}");
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed loading input");

    let part1 = whatever!(find_marker(&input, 4), "Part 1 failed");
    println!("Part 1: {part1}");

    let part2 = whatever!(find_marker(&input, 14), "Part 2 failed");
    println!("Part 2: {part2}");

    Ok(())
}