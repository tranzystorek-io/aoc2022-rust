use std::io::Read;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_operation(line: &str) -> Operation {
    let expr = line.strip_prefix("  Operation: new = ").unwrap();

    match expr {
        "old * old" => Operation::Square,
        s if s.starts_with("old *") => {
            let v = s.strip_prefix("old * ").unwrap().parse().unwrap();
            Operation::Mul(v)
        }
        s if s.starts_with("old +") => {
            let v = s.strip_prefix("old + ").unwrap().parse().unwrap();
            Operation::Add(v)
        }
        _ => unreachable!(),
    }
}

fn parse_monkey(desc: &str) -> Monkey {
    let (_, items, op, test, if_true, if_false) = desc.lines().collect_tuple().unwrap();

    let items = items
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();

    let op = parse_operation(op);

    let test = test
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();

    let if_true = if_true
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    let if_false = if_false
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    Monkey {
        items,
        op,
        test,
        if_true,
        if_false,
    }
}

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Monkey> {
    let mut input = BufferedInput::parse_args("Day 11: Monkey in the Middle - Part 1")?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let split = s.split("\n\n");
    split.map(parse_monkey).collect()
}

const N_ROUNDS: usize = 20;
const BOREDOM_FACTOR: u64 = 3;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn do_operation(worry: u64, op: Operation) -> u64 {
    match op {
        Operation::Add(v) => worry + v,
        Operation::Mul(v) => worry * v,
        Operation::Square => worry * worry,
    }
}

fn play_single_round(monkeys: &mut [Monkey]) -> Vec<usize> {
    let mut result = vec![0; monkeys.len()];

    for i in 0..monkeys.len() {
        let current = &mut monkeys[i];
        let to_inspect = current.items.split_off(0);
        let Monkey {
            op,
            test,
            if_true,
            if_false,
            ..
        } = *current;

        result[i] += to_inspect.len();

        for item in to_inspect {
            let inspected = do_operation(item, op);
            let bored = inspected / BOREDOM_FACTOR;

            let test = bored % test == 0;

            let target = if test {
                &mut monkeys[if_true]
            } else {
                &mut monkeys[if_false]
            };

            target.items.push(bored);
        }
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let mut monkeys = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut business = (0..N_ROUNDS)
            .map(|_| play_single_round(&mut monkeys))
            .reduce(|mut acc, current| {
                for i in 0..acc.len() {
                    acc[i] += current[i];
                }
                acc
            })
            .unwrap();

        business.sort_unstable();

        let [.., a, b] = business[..] else {
            panic!();
        };

        a * b
    });
}
