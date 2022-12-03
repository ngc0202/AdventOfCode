
use crate::prelude::*;
use bitvec::prelude::*;
use std::ops::{Deref, DerefMut};
use std::fmt;

day!(13);


#[derive(Clone, Debug)]
struct Paper {
    vec: BitVec<usize, Lsb0>,
    width: usize
}

impl Deref for Paper {
    type Target = BitSlice<usize, Lsb0>;

    fn deref(&self) -> &Self::Target {
        self.vec.as_bitslice()
    }
}

impl DerefMut for Paper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.vec.as_mut_bitslice()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            self.vec
                .iter()
                .map(|b| if *b { "#" } else { "." })
                .chunks(self.width)
                .into_iter()
                .format_with("\n", |row, f| f(&row.format(""))))
    }
}

impl Paper {
    pub fn new(width: usize) -> Self {
        let mut vec = BitVec::new();
        vec.resize(width * width, false);
        Paper { vec, width }
    }

    pub fn set_coord(&mut self, x: usize, y: usize, val: bool) {
        let idx = y * self.width + x;
        self.set(idx, val);
    }

    pub fn count_dots(&self) -> usize {
        self.vec.count_ones()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item=[usize; 2]> + '_ {
        self.vec.iter_ones().map(|i| [i % self.width, i / self.width])
    }

    // false='x', true='y'
    pub fn fold(&mut self, ax: bool, val: usize) {
        let ones = self.vec.iter_ones().collect_vec();
        for idx in ones {
            let (x, y) = (idx % self.width, idx / self.width);
            if ax {
                if let Some(v) = y.checked_sub(val) {
                    self.set(idx, false);
                    if let Some(y2) = val.checked_sub(v) {
                        self.set_coord(x, y2, true);
                    }
                }
            } else if let Some(v) = x.checked_sub(val) {
                self.set(idx, false);
                if let Some(x2) = val.checked_sub(v) {
                    self.set_coord(x2, y, true);
                }
            }
        }
    }

    pub fn shrunk(&self) -> Paper {
        let max_dim = self.iter_coords().flatten().max().unwrap_or(0);
        let mut paper = Paper::new(max_dim+1);
        for [x, y] in self.iter_coords() {
            paper.set_coord(x, y, true);
        }

        if let Some(last) = paper.vec.last_one() {
            let nmul = last + (paper.width - last % paper.width);
            paper.vec.truncate(nmul);
        }

        paper
    }
}

pub fn run() -> GenResult {
    let input = load_input_string(DAY)?;
    let (coords, folds) = input.split_once("\n\n").ok_or("no sep")?;
    let coords: Vec<[usize; 2]> =
        coords
            .split('\n')
            .map(|line|
                line.split_once(',')
                    .ok_or_else(|| GenError::from("no comma"))
                    .and_then(|(x, y)| x.parse().and_then(|x| y.parse().map(|y| [x, y])).map_err(GenError::from)))
            .try_collect()?;

    let max_coord = *coords.iter().flatten().max().ok_or("no coords")?;
    let mut paper = Paper::new(max_coord+1);
    for &[x, y] in &coords {
        paper.set_coord(x, y, true);
    }

    let mut one = true;
    for fold in folds.trim().split('\n') {
        let (ax, v) = fold[11..].split_once('=').ok_or("no equals")?;
        let ax = match ax {
            "x" => false,
            "y" => true,
            _ => return Err("invalid axis".into())
        };
        paper.fold(ax, v.parse()?);

        if one {
            println!("Part 1: {}", paper.count_dots());
            one = false;
        }
    }

    println!("Part 2:\n{}", &paper.shrunk());

    Ok(())
}
