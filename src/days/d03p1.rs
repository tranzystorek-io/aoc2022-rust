use std::collections::HashSet;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u8>> {
    let input = BufferedInput::parse_args("Day 3: Rucksack Reorganization - Part 1")?;

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
            .map(|mut s| {
                let split_index = s.len() / 2;
                let second = s.split_off(split_index);

                let set_one: HashSet<u8> = s.into_iter().collect();
                let set_two: HashSet<u8> = second.into_iter().collect();

                let common = set_one.intersection(&set_two).copied().next().unwrap();

                priority(common)
            })
            .sum::<usize>()
    });
}
