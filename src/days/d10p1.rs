use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Instruction> {
    let input = BufferedInput::parse_args("Day 10: Cathode-Ray Tube - Part 1")?;

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

#[anyhoo::anyhoo]
fn main() {
    let program = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut cpu = Cpu::new(program);
        let mut result = 0;

        loop {
            let x = cpu.x();
            let cycle = cpu.cycle();

            if cycle % 40 == 20 {
                result += x * cycle;
            }

            if cycle == 220 {
                break;
            }

            cpu.run_cycle();
        }

        result
    });
}
