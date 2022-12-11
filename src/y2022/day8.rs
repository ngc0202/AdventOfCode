use crate::prelude::*;

day!(8);

fn parse_input() -> Result<Vec<Vec<i8>>, Whatever> {
    whatever!(load_input_bytes(DAY), "Failed reading input file")
        .split(|&b| b == b'\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.iter()
                .map(|&b| {
                    (b'0'..=b'9')
                        .contains(&b)
                        .then_some((b - b'0') as i8)
                        .whatever_context("Height must be 0-9")
                })
                .collect()
        })
        .collect()
}

fn set_line_height<'a, 'b>(
    it: impl IntoIterator<Item = &'a i8>,
    hit: impl IntoIterator<Item = &'b mut i8>,
) {
    let mut max = -1_i8;
    for (&val, hgt) in it.into_iter().zip(hit) {
        *hgt = max.min(*hgt);
        max = max.max(val);
    }
}

fn calc_height_map(map: &[Vec<i8>]) -> Vec<Vec<i8>> {
    let mut hmap = map
        .iter()
        .map(|row| {
            let mut hrow = row.clone();
            set_line_height(row, &mut hrow);
            set_line_height(row.iter().rev(), hrow.iter_mut().rev());
            hrow
        })
        .collect_vec();

    // assumes rectangular
    for i in 0..map.len() {
        let col = map.iter().map(|row| &row[i]);
        set_line_height(col.clone(), hmap.iter_mut().map(|row| &mut row[i]));
        set_line_height(col.rev(), hmap.iter_mut().map(|row| &mut row[i]).rev());
    }

    hmap
}

fn score_dir<'a>(hgt: i8, iter: impl IntoIterator<Item = &'a i8>) -> u64 {
    let mut c = 0;
    for &val in iter {
        c += 1;
        if val >= hgt {
            break;
        }
    }
    c
}

fn score_view(map: &[Vec<i8>], x: usize, y: usize) -> u64 {
    let row = &map[y];
    let hgt = row[x];
    let right = score_dir(hgt, &row[x + 1..]);
    let left = score_dir(hgt, row[0..x].iter().rev());
    let ymax = map.len();
    let up = score_dir(hgt, (0..y).rev().map(|y| &map[y][x]));
    let down = score_dir(hgt, (y + 1..ymax).map(|y| &map[y][x]));
    right * left * up * down
}

pub fn run() -> Result<(), Whatever> {
    let input = &parse_input()?;
    let heights = calc_height_map(input);

    let part1 = input
        .iter()
        .flatten()
        .zip(heights.iter().flatten())
        .fold(0u64, |acc, (m, h)| acc + (m > h) as u64);

    println!("Part 1: {part1}");

    let part2 = (0..input.len())
        .flat_map(|y| (0..input[0].len()).map(move |x| score_view(input, x, y)))
        .max()
        .unwrap();

    println!("Part 2: {part2}");

    Ok(())
}
