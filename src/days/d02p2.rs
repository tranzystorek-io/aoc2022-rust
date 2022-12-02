use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Move, Outcome)> {
    let input = BufferedInput::parse_args("Day 2: Rock Paper Scissors - Part 2")?;

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
            let outcome = match mine.as_str() {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!(),
            };

            (their, outcome)
        })
        .try_collect()?
}

#[derive(Clone, Copy)]
enum Move {
    R,
    P,
    S,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }
    }

    fn predict(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (Self::R, Outcome::Draw) | (Self::P, Outcome::Lose) | (Self::S, Outcome::Win) => {
                Self::R
            }
            (Self::P, Outcome::Draw) | (Self::R, Outcome::Win) | (Self::S, Outcome::Lose) => {
                Self::P
            }
            _ => Self::S,
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[anyhoo::anyhoo]
fn main() {
    let strategy = parse_input()?;

    aoc_utils::measure_and_print(|| {
        strategy
            .into_iter()
            .map(|(their, outcome)| outcome.score() + their.predict(&outcome).score())
            .sum::<usize>()
    });
}
