use std::io::Read;

use aoc_utils::BufferedInput;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<usize>> {
    let mut input = BufferedInput::parse_args("Day 1: Calorie Counting - Part 2")?;

    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();

    let split = s.split("\n\n");
    let result = split
        .map(|set| {
            set.split('\n')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect()
        })
        .collect();

    result
}

#[anyhoo::anyhoo]
fn main() {
    let calories_carried = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut sums: Vec<_> = calories_carried
            .into_iter()
            .map(|elf| elf.into_iter().sum::<usize>())
            .collect();

        sums.sort_unstable();

        let index = sums.len() - 3;
        let result: usize = sums[index..].iter().sum();

        result
    });
}
