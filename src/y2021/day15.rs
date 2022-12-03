
use crate::prelude::*;
use crate::utils::Grid;
use bitvec::prelude::*;

day!(15);

#[inline]
fn set_if_lower<T: PartialOrd>(dest: &mut T, val: T) {
    if val < *dest {
        *dest = val;
    }
}

#[inline]
fn wrap_sum(a: u8, b: u8) -> u8 {
    match a + b {
        n if n <= 9 => n,
        n => n - 9
    }
}

// Dijkstra's algorithm
fn lowest_path<const W: usize, const H: usize>(grid: &Grid<u8, W, H>) -> u32 {
    // let visited = bits![const u32, Lsb0; 0; W*H];
    let mut visited = BitVec::<usize, Lsb0>::new();
    visited.resize(W*H, false);

    // let mut distances: Grid<u32, W, H> = Grid::of(u32::MAX);
    let mut distances = vec![u32::MAX; W*H];
    distances[0] = 0;
    let mut cur = [0, 0];

    while !visited[(W*H)-1] {
        let cur_idx = cur[0] + W * cur[1];
        let cur_dist = distances[cur_idx];

        for (nbr_x, nbr_y, &nbr_w) in grid.iter_neighbors(cur[0], cur[1], false) {
            let nbr_idx = nbr_x + W * nbr_y;
            if !visited[nbr_idx] {
                set_if_lower(&mut distances[nbr_idx], cur_dist + u32::from(nbr_w));
            }
        }

        visited.set(cur_idx, true);

        let min_idx = visited.iter_zeros()
                             .min_by_key(|&idx| distances[idx]);

        if let Some(idx) = min_idx {
            cur = [idx % W, idx / W];
        } else {
            break;
        }
    }

    distances[(W*H)-1]
}

fn quintuple_grid(old_grid: &Grid<u8, 100, 100>) -> Grid<u8, 500, 500> {
    let mut grid: Grid<u8, 500, 500> = Grid::default();
    for block_x in 0..5 {
        for block_y in 0..5 {
            let inc = block_x + block_y;
            let (corner_x, corner_y) = ((block_x as usize) * 100, (block_y as usize) * 100);
            for (old_x, old_y, &old_val) in old_grid.iter_coords() {
                let idx = [
                    old_x + (corner_x as usize),
                    old_y + (corner_y as usize)
                ];

                grid[idx] = wrap_sum(old_val, inc);
            }
        }
    }

    grid
}

pub fn run() -> GenResult {
    let input = load_input_string(DAY)?;
    let grid: Grid<u8, 100, 100> = {
        let mut g = Grid::default();
        let x = g.iter_mut().set_from(input.chars().filter_map(|c| c.to_digit(10).and_then(|d| d.try_into().ok())));
        if x < 100*100 {
            return Err("insufficient input for grid".into());
        }
        g
    };

    let part1 = lowest_path(&grid);
    println!("Part 1: {}", part1);

    let grid2 = quintuple_grid(&grid);
    let part2 = lowest_path(&grid2);
    println!("Part 2: {}", part2);

    Ok(())
}
