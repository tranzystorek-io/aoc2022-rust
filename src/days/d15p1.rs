use std::collections::HashSet;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Position, Position)> {
    let input = BufferedInput::parse_args("Day 15: Beacon Exclusion Zone - Part 1")?;

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

const SCANNED_ROW: isize = 2_000_000;

type Position = (isize, isize);
type Scan = HashSet<isize>;

fn manhattan((x, y): Position, (t_x, t_y): Position) -> isize {
    isize::abs_diff(x, t_x) as isize + isize::abs_diff(y, t_y) as isize
}

fn scan_row(report: &[(Position, Position)], row: isize) -> Scan {
    let mut result = Scan::new();

    for &(sensor @ (sx, sy), beacon) in report {
        let radius = manhattan(sensor, beacon);
        let dist_from_row = isize::abs_diff(sy, row) as isize;

        if dist_from_row > radius {
            continue;
        }

        let span = radius - dist_from_row;
        let sweeped = (-span..=span).map(|dx| sx + dx);

        result.extend(sweeped)
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let report = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let excluded_beacons = report
            .iter()
            .copied()
            .map(|(_, beacon)| beacon)
            .filter(|&(_, y)| y == SCANNED_ROW)
            .collect_vec();
        let scanned = scan_row(&report, SCANNED_ROW);

        scanned
            .iter()
            .filter(|&&x| !excluded_beacons.contains(&(x, SCANNED_ROW)))
            .count()
    });
}
