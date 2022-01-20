
use crate::prelude::*;
use bitvec::prelude::*;
use std::fmt;
use itertools::izip;

const DAY: u8 = 8;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Seg {
    A, B, C, D, E, F, G
}

impl TryFrom<char> for Seg {
    type Error = ();

    fn try_from(c: char) -> Result<Seg, ()> {
        match c {
            'a'|'A' => Ok(Seg::A),
            'b'|'B' => Ok(Seg::B),
            'c'|'C' => Ok(Seg::C),
            'd'|'D' => Ok(Seg::D),
            'e'|'E' => Ok(Seg::E),
            'f'|'F' => Ok(Seg::F),
            'g'|'G' => Ok(Seg::G),
            _   => Err(())
        }
    }
}

impl fmt::Display for Seg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Seg::A => "A",
            Seg::B => "B",
            Seg::C => "C",
            Seg::D => "D",
            Seg::E => "E",
            Seg::F => "F",
            Seg::G => "G"
        })
    }
}

macro_rules! segs {
    ($($id:ident),+) => ([$(Seg::$id),+]);
    ($([$($id:ident),*]),+) => ([$(&[$(Seg::$id),*]),+])
}

macro_rules! segbits {
    ($([$($bit:expr),*]),+) => ([$(segbits![$($bit),*]),+]);
    ($($bit:expr),*) => (SegBits(bitarr![const u8, LocalBits; $($bit),*]));
}

type SegBitArray = BitArr!(for 7, in u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct SegBits(SegBitArray);

impl FromStr for SegBits {
    type Err = ();

    fn from_str(s: &str) -> Result<SegBits, ()> {
        s.chars().map(Seg::try_from).collect()
    }
}

impl FromIterator<Seg> for SegBits {
    fn from_iter<I: IntoIterator<Item=Seg>>(iter: I) -> Self {
        iter.into_iter().fold(SegBits::ZERO, |mut sb, sg | {
            sb.set_seg(sg, true);
            sb
        })
    }
}

impl fmt::Display for SegBits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.iter_segs().format(""))
    }
}

impl SegBits {
    pub const ZERO: SegBits = SegBits(SegBitArray::ZERO);
    pub const ONES: SegBits = SegBits::new(0b11111111);

    pub const fn new(num: u8) -> Self {
        SegBits(SegBitArray{data: [num], ..SegBitArray::ZERO})
    }

    pub fn get_seg(&self, seg: Seg) -> bool {
        *self.0.get(seg as usize).unwrap()
    }

    pub fn set_seg(&mut self, seg: Seg, val: bool) {
        self.0.set(seg as usize, val)
    }

    pub fn iter_segs(&self) -> impl Iterator<Item=Seg> + '_ {
        self.0
            .iter()
            .zip(ALL_SEGS)
            .filter_map(|(b, s)| b.then(|| s))
    }
}

const ALL_SEGS: [Seg; 7] = segs![A,B,C,D,E,F,G];

const DIGITS: [SegBits; 10] = segbits![
    [1, 1, 1, 0, 1, 1, 1], // 0
    [0, 0, 1, 0, 0, 1, 0], // 1
    [1, 0, 1, 1, 1, 0, 1], // 2
    [1, 0, 1, 1, 0, 1, 1], // 3
    [0, 1, 1, 1, 0, 1, 0], // 4
    [1, 1, 0, 1, 0, 1, 1], // 5
    [1, 1, 0, 1, 1, 1, 1], // 6
    [1, 0, 1, 0, 0, 1, 0], // 7
    [1, 1, 1, 1, 1, 1, 1], // 8
    [1, 1, 1, 1, 0, 1, 1]  // 9
];

const POS_MAP: [SegBits; 5] = segbits![
    [0, 0, 1, 0, 0, 1, 0], // 2
    [1, 0, 1, 0, 0, 1, 0], // 3
    [0, 1, 1, 1, 0, 1, 0], // 4
    [1, 1, 1, 1, 1, 1, 1], // 5
    [1, 1, 1, 1, 1, 1, 1]  // 6
];

const NEG_MAP: [SegBits; 5] = segbits![
    [1, 1, 0, 1, 1, 0, 1], // 2
    [0, 1, 0, 1, 1, 0, 1], // 3
    [1, 0, 0, 0, 1, 0, 1], // 4
    [0, 1, 1, 0, 1, 1, 0], // 5
    [0, 0, 1, 1, 1, 0, 0]  // 6
];

fn determine_digits(s: &str) -> u32 {
    let (seq, output) = s.rsplit_once(" | ").expect("no pipe");
    let mut fake2real = [(); 7].map(|_| SegBits::ONES);
    let mut mapping: [Option<Seg>; 7] = [None; 7];

    // narrow down
    for fs_str in seq.split_whitespace() {
        let fake_segs: SegBits = fs_str.parse().expect("parse fail");
        let fslen = fake_segs.0.count_ones();

        assert!((2..=7).contains(&fslen), "Input must be in [2, 7] segments.");
        if fslen == 7 { continue }

        let new_pos = POS_MAP[fslen-2];
        let new_neg = NEG_MAP[fslen-2];

        for (seg, f2r, map) in izip![ALL_SEGS,  &mut fake2real, &mut mapping] {
            if map.is_none() {
                f2r.0 &= if fake_segs.get_seg(seg) { new_pos.0 } else { new_neg.0 };
                if let Ok(v) = f2r.iter_segs().exactly_one() {
                    map.replace(v);
                }
            }
        }
    }

    mapping
        .into_iter()
        .flatten()
        .for_each(|sg|
            fake2real
                .iter_mut()
                .for_each(|f2r| f2r.set_seg(sg, false)));

    mapping
        .iter_mut()
        .zip(&fake2real)
        .filter(|(v,_)| v.is_none())
        .for_each(|(v, w)| {
            v.replace(w.iter_segs().exactly_one().or(Err("not exactly one")).unwrap());
        });

    assert!(mapping.iter().all(Option::is_some),
            "Could not resolve:\n{:?}\n[{}]", mapping, fake2real.iter().format(", "));

    let mapping = mapping.map(Option::unwrap);

    // translate outputs
    output.split_whitespace()
        .map(|s| s.parse::<SegBits>().unwrap().iter_segs().map(|sg| mapping[sg as usize]).collect())
        .map(|sb| DIGITS.into_iter()
                       .find_position(|&d| d == sb)
                       .ok_or_else(|| Result::<(), _>::Err(format!("{}", sb)))
                       .expect("No match for output").0 as u32)
        .take(4)
        .fold(0u32, |acc, v| acc * 10 + v)
}

pub fn run() -> GenResult {
    let input = load_input_string(DAY)?;
    let input_iter = input.trim().split('\n');

    let part1: usize =
        input_iter
            .clone()
            .map(|line|
                line
                    .rsplit_once(" | ")
                    .unwrap().1
                    .split_whitespace()
                    .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                    .count())
                .sum();

    println!("Part 1: {}", part1);

    let part2: u32 = input_iter.map(determine_digits).sum();

    println!("Part 2: {}", part2);

    Ok(())
}
