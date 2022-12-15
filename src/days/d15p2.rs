use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Position, Position)> {
    let input = BufferedInput::parse_args("Day 15: Beacon Exclusion Zone - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            let (sx, sy, bx, by) = scan_fmt!(
                &line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                _,
                _,
                _,
                _
            )
            .unwrap();

            ((sx, sy), (bx, by))
        })
        .try_collect()?
}

const MIN_COORD: isize = 0;
const MAX_COORD: isize = 4_000_000;
const TUNING_FACTOR: isize = 4_000_000;

type Position = (isize, isize);

fn manhattan((x, y): Position, (t_x, t_y): Position) -> isize {
    isize::abs_diff(x, t_x) as isize + isize::abs_diff(y, t_y) as isize
}

fn search(report: &[(Position, Position)]) -> Position {
    let sensors = report
        .iter()
        .map(|&(sensor, beacon)| (sensor, manhattan(sensor, beacon)))
        .collect_vec();

    for y in MIN_COORD..=MAX_COORD {
        let spans_x = sensors
            .iter()
            .filter_map(|&((sx, sy), radius)| {
                let vert = isize::abs_diff(y, sy) as isize;

                if vert > radius {
                    return None;
                }

                let span = radius - vert;
                let lower = std::cmp::max(MIN_COORD, sx - span);
                let upper = std::cmp::min(MAX_COORD, sx + span);

                Some((lower, upper))
            })
            .sorted()
            .coalesce(|(a, b), (c, d)| {
                if d <= b {
                    Ok((a, b))
                } else if c <= b {
                    Ok((a, d))
                } else {
                    Err(((a, b), (c, d)))
                }
            });

        if let Some(((_, upper), _)) = spans_x.collect_tuple() {
            return (upper + 1, y);
        }
    }

    panic!("Could not find distress beacon");
}

#[anyhoo::anyhoo]
fn main() {
    let report = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (x, y) = search(&report);

        x * TUNING_FACTOR + y
    });
}
