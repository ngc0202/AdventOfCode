use std::cmp;

use crate::prelude::*;

day!(1);

pub fn run() -> GenResult {
    let (top1, top2, top3) = process_inputs(DAY, |lines|
        itertools::process_results(lines
            .into_iter()
            .group_by(|l| l.trim().is_empty())
            .into_iter()
            .filter_map(|(sep, elf)| (!sep).then_some(elf))
            .map(|mut elf| elf.try_fold(0u64, |acc, cal| cal.trim().parse::<u64>().map(|c| acc + c))),
            |sums| sums.map(cmp::Reverse)
                    .k_smallest(3)
                    .into_iter()
                    .map(|v| v.0)
                    .collect_tuple()
                    .ok_or(NoInputError))
    )???;

    println!("Part 1: {top1}\nPart 2: {}", top1 + top2 + top3);

    Ok(())
}
