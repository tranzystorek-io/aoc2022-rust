use std::collections::HashSet;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u8>> {
    let input = BufferedInput::parse_args("Day 3: Rucksack Reorganization - Part 2")?;

    input.lines().map_ok(|l| l.into_bytes()).try_collect()?
}

fn priority(item: u8) -> usize {
    match item {
        b'a'..=b'z' => (item - b'a') as usize + 1,
        b'A'..=b'Z' => (item - b'A') as usize + 27,
        _ => unreachable!(),
    }
}

#[anyhoo::anyhoo]
fn main() {
    let sacks = parse_input()?;

    aoc_utils::measure_and_print(|| {
        sacks
            .into_iter()
            .tuples()
            .map(|(a, b, c)| {
                let a: HashSet<u8> = a.into_iter().collect();
                let b: HashSet<u8> = b.into_iter().collect();
                let c: HashSet<u8> = c.into_iter().collect();

                let intersect: HashSet<u8> = a.intersection(&b).copied().collect();
                let badge = c.intersection(&intersect).copied().next().unwrap();

                priority(badge)
            })
            .sum::<usize>()
    });
}
