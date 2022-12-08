use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Grid<u32> {
    let input = BufferedInput::parse_args("Day 8: Treetop Tree House - Part 1")?;

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

    fn len(&self) -> usize {
        self.values.len()
    }

    fn at(&self, x: usize, y: usize) -> &T {
        let index = y * self.width + x;

        &self.values[index]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = y * self.width + x;

        &mut self.values[index]
    }
}

fn traverse(grid: &Grid<u32>) -> Grid<bool> {
    let width = grid.width();
    let mut result = Grid::new(vec![false; grid.len()], width);

    for x in 0..width {
        let mut max = 0;
        for y in 0..width {
            let &v = grid.at(x, y);
            let dest = result.at_mut(x, y);
            *dest = *dest || v > max;

            max = std::cmp::max(max, v);
        }

        let mut max = 0;
        for y in (0..width).rev() {
            let &v = grid.at(x, y);
            let dest = result.at_mut(x, y);
            *dest = *dest || v > max;

            max = std::cmp::max(max, v);
        }
    }

    for y in 0..width {
        let mut max = 0;
        for x in 0..width {
            let &v = grid.at(x, y);
            let dest = result.at_mut(x, y);
            *dest = *dest || v > max;

            max = std::cmp::max(max, v);
        }

        let mut max = 0;
        for x in (0..width).rev() {
            let &v = grid.at(x, y);
            let dest = result.at_mut(x, y);
            *dest = *dest || v > max;

            max = std::cmp::max(max, v);
        }
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let grid = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let visibility = traverse(&grid);
        let width = visibility.width();

        let inner = itertools::iproduct!(1..width - 1, 1..width - 1);

        let inner_visible = inner.filter(|&(x, y)| *visibility.at(x, y)).count();
        let outer = 4 * width - 4;

        inner_visible + outer
    });
}
