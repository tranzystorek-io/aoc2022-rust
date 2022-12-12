use std::collections::{HashSet, VecDeque};

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Grid<u8> {
    let input = BufferedInput::parse_args("Day 12: Hill Climbing Algorithm - Part 2")?;
    let mut width = 0;
    let mut width_found = false;

    let contents = input
        .unwrapped_lines()
        .inspect(|line| {
            if !width_found {
                width = line.len();
                width_found = true;
            }
        })
        .flat_map(|line| {
            line.bytes()
                .map(|b| match b {
                    END_MARKER => END_MARKER,
                    START_MARKER => 0,
                    v @ b'a'..=b'z' => v - b'a',
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = contents.len() / width;

    Grid {
        contents,
        width,
        height,
    }
}

const START_MARKER: u8 = b'S';
const END_MARKER: u8 = b'E';

type Position = (usize, usize);

struct Grid<T> {
    contents: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn at(&self, x: usize, y: usize) -> &T {
        let index = y * self.width + x;

        &self.contents[index]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = y * self.width + x;

        &mut self.contents[index]
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let width = self.width as isize;
        let height = self.height as isize;

        dirs.into_iter().filter_map(move |(dx, dy)| {
            let target_x: isize = x as isize + dx;
            let target_y: isize = y as isize + dy;

            if target_x < 0 || target_x >= width || target_y < 0 || target_y >= height {
                return None;
            }

            Some((target_x as _, target_y as _))
        })
    }
}

fn find_end(grid: &mut Grid<u8>) -> Position {
    let width = grid.width();
    let height = grid.height();

    for y in 0..height {
        for x in 0..width {
            let v = grid.at_mut(x, y);

            if *v == END_MARKER {
                *v = b'z' - b'a';
                return (x, y);
            }
        }
    }

    panic!();
}

fn traverse_from_end(grid: &Grid<u8>, end: Position) -> Option<usize> {
    let mut searchspace: VecDeque<_> = [(end, 0)].into();
    let mut visited = HashSet::new();

    while let Some(((current_x, current_y), len)) = searchspace.pop_front() {
        let &current_elev = grid.at(current_x, current_y);

        if !visited.insert((current_x, current_y)) {
            continue;
        }

        for (next_x, next_y) in grid.neighbors(current_x, current_y) {
            if visited.contains(&(next_x, next_y)) {
                continue;
            }

            let &next_elev = grid.at(next_x, next_y);

            if next_elev < current_elev - 1 {
                continue;
            }

            if next_elev == 0 {
                return Some(len + 1);
            }

            searchspace.push_back(((next_x, next_y), len + 1));
        }
    }

    None
}

#[anyhoo::anyhoo]
fn main() {
    let mut grid = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let end = find_end(&mut grid);

        traverse_from_end(&grid, end).unwrap()
    });
}
