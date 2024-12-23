use super::Solution;
use crate::utils::NomFail;
use enum_map::{Enum, EnumArray, EnumMap};
use smallvec::SmallVec;

day!(run 21);

type Coord = crate::utils::Coord<u8>;

struct Day21 {
    codes: SmallVec<[Input; 5]>,
}

struct Input {
    code: [NumPad; 4],
    val: u16,
}

impl<'i> Solution<'i> for Day21 {
    fn parse(input: &'i mut Vec<u8>) -> Result<Self, NomFail> {
        parse::parse(input).map(|codes| Self { codes })
    }

    fn part1(&mut self) -> u64 {
        self.process_inputs(2)
    }

    fn part2(&mut self) -> u64 {
        self.process_inputs(25)
    }
}

impl Day21 {
    pub fn process_inputs(&self, n: usize) -> u64 {
        let cache = build_cache(n);
        let mut ans = 0;
        for input in &self.codes {
            let mut last = NumPad::Activate;
            let mut sum = 0;
            for button in input.code {
                sum += cache.get(last, button);
                last = button;
            }
            ans += sum * u64::from(input.val);
        }
        ans
    }
}

fn build_cache(n: usize) -> Cache<NumPad> {
    let mut prev_cache = Cache::default();
    let mut cur_cache = if n > 0 {
        Cache::with_dists()
    } else {
        Cache::ones()
    };

    // Build robot cache
    for _ in 1..n {
        std::mem::swap(&mut prev_cache, &mut cur_cache);
        for (a, arr) in &mut cur_cache.map {
            for (b, dist) in arr {
                *dist = calc_dist::<DirPad>(a, b, &prev_cache);
            }
        }
    }

    // Build door cache
    let map = EnumMap::from_fn(|a| EnumMap::from_fn(|b| calc_dist::<NumPad>(a, b, &cur_cache)));

    Cache { map }
}

fn calc_dist<K: Keypad>(src: K, dst: K, cache: &Cache<DirPad>) -> u64 {
    use DirPad::Activate;

    let mut total_dist = 0u64;
    let mut add = |a, b| total_dist += cache.get(a, b);
    let path = possible_paths::<K>(src, dst, cache);
    match path {
        Path::Single { key, dist } => {
            add(Activate, key);
            for _ in 1..dist {
                add(key, key);
            }
            add(key, Activate);
        }
        Path::Double {
            key1,
            dist1,
            key2,
            dist2,
        } => {
            add(Activate, key1);
            for _ in 1..dist1 {
                add(key1, key1);
            }
            add(key1, key2);
            for _ in 1..dist2 {
                add(key2, key2);
            }
            add(key2, Activate);
        }
        Path::Activate => add(Activate, Activate),
    }

    total_dist
}

// ----------- Path Finding -----------

fn possible_paths<K: Keypad>(srcb: K, dstb: K, cache: &Cache<DirPad>) -> Path {
    use std::cmp::Ordering::*;
    use DirPad::*;

    let src = srcb.coord();
    let dst = dstb.coord();

    assert_ne!(src, K::GAP);

    // Check if at dst
    if src == dst {
        return Path::Activate;
    }

    let xdist = src.x.abs_diff(dst.x);
    let ydist = src.y.abs_diff(dst.y);

    let xcmp = dst.x.cmp(&src.x);
    let ycmp = dst.y.cmp(&src.y);

    let contains = |s: u8, d, g| (s.min(d)..=s.max(d)).contains(&g);

    // Avoid gap and consult cache
    let choose = |x, y| {
        let yfirst = src.y == K::GAP.y && contains(src.x, dst.x, K::GAP.x);
        let xfirst = src.x == K::GAP.x && contains(src.y, dst.y, K::GAP.y);

        if xfirst || (!yfirst && cache.get(x, y) < cache.get(y, x)) {
            Path::double(x, xdist, y, ydist)
        } else {
            Path::double(y, ydist, x, xdist)
        }
    };

    match (xcmp, ycmp) {
        (Equal, Equal) => Path::Activate,
        (Equal, Less) => Path::single(Down, ydist),
        (Equal, Greater) => Path::single(Up, ydist),
        (Less, Equal) => Path::single(Left, xdist),
        (Greater, Equal) => Path::single(Right, xdist),
        (Less, Less) => choose(Left, Down),
        (Less, Greater) => choose(Left, Up),
        (Greater, Less) => choose(Right, Down),
        (Greater, Greater) => choose(Right, Up),
    }
}

