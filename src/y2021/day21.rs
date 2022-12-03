
use crate::prelude::*;
use cached::proc_macro::cached;

day!(21);

static UNIS: &[(u16, u64)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
const PT_CAP: u16 = 21;


pub fn run() -> GenResult {
    // Starting positions
    let player1 = Player::new(9);
    let player2 = Player::new(10);

    let part1 = count_rolls(player1, player2);
    println!("Part 1: {}", part1);

	let start = std::time::Instant::now();
    let part2 = count_universes(player1, player2);
	let dur = start.elapsed();
    println!("Part 2: {} ({:?}) ({} iters)", part2, dur, COUNT.load(Ordering::Relaxed));

    Ok(())
}

fn count_rolls(mut ply1: Player, mut ply2: Player) -> u64 {
    let mut ddie = DetDie::new();
    loop {
        // Player 1
        if ply1.turn(&mut ddie).unwrap() >= 1000 {
            return ddie.rolls * u64::from(ply2.score);
        }

        // Player 2
        if ply2.turn(&mut ddie).unwrap() >= 1000 {
            return ddie.rolls * u64::from(ply1.score);
        }
    }
}

fn count_universes(ply1: Player, ply2: Player) -> u64 {
    let (a, b) = inner_unis2(ply1, ply2, true);
    a.max(b)
}

use std::sync::atomic::{AtomicU64, Ordering};
static COUNT: AtomicU64 = AtomicU64::new(0);

#[cached]
fn inner_unis2(ply1: Player, ply2: Player, turn: bool) -> (u64, u64) {
	COUNT.fetch_add(UNIS.len() as u64, Ordering::Relaxed);
    UNIS.iter()
        .map(move |&(n, c)| {
            if turn {
                // Player 1
                let ply1 = ply1.turn3(n);
                if ply1.score >= PT_CAP {
                    (c, 0)
                } else {
                    let r1 = inner_unis2(ply1, ply2, false);
                    (c * r1.0, c * r1.1)
                }
            } else {
                // Player 2
                let ply2 = ply2.turn3(n);
                if ply2.score >= PT_CAP {
                    (0, c)
                } else {
                    let r2 = inner_unis2(ply1, ply2, true);
                    (c * r2.0, c * r2.1)
                }
            }
        })
        .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Player {
    pos: u8,
    score: u16
}

impl Player {
    pub fn new(pos: u8) -> Self {
        Player { pos, score: 0 }
    }

    #[inline]
    pub fn turn<D: Iterator<Item=u16>>(&mut self, die: &mut D) -> Option<u16> {
        let sum = die.next_tuple().map(|(a, b, c)| wrap10(a+b+c))?;
        *self = self.turn3(sum);
        Some(self.score)
    }

    pub fn turn3(self, sum: u16) -> Self {
        let pos = wrap10(u16::from(self.pos) + sum) as u8;
        let score = self.score + u16::from(pos);
        Player { pos, score }
    }
}

#[derive(Clone, Debug)]
struct DetDie {
    state: u16,
    rolls: u64
}

impl Iterator for DetDie {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        Some(self.roll())
    }
}

impl DetDie {
    pub fn new() -> Self {
        Self { state: 1, rolls: 0 }
    }

    pub fn roll(&mut self) -> u16 {
        self.rolls += 1;
        let ret = self.state;
        self.state = self.state % 100 + 1;
        ret
    }
}

#[inline]
fn wrap10(n: u16) -> u16 {
    ((n - 1) % 10) + 1
}