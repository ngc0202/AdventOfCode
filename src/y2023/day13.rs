
use crate::{prelude::*, utils::sgrid::{GridParseErr, Grid}};

day!(13);

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let mut part1 = 0;
    let mut part2 = 0;
    let res = each_grid(&input, |g| {
        part1 += solve(&g, 0);
        part2 += solve(&g, 1);
    });

    whatever!(res, "Failed to parse input");
    println!("Part 1: {part1}\nPart 2: {part2}");

    Ok(())
}

fn solve(grid: &Grid<bool>, smudges: usize) -> usize {
    let width = grid.width();
    if grid.is_empty() || width == 0 {
        return 0;
    }

    // Check for vertical line of reflection
    'y: for y in 1..width {
        let mut errata = 0;
        for line in grid.rows() {
            let (left, right) = line.split_at(y);
            for (l, r) in left.iter().rev().zip(right) {
                if l != r {
                    errata += 1;
                    if errata > smudges {
                        continue 'y;
                    }
                }
            }
        }

        if errata == smudges {
            return y;
        }
    }

    // Check for horizontal line of reflection
    let height = grid.height();
    'x: for x in 1..height {
        let mut errata = 0;
        let rows = x.min(height - x);
        for y in 0..rows {
            let top = grid.row(x - y - 1);
            let bot = grid.row(x + y);
            for (l, r) in top.iter().zip(bot) {
                if l != r {
                    errata += 1;
                    if errata > smudges {
                        continue 'x;
                    }
                }
            }
        }

        if errata == smudges {
            return 100 * x;
        }
    }

    0
}

fn each_grid<F: FnMut(Grid<bool>)>(mut input: &[u8], mut f: F) -> Result<(), GridParseErr> {
    let line_ending = nom::character::complete::line_ending::<&[u8], ()>;

    let parse_elem = |b| match b {
        b'#' => Some(true),
        b'.' => Some(false),
        _ => None,
    };

    while !input.is_empty() {
        if let Ok((inp, _)) = line_ending(input) {
            input = inp;
        } 

        let grid;
        (input, grid) = Grid::parse(input, parse_elem)?;
        f(grid);
    }

    Ok(())
}