#[derive(Debug)]
enum Path {
    Activate,
    Single {
        key: DirPad,
        dist: u8,
    },
    Double {
        key1: DirPad,
        dist1: u8,
        key2: DirPad,
        dist2: u8,
    },
}

impl Path {
    pub fn single(key: DirPad, dist: u8) -> Self {
        Self::Single { key, dist }
    }

    pub fn double(key1: DirPad, dist1: u8, key2: DirPad, dist2: u8) -> Self {
        Self::Double {
            key1,
            dist1,
            key2,
            dist2,
        }
    }
}

// ------------- CACHE -------------

struct Cache<B: Cachable> {
    map: EnumMap<B, EnumMap<B, u64>>,
}

impl<B: Cachable> Cache<B> {
    pub fn get(&self, a: B, b: B) -> u64 {
        self.map[a][b]
    }

    pub fn ones() -> Self {
        Self {
            map: EnumMap::from_fn(|_| EnumMap::from_fn(|_| 1)),
        }
    }
}

impl<B: Cachable + Keypad> Cache<B> {
    pub fn with_dists() -> Self {
        Self {
            map: EnumMap::from_fn(|a: B| {
                EnumMap::from_fn(|b: B| 1 + u64::from(a.coord().dist(b.coord())))
            }),
        }
    }
}

impl<B: Cachable> Default for Cache<B> {
    fn default() -> Self {
        Self {
            map: EnumMap::default(),
        }
    }
}

trait Cachable: EnumArray<u64> + EnumArray<EnumMap<Self, u64>> + PartialEq {}
impl<T: Enum + EnumArray<u64> + EnumArray<EnumMap<Self, u64>> + PartialEq> Cachable for T {}

// ------------- BUTTONS -------------

#[derive(Copy, Clone, Debug, Enum, PartialEq, Eq)]
enum DirPad {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

#[derive(Copy, Clone, Debug, Enum, PartialEq, Eq)]
enum NumPad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

// -------------  KEYPADS -------------
trait Keypad: Copy + Cachable {
    /// Location of the board's gap
    const GAP: Coord;

    /// Gets the coordinate to this key on the pad
    fn coord(self) -> Coord;
}

impl Keypad for NumPad {
    const GAP: Coord = Coord { x: 0, y: 0 };

    fn coord(self) -> Coord {
        // (0,0) is considered bottom left (gap)
        let (x, y) = match self {
            NumPad::Zero => (1, 0),
            NumPad::Activate => (2, 0),
            NumPad::One => (0, 1),
            NumPad::Two => (1, 1),
            NumPad::Three => (2, 1),
            NumPad::Four => (0, 2),
            NumPad::Five => (1, 2),
            NumPad::Six => (2, 2),
            NumPad::Seven => (0, 3),
            NumPad::Eight => (1, 3),
            NumPad::Nine => (2, 3),
        };
        Coord { x, y }
    }
}

impl Keypad for DirPad {
    const GAP: Coord = Coord { x: 0, y: 1 };

    fn coord(self) -> Coord {
        use DirPad::*;
        let (x, y) = match self {
            Left => (0, 0),
            Down => (1, 0),
            Right => (2, 0),
            Up => (1, 1),
            Activate => (2, 1),
        };
        Coord { x, y }
    }
}

mod parse {
    use crate::utils::{
        parser::{for_each, line, many_array},
        NomFail,
    };
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::u16,
        combinator::{all_consuming, value},
        Finish, IResult,
    };
    use smallvec::SmallVec;

    use super::{Input, NumPad};

    fn button(input: &[u8]) -> IResult<&[u8], NumPad> {
        use NumPad::*;
        alt((
            value(Activate, tag([b'A'])),
            value(Zero, tag([b'0'])),
            value(One, tag([b'1'])),
            value(Two, tag([b'2'])),
            value(Three, tag([b'3'])),
            value(Four, tag([b'4'])),
            value(Five, tag([b'5'])),
            value(Six, tag([b'6'])),
            value(Seven, tag([b'7'])),
            value(Eight, tag([b'8'])),
            value(Nine, tag([b'9'])),
        ))(input)
    }

    fn pinput(input: &[u8]) -> IResult<&[u8], Input> {
        let (i, code) = many_array(button)(input)?;
        let (_, val) = u16(input)?;
        let inp = Input { code, val };
        Ok((i, inp))
    }

    pub fn parse(input: &[u8]) -> Result<SmallVec<[Input; 5]>, NomFail> {
        let mut vec = SmallVec::new();
        _ = all_consuming(for_each(line(pinput), |inp| vec.push(inp)))(input).finish()?;
        Ok(vec)
    }
}
