use std::collections::HashSet;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Move> {
    let input = BufferedInput::parse_args("Day 9: Rope Bridge - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            let (d, n) = scan_fmt!(&line, "{} {d}", _, _).unwrap();

            match d {
                'L' => Move::Left(n),
                'R' => Move::Right(n),
                'U' => Move::Up(n),
                'D' => Move::Down(n),
                _ => unreachable!(),
            }
        })
        .try_collect()?
}

type Position = (i64, i64);

#[derive(Clone, Copy)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl Move {
    fn n(&self) -> usize {
        match *self {
            Move::Left(n) | Move::Right(n) | Move::Up(n) | Move::Down(n) => n,
        }
    }

    fn dir(&self) -> (i64, i64) {
        match self {
            Move::Left(_) => (-1, 0),
            Move::Right(_) => (1, 0),
            Move::Up(_) => (0, 1),
            Move::Down(_) => (0, -1),
        }
    }
}

fn tail_dir((xh, yh): Position, (xt, yt): Position) -> (i64, i64) {
    match (xh - xt, yh - yt) {
        (-1..=1, -1..=1) => (0, 0),
        (xd, yd) => (xd.signum(), yd.signum()),
    }
}

fn single_move(rope: &mut [Position], visited: &mut HashSet<Position>, m: Move) {
    let n = m.n();
    let (dir_x, dir_y) = m.dir();
    let len = rope.len();

    for _ in 0..n {
        // move head
        let (head_x, head_y) = &mut rope[0];
        *head_x += dir_x;
        *head_y += dir_y;

        // move tail
        for i in 0..(len - 1) {
            let (xh, yh) = rope[i];
            let (xt, yt) = &mut rope[i + 1];

            let (tail_step_x, tail_step_y) = tail_dir((xh, yh), (*xt, *yt));
            *xt += tail_step_x;
            *yt += tail_step_y;
        }

        let true_tail = rope[len - 1];
        visited.insert(true_tail);
    }
}

fn simulate(moves: &[Move]) -> HashSet<Position> {
    let mut result = [(0, 0)].into();
    let mut rope = [(0, 0); 10];

    for &m in moves {
        single_move(&mut rope, &mut result, m);
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let moves = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let v = simulate(&moves);

        v.len()
    });
}
