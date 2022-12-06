use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<u8> {
    let input = BufferedInput::parse_args("Day 6: Tuning Trouble - Part 1")?;

    input.unwrapped_lines().next().unwrap().into_bytes()
}

fn is_sop_marker(slice: &[u8]) -> bool {
    slice.iter().copied().all_unique()
}

#[anyhoo::anyhoo]
fn main() {
    let stream = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let start = stream.windows(4).position(is_sop_marker).unwrap();

        start + 4
    });
}
