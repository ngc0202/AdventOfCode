
use crate::GenResult;

use std::io::BufRead;
use std::num::ParseIntError;

use bitvec::prelude::*;
use bitvec::ptr::Mut as BVMut;
use bitvec::access::BitSafeU32;
use itertools::Itertools;


const DAY: u8 = 4;
type Ubingo = u8;

// 5x5 bingo board
#[derive(Clone, Debug, PartialEq, Eq)]
struct BingoBoard {
    numbers: [Ubingo; 25],
    marks: BitArr!(for 25, in u32)
}

impl BingoBoard {

    pub fn from_text<S: AsRef<str>, I: IntoIterator<Item=S>>(text_iter: I) -> Result<BingoBoard, ParseIntError> {
        text_iter.into_iter()
                 .flat_map(|l|
                     l.as_ref()
                     .split_whitespace()
                     .map(|t| t.parse::<Ubingo>())
                     .collect::<Vec<_>>()) // :(
                 .collect()
    }

    // Returns whether any were marked
    pub fn mark(&mut self, num: Ubingo) -> bool {
        let mut found = false;
        for (i, &n) in self.numbers.iter().enumerate() {
            if num == n {
                self.marks.set(i, true);
                found = true;
            }
        }
        found
    }

    pub fn iter(&'_ self) -> impl Iterator<Item=(Ubingo, bool)> + '_ {
        self.numbers.iter().copied().zip(self.marks.iter().by_val())
    }

    pub fn iter_mut(&'_ mut self) -> impl Iterator<Item=(&'_ mut Ubingo, BitRef<BVMut, LocalBits, BitSafeU32>)> {
        self.numbers.iter_mut().zip(self.marks.iter_mut())
    }

    pub fn has_won(&self) -> bool {
        let bits = self.marks.as_bitslice();

        // Check rows
        if bits.chunks(5).any(BitSlice::all) {
            return true;
        }

        // Check columns
        for i in 0..5 {
            if (i..25).step_by(5).all(|j| *bits.get(j).unwrap()) {
                return true;
            }
        }

        // Nope
        false
    }

    // Sum of all non-marked squares
    pub fn score(&self) -> u64 {
        self.iter().filter_map(|(v, m)| (!m).then(|| u64::from(v))).sum()
    }

    pub fn reset(&mut self) {
        self.marks.set_all(false);
    }
}

impl FromIterator<Ubingo> for BingoBoard {
    fn from_iter<I: IntoIterator<Item=Ubingo>>(numbers: I) -> Self {
        let mut numarray = [0; 25];
        let mut counter = 0;

        for (n, x) in numbers.into_iter().zip(numarray.iter_mut()) {
            *x = n;
            counter += 1;
        }

        assert!(counter >= 25, "Only loaded {}/25 values for bingo card.", counter);

        BingoBoard {
            numbers: numarray,
            marks: BitArray::zeroed()
        }
    }
}

pub fn run() -> GenResult {
    let mut input = crate::load_input(DAY)?.lines();

    let nums = input.next()
                    .unwrap()?
                    .split(',')
                    .map(|n| n.parse())
                    .collect::<Result<Vec<u8>, ParseIntError>>()?;

    let mut boards = input.chunks(6)
                      .into_iter()
                      .map(|chk|
                          BingoBoard::from_text(chk.map(Result::unwrap)))
                      .collect::<Result<Vec<_>, ParseIntError>>()?;

    // Part 1 - First Winner
    'out: for &num in nums.iter() {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.has_won() {
                let score = board.score();
                println!("Part 1: {}", u64::from(num) * score);
                break 'out;
            }
        }
    }

    // Reset
    boards.iter_mut().for_each(BingoBoard::reset);

    // Part 2 - Last Winner
    'out2: for &num in nums.iter() {
        let mut idx = 0;
        while idx < boards.len() {
            let board = boards.get_mut(idx).unwrap();
            board.mark(num);
            if board.has_won() {
                if boards.len() == 1 {
                    let last_board = boards.get(0).unwrap();
                    println!("Part 2: {}", (num as u64) * last_board.score());
                    break 'out2;
                } else {
                    boards.swap_remove(idx);
                }
            } else {
                idx += 1;
            }
        }
    }

    Ok(())
}