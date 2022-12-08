use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Grid<u32> {
    let input = BufferedInput::parse_args("Day 8: Treetop Tree House - Part 2")?;

    let mut width_found = false;
    let mut width = 0;
    let values = input
        .unwrapped_lines()
        .inspect(|line| {
            if !width_found {
                width = line.len();
                width_found = true;
            }
        })
        .flat_map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect_vec()
        })
        .collect();

    Grid::new(values, width)
}

struct Grid<T> {
    values: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn new(values: Vec<T>, width: usize) -> Self {
        Self { values, width }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn at(&self, x: usize, y: usize) -> &T {
        let index = y * self.width + x;

        &self.values[index]
    }
}

fn scenic_score(grid: &Grid<u32>, x: usize, y: usize) -> usize {
    let width = grid.width();
    if x == 0 || x == width - 1 || y == 0 || y == width - 1 {
        return 0;
    }

    let mut result = 1;
    let my_tallness = grid.at(x, y);

    let mut dist = 0;
    for x in (0..x).rev() {
        dist += 1;

        let v = grid.at(x, y);
        if v >= my_tallness {
            break;
        }
    }
    result *= dist;

    let mut dist = 0;
    for x in (x + 1)..width {
        dist += 1;

        let v = grid.at(x, y);
        if v >= my_tallness {
            break;
        }
    }
    result *= dist;

    let mut dist = 0;
    for y in (0..y).rev() {
        dist += 1;

        let v = grid.at(x, y);
        if v >= my_tallness {
            break;
        }
    }
    result *= dist;

    let mut dist = 0;
    for y in (y + 1)..width {
        dist += 1;

        let v = grid.at(x, y);
        if v >= my_tallness {
            break;
        }
    }
    result *= dist;

    result
}

#[anyhoo::anyhoo]
fn main() {
    let grid = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let width = grid.width();

        itertools::iproduct!(1..width - 1, 1..width - 1)
            .map(|(x, y)| scenic_score(&grid, x, y))
            .max()
            .unwrap()
    });
}
