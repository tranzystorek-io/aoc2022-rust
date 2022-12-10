use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Instruction> {
    let input = BufferedInput::parse_args("Day 10: Cathode-Ray Tube - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            let split: Vec<_> = line.split_whitespace().collect();

            match split.as_slice() {
                ["noop"] => Instruction::Noop,
                ["addx", v] => Instruction::Addx(v.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .try_collect()?
}

const COLS: usize = 40;
const ROWS: usize = 6;

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

struct Fetcher {
    instr: Instruction,
    state: usize,
}

struct Cpu {
    x: i64,
    cycle: i64,
    pc: usize,
    fetcher: Fetcher,
    program: Vec<Instruction>,
}

struct Crt {
    pixels: [bool; COLS * ROWS],
}

impl Fetcher {
    fn needs_fetching(&self) -> bool {
        matches!(
            (self.instr, self.state),
            (Instruction::Noop, 1) | (Instruction::Addx(_), 2)
        )
    }

    fn increment(&mut self) {
        self.state += 1;
    }

    fn fetch(&mut self, i: Instruction) {
        self.instr = i;
        self.state = 0;
    }
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        let fetcher = Fetcher {
            instr: program[0],
            state: 0,
        };

        Self {
            x: 1,
            cycle: 1,
            pc: 0,
            fetcher,
            program,
        }
    }

    fn x(&self) -> i64 {
        self.x
    }

    fn cycle(&self) -> i64 {
        self.cycle
    }

    fn run_cycle(&mut self) {
        self.fetcher.increment();

        self.execute();

        if self.fetcher.needs_fetching() {
            self.pc += 1;
            self.fetcher.fetch(self.program[self.pc]);
        }

        self.cycle += 1;
    }

    fn execute(&mut self) {
        if let (Instruction::Addx(v), 2) = (self.fetcher.instr, self.fetcher.state) {
            self.x += v;
        }
    }
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: [false; COLS * ROWS],
        }
    }

    fn draw(&mut self, cycle: i64, x: i64) {
        let pos_drawn = cycle - 1;
        let normalized = pos_drawn % COLS as i64;
        let sprite = (x - 1)..=(x + 1);

        if sprite.contains(&normalized) {
            let pos_drawn = pos_drawn as usize;
            self.pixels[pos_drawn] = true;
        }
    }

    fn print(&self) -> String {
        self.pixels
            .chunks(COLS)
            .map(|row| {
                row.iter()
                    .map(|&p| if p { '#' } else { ' ' })
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[anyhoo::anyhoo]
fn main() {
    let program = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut cpu = Cpu::new(program);
        let mut crt = Crt::new();
        let n_cycles = COLS * ROWS;

        loop {
            let x = cpu.x();
            let cycle = cpu.cycle();

            crt.draw(cycle, x);

            if cycle == n_cycles as i64 {
                break;
            }

            cpu.run_cycle();
        }

        crt.print()
    });
}
