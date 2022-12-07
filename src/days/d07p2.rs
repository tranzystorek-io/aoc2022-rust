use std::collections::HashMap;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<ShellLine> {
    let input = BufferedInput::parse_args("Day 7: No Space Left On Device - Part 2")?;

    input
        .lines()
        .map_ok(|line| {
            let split: Vec<_> = line.split_whitespace().collect();

            match split.as_slice() {
                ["$", "ls"] => ShellLine::Ls,
                ["$", "cd", "/"] => ShellLine::CdRoot,
                ["$", "cd", ".."] => ShellLine::CdUp,
                ["$", "cd", name] => ShellLine::Cd(name.to_string()),
                ["dir", name] => ShellLine::Dir(name.to_string()),
                [size, name] => ShellLine::File(name.to_string(), size.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .try_collect()?
}

type Filesystem = HashMap<String, Vec<Entry>>;
type DiskUsage = HashMap<String, usize>;

#[derive(Debug)]
enum ShellLine {
    Cd(String),
    CdRoot,
    CdUp,
    Ls,
    Dir(String),
    File(String, usize),
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(String, usize),
}

fn realpath(chunks: &[&str]) -> String {
    match chunks {
        ["/"] => "/".to_string(),
        ["/", rest @ ..] => itertools::chain!(&[""], rest).join("/"),
        _ => unreachable!(),
    }
}

fn traverse(terminal: Vec<ShellLine>) -> Filesystem {
    let mut cwd = vec![];
    let mut result = Filesystem::new();

    for line in &terminal {
        match line {
            ShellLine::CdRoot => cwd = vec!["/"],
            ShellLine::CdUp => {
                cwd.pop();
            }
            ShellLine::Cd(name) => cwd.push(name),
            ShellLine::Dir(name) => {
                let current_dir = realpath(&cwd);
                let d = result.entry(current_dir).or_default();

                d.push(Entry::Dir(name.clone()));
            }
            ShellLine::File(name, size) => {
                let current_dir = realpath(&cwd);
                let d = result.entry(current_dir).or_default();

                d.push(Entry::File(name.clone(), *size))
            }
            ShellLine::Ls => (),
        }
    }

    result
}

fn disk_usage(fs: &Filesystem) -> DiskUsage {
    let mut searchspace = vec![vec!["/"]];
    let mut result = DiskUsage::new();

    while let Some(path) = searchspace.pop() {
        let dirname = realpath(&path);

        for entry in &fs[&dirname] {
            match entry {
                Entry::File(_, size) => {
                    for n in 1..=path.len() {
                        let d = realpath(&path[..n]);
                        let entry = result.entry(d).or_default();

                        *entry += size;
                    }
                }
                Entry::Dir(name) => {
                    let next = path.iter().copied().chain([name.as_str()]).collect();
                    searchspace.push(next);
                }
            }
        }
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let lines = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let fs = traverse(lines);
        let du = disk_usage(&fs);

        let used = du["/"];
        let unused = 70_000_000 - used;
        let to_free = 30_000_000 - unused;

        du.values()
            .filter(|&&size| size >= to_free)
            .min()
            .copied()
            .unwrap()
    });
}
