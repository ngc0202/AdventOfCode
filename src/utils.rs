use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T, const W: usize, const H: usize>([[T; W]; H]);

impl<T: Default + Copy, const W: usize, const H: usize> Default for Grid<T, W, H> {
    fn default() -> Grid<T, W, H> {
        Grid([[T::default(); W]; H])
    }
}

impl<T: fmt::Display, const W: usize, const H: usize> fmt::Display for Grid<T, W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format = self
            .0
            .iter()
            .format_with("\n", |row, f| f(&row.iter().format_with(" ", |v, g| g(v))));

        write!(f, "{}", format)
    }
}

impl<T, const W: usize, const H: usize> Grid<T, W, H> {
    pub const fn size(&self) -> usize {
        H * W
    }

    pub const fn width(&self) -> usize {
        W
    }

    pub const fn height(&self) -> usize {
        H
    }

    pub fn get(&self, xidx: usize, yidx: usize) -> Option<&T> {
        if xidx < W && yidx < H {
            Some(&self.0[yidx][xidx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, xidx: usize, yidx: usize) -> Option<&mut T> {
        if xidx < W && yidx < H {
            Some(&mut self.0[yidx][xidx])
        } else {
            None
        }
    }

    pub fn get_flat(&self, idx: usize) -> Option<&T> {
        if idx < self.size() {
            let (h, w) = ((idx / W), (idx % W));
            Some(&self.0[h][w])
        } else {
            None
        }
    }

    pub fn get_flat_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.size() {
            let (h, w) = ((idx / W), (idx % W));
            Some(&mut self.0[h][w])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flat_map(|row| row.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..H)
            .cartesian_product(0..W)
            .zip(self.iter())
            .map(|((x, y), v)| (x, y, v))
    }

    pub fn iter_neighbors(
        &self,
        xidx: usize,
        yidx: usize,
        diagonals: bool,
    ) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..3)
            .cartesian_product(0..3)
            .filter(move |&(x, y)| ((x != 1) || (y != 1)) && (diagonals || ((x == 1) || (y == 1))))
            .filter_map(move |(xadj, yadj)| {
                (xidx + xadj).checked_sub(1).and_then(|xnew| {
                    (yidx + yadj)
                        .checked_sub(1)
                        .and_then(|ynew| self.get(xnew, ynew).map(|val| (xnew, ynew, val)))
                })
            })
    }
}
