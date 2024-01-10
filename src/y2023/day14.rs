use std::{num::Wrapping, fmt::{Display, Write}};

use indexmap::IndexSet;

use crate::prelude::*;

day!(14);

type Grid = crate::utils::sgrid::Grid<Tile>;

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let parse_elem = |b| match b {
        b'O' => Some(Tile::Round),
        b'#' => Some(Tile::Cube),
        b'.' => Some(Tile::Empty),
        _ => None,
    };

    let (_, grid) = whatever!(Grid::parse(&input, parse_elem), "Failed to parse input");

    // println!("{grid}\n\n{}", grid.rotate());

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

fn part1(grid: &Grid) -> usize {
    let width = grid.width();
    let height = grid.height();

    let mut part1 = 0;
    for xval in 0..width {
        let mut rocks = 0_usize;
        for (yval, &tile) in grid.column(xval).rev().enumerate() {
            if tile == Tile::Round {
                rocks += 1;
            } else if tile == Tile::Cube && rocks > 0 {
                part1 += score(rocks, yval);
                rocks = 0;
            }
        }

        if rocks > 0 {
            part1 += score(rocks, height);
        }
    }

    part1
}

fn part2(grid: &Grid) -> usize {
    const GOAL: usize = 4_000_000_000;

    let mut cache = IndexSet::<Box<[Tile]>>::new();
    cache.insert(grid.clone().into_inner().into_boxed_slice());

    let mut cur = grid.rotate_left();

    for cur_idx in 1..=GOAL {
        shift_left(&mut cur);
        if cur_idx == GOAL { break }

        let next = cur.rotate_right();
        let (match_idx, inserted) = cache.insert_full(cur.into_inner().into_boxed_slice());

        if !inserted {
            let cycle_len = cur_idx - match_idx;
            println!("Cycle detected from {match_idx} to {cur_idx}  (length {cycle_len})");
            let final_idx = (GOAL - match_idx) % cycle_len + match_idx;
            let final_elems = cache.swap_remove_index(final_idx).expect("Missing final index");
            let final_grid = Grid::new(final_elems.into_vec(), grid.height()).rotate_right();
            cur = final_grid;
            break;
        }

        cur = next;
    }

    score_grid(&cur)
}

fn score_grid(grid: &Grid) -> usize {
    grid.iter()
        .zip((1..=grid.width()).rev().cycle())
        .filter(|(&t, _)| t == Tile::Round)
        .map(|(_, i)| i)
        .sum()
}

fn shift_left(grid: &mut Grid) {
    fn set_rocks(tiles: &mut [Tile], rocks: usize) {
        tiles[..rocks].fill(Tile::Round);
    }

    for row in grid.rows_mut() {
        let mut rocks = 0_usize;
        for idx in (1..=row.len()).rev() {
            let ([.., tile], post) = row.split_at_mut(idx) else {
                unreachable!()
            };

            if *tile == Tile::Round {
                rocks += 1;
                *tile = Tile::Empty;
            } else if *tile == Tile::Cube && rocks > 0 {
                set_rocks(post, rocks);
                rocks = 0;
            }
        }

        set_rocks(row, rocks);
    }
}

#[must_use]
fn score(rocks: usize, row: usize) -> usize {
    let rocks = Wrapping(rocks);
    let row = Wrapping(row);
    (rocks * (Wrapping(1) + row + row - rocks)).0 / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Tile::Round => 'O',
            Tile::Cube => '#',
            Tile::Empty => '.',
        };

        f.write_char(c)
    }
}