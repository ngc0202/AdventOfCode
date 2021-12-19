use crate::prelude::*;
use std::cmp::Ord;
use std::ops::{Add, Mul};

use nom::{self, error::Error, Finish, IResult};

const DAY: u8 = 16;

pub fn run() -> GenResult {
    let input = crate::load_input_string(DAY)?
        .trim()
        .chars()
        .map(|n| n.to_digit(16).unwrap() as u8)
        .tuples()
        .map(|(a, b)| (a << 4) + b)
        .collect_vec();

    let (part1, part2) = parse_transmission(&input).unwrap();
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}

fn parse_transmission(input: &[u8]) -> Result<(u64, u64), Error<&[u8]>> {
    nom::bits::<_, _, Error<(&[u8], usize)>, _, _>(parse_packet)(input)
        .finish()
        .map(|(_, x)| x)
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), (u64, u64)> {
    let (input, (version, ptype)): (_, (u64, u8)) = nom::sequence::tuple((
        nom::bits::complete::take(3usize),
        nom::bits::complete::take(3usize),
    ))(input)?;

    let (input, (vsn, value)) = if ptype == 4 {
        let (input, literal) = parse_literal(input)?;
        (input, (0, literal))
    } else {
        let (input, length_type): (_, u8) = nom::bits::complete::take(1usize)(input)?;
        let (init_fn, fold_fn) = get_operation(ptype);
        if length_type == 0 {
            let (input, num_bits): (_, usize) = nom::bits::complete::take(15usize)(input)?;
            let (sp_input, rst_input) = take_split_bits(input, num_bits);
            let (_, res) = nom::multi::fold_many1(parse_packet, init_fn, fold_fn)(sp_input)?;
            (rst_input, res)
        } else {
            let (input, num_packets): (_, usize) = nom::bits::complete::take(11usize)(input)?;
            nom::multi::fold_many_m_n(num_packets, num_packets, parse_packet, init_fn, fold_fn)(
                input,
            )?
        }
    };

    Ok((input, (version + vsn, value)))
}

fn get_operation(
    ptype: u8,
) -> (
    impl FnMut() -> (u64, u64),
    impl FnMut((u64, u64), (u64, u64)) -> (u64, u64),
) {
    let (init, fold_fn_inner): (u64, fn(u64, u64) -> u64) = match ptype {
        0 => (0, Add::add),
        1 => (1, Mul::mul),
        2 => (u64::MAX, Ord::min),
        3 => (0, Ord::max),
        5 => (u64::MAX, |acc, v| if acc == u64::MAX { v } else if acc > v { 1 } else { 0 }),
        6 => (u64::MAX, |acc, v| if acc == u64::MAX { v } else if acc < v { 1 } else { 0 }),
        7 => (u64::MAX, |acc, v| if acc == u64::MAX { v } else if acc == v { 1 } else { 0 }),
        _ => panic!("Invalid packet type: {}", ptype),
    };

    let fold_fn =
        move |acc: (u64, u64), val: (u64, u64)| (acc.0 + val.0, fold_fn_inner(acc.1, val.1));
    let init_fn = move || (0u64, init);

    (init_fn, fold_fn)
}

fn take_split_bits(input: (&[u8], usize), count: usize) -> ((&[u8], usize), (&[u8], usize)) {
    let bit_pos = input.1 + count;
    let (byte_offset, bit_offset) = (bit_pos / 8, bit_pos % 8);
    (
        (&input.0[0..=byte_offset], input.1),
        (&input.0[byte_offset..], bit_offset),
    )
}

fn parse_literal(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    let mut literal = 0u64;
    loop {
        let (input2, cont): (_, u8) = nom::bits::complete::take(1usize)(input)?;
        let (input3, bits): (_, u64) = nom::bits::complete::take(4usize)(input2)?;
        input = input3;
        literal <<= 4;
        literal |= bits;
        if cont == 0 {
            return Ok((input3, literal));
        }
    }
}
