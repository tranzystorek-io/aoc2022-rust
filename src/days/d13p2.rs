use std::cmp::Ordering;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

fn parse_packet(line: &str) -> Entry {
    let mut stack: Vec<Vec<Entry>> = vec![];
    let mut int_buffer: Option<u16> = None;
    let mut bracket_before = false;

    for (i, byte) in line.bytes().enumerate() {
        match byte {
            b'[' => {
                stack.push(vec![]);
            }
            b']' => {
                if let Some(v) = int_buffer.take() {
                    let outer = stack.last_mut().unwrap();
                    outer.push(Entry::Int(v));
                }

                if i == line.len() - 1 {
                    break;
                }

                let items = stack.pop().unwrap();

                let outer = stack.last_mut().unwrap();
                outer.push(Entry::List(items));
            }
            b',' => {
                if let Some(v) = int_buffer.take() {
                    let outer = stack.last_mut().unwrap();
                    outer.push(Entry::Int(v));
                } else if !bracket_before {
                    let items = stack.pop().unwrap();
                    let outer = stack.last_mut().unwrap();
                    outer.push(Entry::List(items));
                }
            }
            digit @ b'0'..=b'9' => {
                let v = match &mut int_buffer {
                    Some(v) => v,
                    None => {
                        int_buffer = Some(0);
                        int_buffer.as_mut().unwrap()
                    }
                };

                let d = (digit - b'0') as u16;
                *v = *v * 10 + d;
            }
            _ => unreachable!(),
        }

        bracket_before = byte == b']';
    }

    let items = stack.pop().unwrap();

    Entry::List(items)
}

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Entry> {
    let input = BufferedInput::parse_args("Day 13: Distress Signal - Part 2")?;

    input
        .lines()
        .filter_ok(|line| !line.is_empty())
        .map_ok(|line| parse_packet(line.trim()))
        .try_collect()?
}

#[derive(Clone, Debug)]
enum Entry {
    List(Vec<Entry>),
    Int(u16),
}

fn compare(left: &Entry, right: &Entry) -> Ordering {
    match (left, right) {
        (Entry::List(vl), Entry::List(vr)) => {
            for (it_l, it_r) in std::iter::zip(vl, vr) {
                let order = compare(it_l, it_r);
                if !order.is_eq() {
                    return order;
                }
            }

            Ord::cmp(&vl.len(), &vr.len())
        }
        (Entry::Int(il), Entry::Int(ir)) => Ord::cmp(&il, &ir),
        (Entry::List(vl), Entry::Int(ir)) => {
            let list = Entry::List(vl.clone());
            let tmp_r = Entry::List(vec![Entry::Int(*ir)]);

            compare(&list, &tmp_r)
        }
        (Entry::Int(il), Entry::List(vr)) => {
            let tmp_l = Entry::List(vec![Entry::Int(*il)]);
            let list = Entry::List(vr.clone());

            compare(&tmp_l, &list)
        }
    }
}

fn divider(value: u16) -> Entry {
    Entry::List(vec![Entry::List(vec![Entry::Int(value)])])
}

#[anyhoo::anyhoo]
fn main() {
    let packets = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let divider_a = divider(2);
        let divider_b = divider(6);

        itertools::chain!(
            packets.into_iter().map(|packet| (false, packet)),
            [(true, divider_a), (true, divider_b)]
        )
        .sorted_by(|(_, a), (_, b)| compare(a, b))
        .enumerate()
        .filter_map(|(i, (is_divider, _))| is_divider.then_some(i + 1))
        .product::<usize>()
    });
}
