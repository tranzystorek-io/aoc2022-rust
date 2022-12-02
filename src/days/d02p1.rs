use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Move, Move)> {
    let input = BufferedInput::parse_args("Day 2: Rock Paper Scissors - Part 1")?;

    input
        .lines()
        .map_ok(|l| {
            let (their, mine) = scan_fmt!(&l, "{} {}", String, String).unwrap();

            let their = match their.as_str() {
                "A" => Move::R,
                "B" => Move::P,
                "C" => Move::S,
                _ => unreachable!(),
            };
            let mine = match mine.as_str() {
                "X" => Move::R,
                "Y" => Move::P,
                "Z" => Move::S,
                _ => unreachable!(),
            };

            (their, mine)
        })
        .try_collect()?
}

#[derive(Clone, Copy)]
enum Move {
    R,
    P,
    S,
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }
    }

    fn spar(&self, other: &Self) -> usize {
        match (self, other) {
            (Self::R, Self::S) | (Self::P, Self::R) | (Self::S, Self::P) => 6,
            (Self::R, Self::R) | (Self::P, Self::P) | (Self::S, Self::S) => 3,
            _ => 0,
        }
    }
}

#[anyhoo::anyhoo]
fn main() {
    let strategy = parse_input()?;

    aoc_utils::measure_and_print(|| {
        strategy
            .into_iter()
            .map(|(their, mine)| mine.score() + mine.spar(&their))
            .sum::<usize>()
    });
}
