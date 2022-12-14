use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Path> {
    let input = BufferedInput::parse_args("Day 14: Regolith Reservoir - Part 1")?;

    input
        .lines()
        .map_ok(|line| {
            let split = line.split(" -> ");

            split
                .map(|s| {
                    let (x, y) = s.split_once(',').unwrap();

                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .try_collect()?
}

const START_POS: Position = (500, 0);

type Position = (usize, usize);
type Path = Vec<Position>;

struct Grid<T> {
    contents: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn at(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);

        let index = y * self.width + x;

        &self.contents[index]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < self.width);

        let index = y * self.width + x;

        &mut self.contents[index]
    }
}

#[derive(Clone, Copy)]
enum Space {
    Air,
    Rock,
    Sand,
}

impl Space {
    fn is_air(&self) -> bool {
        matches!(self, Self::Air)
    }
}

fn max_coords(paths: &[Path]) -> (usize, usize) {
    paths
        .iter()
        .flatten()
        .fold((0, 0), |(max_x, max_y), &(x, y)| {
            let new_x = std::cmp::max(max_x, x);
            let new_y = std::cmp::max(max_y, y);

            (new_x, new_y)
        })
}

fn line((s_x, s_y): Position, (e_x, e_y): Position) -> Vec<Position> {
    match ((s_x, s_y), (e_x, e_y)) {
        ((_, start), (_, end)) if s_x == e_x => {
            let l_start = std::cmp::min(start, end);
            let l_end = std::cmp::max(start, end);

            (l_start..=l_end).map(|y| (s_x, y)).collect()
        }
        ((start, _), (end, _)) if s_y == e_y => {
            let l_start = std::cmp::min(start, end);
            let l_end = std::cmp::max(start, end);

            (l_start..=l_end).map(|x| (x, s_y)).collect()
        }
        _ => unreachable!(),
    }
}

fn place_rocks(paths: &[Path], max_x: usize, max_y: usize) -> Grid<Space> {
    let width = max_x + 1;
    let height = max_y + 1;
    let contents = vec![Space::Air; width * height];
    let mut grid = Grid { contents, width };

    for path in paths {
        for (start, end) in path.iter().copied().tuple_windows() {
            for (x, y) in line(start, end) {
                let v = grid.at_mut(x, y);
                *v = Space::Rock;
            }
        }
    }

    grid
}

fn pour_sand_unit(cave: &mut Grid<Space>, max_y: usize) -> bool {
    let (mut sand_x, mut sand_y) = START_POS;
    let scan_x = [0, -1, 1];

    loop {
        let new_y = sand_y + 1;

        let maybe_target = scan_x.into_iter().find_map(|dx| {
            let scanned = sand_x as isize + dx;

            let space = cave.at(scanned as usize, new_y);
            space.is_air().then_some((scanned as usize, new_y))
        });

        match maybe_target {
            Some((_, y)) if y >= max_y => {
                return true;
            }
            Some((x, y)) => {
                sand_x = x;
                sand_y = y;
            }
            None => {
                let v = cave.at_mut(sand_x, sand_y);
                *v = Space::Sand;
                return false;
            }
        }
    }
}

#[anyhoo::anyhoo]
fn main() {
    let paths = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (max_x, max_y) = max_coords(&paths);
        let max_x = max_x + 1;
        let mut cave = place_rocks(&paths, max_x, max_y);

        std::iter::repeat_with(|| pour_sand_unit(&mut cave, max_y))
            .take_while(|abyss_flow| !abyss_flow)
            .count()
    });
}
