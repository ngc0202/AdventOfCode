use crate::prelude::*;

use super::Solution;

day!(run 9);

struct Day9 {
    vec: Vec<Block>,
}

#[derive(Copy, Clone)]
struct Block {
    tag: Option<usize>,
    len: u8,
    idx: usize,
}

impl Solution for Day9 {
    const DAY: Day = DAY;

    fn parse(input: Vec<u8>) -> Result<Self, BadByte> {
        input
            .into_iter()
            .enumerate()
            .filter(|(_, len)| *len != b'0')
            .scan(0, |n, (i, len)| {
                if !len.is_ascii_digit() {
                    return Some(BadByteSnafu { b: len }.fail());
                }
                let tag = (i % 2 == 0).then_some(i / 2);
                let idx = *n;
                let len = len - b'0';
                *n += usize::from(len);
                Some(Ok(Block { tag, len, idx }))
            })
            .try_collect()
            .map(|vec| Self { vec })
    }

    fn part1(&mut self) -> usize {
        let mut blocks = self
            .vec
            .iter()
            .flat_map(|b| iter::repeat_n(b.tag, b.len.into()));

        let mut i = 0_usize;
        let mut checksum = 0_usize;
        loop {
            // Find first free space
            loop {
                match blocks.next() {
                    None => return checksum,
                    Some(None) => break,
                    Some(Some(id)) => {
                        checksum += i * id;
                        i += 1;
                    }
                }
            }

            // Find last taken space
            let Some(id) = blocks.by_ref().flatten().next_back() else {
                return checksum;
            };

            checksum += i * id;
            i += 1;
        }
    }

    fn part2(&mut self) -> usize {
        let blocks = self.vec.as_mut_slice();
        let csum = |idx, len, tag| (idx..idx + len).map(|j| j * tag).sum::<usize>();

        // Place each from back
        let checksum = (0..blocks.len())
            .rev()
            .map(|bi| {
                let block = blocks[bi];
                let Some(tag) = block.tag else { return 0 };
                let gap = blocks[..bi]
                    .iter_mut()
                    .find(|b| b.tag.is_none() && b.len >= block.len);
                if let Some(gap) = gap {
                    let blen = usize::from(block.len);
                    let chk = csum(gap.idx, blen, tag);
                    let rem = gap.len - block.len;
                    gap.len = rem;
                    gap.idx += blen;
                    chk
                } else {
                    csum(block.idx, usize::from(block.len), tag)
                }
            })
            .sum();

        checksum
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Got invalid byte `{b:?}`"))]
struct BadByte {
    b: char,
}
