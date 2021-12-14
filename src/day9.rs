use crate::prelude::*;
use crate::utils::Grid;

const DAY: u8 = 9;

const DIM_X: usize = 100;
const DIM_Y: usize = 100;

#[derive(Debug, Clone, Default)]
struct Cave {
    grid: Grid<u8, DIM_X, DIM_Y>,
}

impl Cave {
    pub fn from_input<B: BufRead>(rdr: &mut B) -> io::Result<Self> {
        let mut cave = Cave::default();
        let mut idx = 0;
        for line in rdr.lines() {
            for c in line?.chars() {
                *cave.grid.get_flat_mut(idx).unwrap() = c.to_digit(10).unwrap() as u8;
                idx += 1;
            }
        }
        Ok(cave)
    }

    pub fn get_danger(&self, xidx: usize, yidx: usize) -> u8 {
        let &cur = self.grid.get(xidx, yidx).unwrap();
        if self
            .grid
            .iter_neighbors(xidx, yidx, false)
            .all(|(_, _, &x)| cur < x)
        {
            1 + cur
        } else {
            0
        }
    }

    pub fn find_basins(&self) -> u64 {
        let mut basins: Grid<u16, DIM_X, DIM_Y> = Grid::default();
        let mut c = 0;

        for (x, y, &v) in self.grid.iter_coords() {
            if v < 9 && *basins.get(x, y).unwrap() == 0 {
                c += 1;
                self.flow_basin(x, y, c, &mut basins);
            }
        }

        basins
            .iter()
            .copied()
            .filter(|&v| v > 0)
            .counts()
            .into_values()
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(3)
            .map(|v| u64::try_from(v).unwrap())
            .product()
    }

    fn flow_basin(
        &self,
        xidx: usize,
        yidx: usize,
        code: u16,
        basins: &mut Grid<u16, DIM_X, DIM_Y>,
    ) {
        for (nbr_x, nbr_y, &nbr_val) in self.grid.iter_neighbors(xidx, yidx, false) {
            let basin_nbr = basins.get_mut(nbr_x, nbr_y).unwrap();
            if nbr_val < 9 && *basin_nbr == 0 {
                *basin_nbr = code;
                self.flow_basin(nbr_x, nbr_y, code, basins);
            }
        }
    }
}

pub fn run() -> GenResult {
    let cave = Cave::from_input(&mut load_input(DAY)?)?;

    // Part 1
    let part1: u64 = cave
        .grid
        .iter_coords()
        .map(|(x, y, _)| cave.get_danger(x, y) as u64)
        .sum();
    println!("Part 1: {}", part1);

    let part2 = cave.find_basins();
    println!("Part 2: {}", part2);

    Ok(())
}
