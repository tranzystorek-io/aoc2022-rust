use std::cmp::Ordering;
use std::io::Read;

use aoc_utils::BufferedInput;

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
fn parse_input() -> Vec<(Entry, Entry)> {
    let mut input = BufferedInput::parse_args("Day 13: Distress Signal - Part 1")?;

    let mut s = String::new();
    input.read_to_string(&mut s)?;

    let split = s.split("\n\n");
    split
        .map(|pair| {
            let (a, b) = pair.split_once('\n').unwrap();

            (parse_packet(a.trim()), parse_packet(b.trim()))
        })
        .collect()
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
