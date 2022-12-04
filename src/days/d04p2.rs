use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Range, Range)> {
    let input = BufferedInput::parse_args("Day 4: Camp Cleanup - Part 2")?;

    input
        .lines()
        .map_ok(|l| {
            let (a, b, c, d) = scan_fmt!(&l, "{d}-{d},{d}-{d}", _, _, _, _).unwrap();

            ((a, b), (c, d))
        })
        .try_collect()?
}

type Range = (u32, u32);

fn overlaps((a, b): Range, (c, d): Range) -> bool {
    let r = a..=b;
    r.contains(&c) || r.contains(&d)
}

#[anyhoo::anyhoo]
fn main() {
    let pairs = parse_input()?;

    aoc_utils::measure_and_print(|| {
        pairs
            .into_iter()
            .filter(|&(u, v)| overlaps(u, v) || overlaps(v, u))
            .count()
    });
}
