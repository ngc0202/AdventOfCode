
use std::collections::HashMap;

use crate::prelude::*;

day!(3);

// Anything not a digit or period.
fn is_symbol(b: u8) -> bool {
    b != b'.' && !b.is_ascii_digit()
}

fn process(lines: &[&[u8]]) -> (u64, u64) {
    let mut sum = 0u64;
    let mut gears = HashMap::<(usize, usize), (u64, u64)>::new();

    for (line_num, &line) in lines.iter().enumerate() {
        let mut cidx = 0;
        while cidx < line.len() {
            if line[cidx].is_ascii_digit() {
                let num_start = cidx;
                let num_len = line[cidx..]
                    .iter()
                    .take_while(|b| b.is_ascii_digit()).count();

                cidx += num_len;

                // Parse the num
                let num = line[num_start..][..num_len]
                    .iter()
                    .fold(0u64, |acc, b| acc * 10 + u64::from(b - b'0'));

                let left_check = num_start.saturating_sub(1);
                let right_check = (line.len()-1).min(num_start+num_len);

                // Get adjacent lines
                let line_above = (line_num > 0).then(|| lines[line_num-1]).unwrap_or(&[]);
                let line_below = lines.get(line_num+1).copied().unwrap_or(&[]);

                // Iterate adjacencies
                let mut has_symbol = false;
                for (clnum, cline) in [line_above, line, line_below].into_iter().enumerate() {
                    if let Some(cline) = cline.get(left_check..=right_check) {
                        for (csnum, &cbyte) in cline.iter().enumerate() {
                            // Check for symbol (part 1)
                            if !has_symbol && is_symbol(cbyte) {
                                has_symbol = true;
                            }

                            // Check for gear (part 2)
                            if cbyte == b'*' {
                                let gline = clnum + line_num - 1;
                                let gidx = left_check + csnum;
                                let (cnt, ratio) = gears.entry((gline, gidx)).or_default();
                                *cnt = cnt.saturating_add(1);
                                if *cnt == 1 {
                                    *ratio = num;
                                } else if *cnt == 2 {
                                    *ratio = ratio.saturating_mul(num);
                                }
                            }
                        } 
                    }
                }

                // If has symbol, add it
                if has_symbol {
                    sum += num;
                }
            } else {
                cidx += 1;
            }
        }
    }

    let ratio: u64 = gears.into_iter()
        .filter_map(|(_, (c, r))| (c == 2).then_some(r))
        .sum();

    (sum, ratio)
}

// 467835 - too low
pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");
    let lines: Vec<_> = input.split(|&b| b == b'\n').filter(|l| !l.is_empty()).collect();

    let (part1, part2) = process(&lines);
    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}
