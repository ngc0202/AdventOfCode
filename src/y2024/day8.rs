use super::Solution;
use crate::utils::sgrid::GridParseErr;

day!(run 8);

struct Day8 {
    antennae: Vec<Antenna>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
struct Antenna {
    freq: u8,
    y: usize,
    x: usize,
}

impl<'i> Solution<'i> for Day8 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, GridParseErr> {
        let mut antennae = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut skip = false;

        for b in input.drain(..) {
            match b {
                b'\n' | b'\r' => {
                    if !skip {
                        y += 1;
                        if width == 0 {
                            width = x;
                        } else if width != x {
                            return Err(GridParseErr::JaggedEdge);
                        }
                        x = 0;
                        skip = true;
                    }
                }
                b'.' => {
                    x += 1;
                    skip = false;
                }
                b if b.is_ascii_alphanumeric() => {
                    antennae.push(Antenna { freq: b, x, y });
                    x += 1;
                    skip = false;
                }
                _ => return Err(GridParseErr::BadByte { b }),
            }
        }

        if antennae.is_empty() {
            return Err(GridParseErr::Empty);
        }

        antennae.sort_unstable_by_key(|a| a.freq);

        let height = if skip { y } else { y + 1 };

        Ok(Self {
            antennae,
            width,
            height,
        })
    }

    fn part1(&mut self) -> u64 {
        let mut cache = vec![false; self.width * self.height];

        self.fold_pairs(0, |mut cnt, a, b| {
            let nodes = [next_node(a, b), next_node(b, a)];
            for [x, y] in nodes.into_iter().flatten() {
                if self.set_cache(&mut cache, x, y) {
                    cnt += 1;
                }
            }
            cnt
        })
    }

    fn part2(&mut self) -> u64 {
        let mut cache = vec![false; self.width * self.height];

        let mut each_node = |cnt: &mut u64, mut a: Antenna, mut b: Antenna| {
            while a.x < self.width && a.y < self.height {
                // Update cache and count
                if self.set_cache(&mut cache, a.x, a.y) {
                    *cnt += 1;
                }

                // Get next node
                let Some([x, y]) = next_node(&a, &b) else {
                    break;
                };

                // Cycle nodes
                b = a;
                a.x = x;
                a.y = y;
            }
        };

        self.fold_pairs(0, |mut cnt, &a, &b| {
            each_node(&mut cnt, a, b);
            each_node(&mut cnt, b, a);
            cnt
        })
    }
}

impl Day8 {
    /// Sets the cache at (x,y) to true, returning whether this index is valid and unique
    pub fn set_cache(&self, cache: &mut [bool], x: usize, y: usize) -> bool {
        x < self.width
            && y < self.height
            && !std::mem::replace(&mut cache[y * self.width + x], true)
    }

    pub fn fold_pairs<B, F>(&self, mut acc: B, mut f: F) -> B
    where
        F: FnMut(B, &Antenna, &Antenna) -> B,
    {
        for group in self.antennae.chunk_by(|a, b| a.freq == b.freq) {
            let mut it = group.iter();
            while let Some(a) = it.next() {
                for b in it.as_slice() {
                    acc = f(acc, a, b);
                }
            }
        }
        acc
    }
}

/// Checked calculation of: 2a - b
fn dim(a: usize, b: usize) -> Option<usize> {
    a.checked_add(a).and_then(|n| n.checked_sub(b))
}

/// Find the next inline node
fn next_node(a: &Antenna, b: &Antenna) -> Option<[usize; 2]> {
    dim(a.x, b.x).and_then(|x| dim(a.y, b.y).map(|y| [x, y]))
}
