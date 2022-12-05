use std::io::Read;

use aoc_utils::BufferedInput;
use scan_fmt::scan_fmt;

fn parse_stacks(raw: &str) -> Vec<Vec<char>> {
    let numbers = raw.lines().last().unwrap();

    let n: usize = numbers.split_whitespace().last().unwrap().parse().unwrap();

    let stacks_rev = raw.lines().rev().skip(1);
    let mut result = vec![vec![]; n];

    for line in stacks_rev {
        let bytes = line.as_bytes();
        let indices = itertools::iterate(1, |&i| i + 4).take(n);

        for (stack_no, i) in indices.enumerate() {
            let b = bytes[i];

            if b != b' ' {
                result[stack_no].push(b.into());
            }
        }
    }

    result
}

#[anyhoo::anyhoo]
fn parse_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let mut input = BufferedInput::parse_args("Day 5: Supply Stacks - Part 1")?;

    let mut raw = String::new();
    input.read_to_string(&mut raw)?;

    let (stacks, moves) = raw.split_once("\n\n").unwrap();

    let stacks = parse_stacks(stacks);

    let moves = moves
        .lines()
        .map(|l| {
            let (n, from, to) = scan_fmt!(l, "move {d} from {d} to {d}", _, _, _).unwrap();

            (from, to, n)
        })
        .collect();

    (stacks, moves)
}

// (from, to, n)
type Move = (usize, usize, usize);

#[anyhoo::anyhoo]
fn main() {
    let (mut stacks, moves) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        for (from, to, n) in moves {
            let from = &mut stacks[from - 1];
            let split_index = from.len() - n;

            let transfered = from.split_off(split_index);

            let to = &mut stacks[to - 1];

            to.extend(transfered.into_iter().rev());
        }

        let answer: String = stacks.iter().map(|s| s.last().unwrap()).collect();

        answer
    });
}
