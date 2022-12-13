use std::cmp::Ordering;
use std::io::Read;

use aoc_utils::BufferedInput;

fn parse_packet(line: &str) -> Packet {
    let mut nesting = 0;
    let mut bracket_before = false;
    let mut int_buffer = String::new();
    let mut result = vec![];

    let line = line.trim();

    for c in line.chars() {
        match c {
            '[' => {
                nesting += 1;
                bracket_before = true;
            }
            ']' => {
                if !int_buffer.is_empty() {
                    let v = int_buffer.parse().unwrap();
                    result.push((nesting, Item::Int(v)));
                    int_buffer.clear();
                }

                nesting -= 1;

                if bracket_before {
                    result.push((nesting, Item::EmptyList))
                }

                bracket_before = false;
            }
            ',' => {
                if !int_buffer.is_empty() {
                    let v = int_buffer.parse().unwrap();
                    result.push((nesting, Item::Int(v)));
                    int_buffer.clear();
                }

                bracket_before = false;
            }
            digit @ '0'..='9' => {
                int_buffer.push(digit);
                bracket_before = false;
            }
            err => unreachable!("found unexpected {:?}", err),
        }
    }

    result
}

#[anyhoo::anyhoo]
fn parse_input() -> Vec<(Packet, Packet)> {
    let mut input = BufferedInput::parse_args("Day 12: Hill Climbing Algorithm - Part 1")?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let split = s.split("\n\n");
    split
        .map(|pair| {
            let (a, b) = pair.split_once('\n').unwrap();

            (parse_packet(a), parse_packet(b))
        })
        .collect()
}

type Entry = (usize, Item);
type Packet = Vec<Entry>;

#[derive(Clone, Copy, Debug)]
enum Item {
    Int(u16),
    EmptyList,
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    for (&(nest_l, item_l), &(nest_r, item_r)) in std::iter::zip(left, right) {
        match (item_l, item_r) {
            (Item::Int(int_l), Item::Int(int_r)) if usize::abs_diff(nest_l, nest_r) <= 1 => {
                let order = Ord::cmp(&int_l, &int_r);
                if !order.is_eq() {
                    return order;
                }
            }
            // probably incorrect
            (Item::EmptyList, Item::Int(_)) if usize::abs_diff(nest_l, nest_r) <= 1 => return Ordering::Less,
            (Item::Int(_), Item::EmptyList) if usize::abs_diff(nest_l, nest_r) <= 1 => return Ordering::Greater,
            _ => (),
        }

        let order = Ord::cmp(&nest_l, &nest_r);
        if !order.is_eq() {
            return order;
        }
    }

    Ord::cmp(&left.len(), &right.len())
}

#[anyhoo::anyhoo]
fn main() {
    let pairs = parse_input()?;

    aoc_utils::measure_and_print(|| {
        pairs
            .into_iter()
            .enumerate()
            .filter_map(|(i, (l, r))| compare(&l, &r).is_le().then_some(i + 1))
            .sum::<usize>()
    });
}
