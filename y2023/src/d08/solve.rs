use std::{
    cmp::{max, min},
    collections::HashMap,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::util::read::read;

type Nodes = HashMap<[u8; 3], ([u8; 3], [u8; 3])>;

pub(crate) fn b(input: &str) -> usize {
    let mut lines = read(input, |l| l.unwrap());

    let instructions = lines.next().unwrap();
    lines.next();

    let mut starts: Vec<[u8; 3]> = vec![];

    let nodes: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = lines
        .map(|line| {
            let mut parts = line.split(" = ");
            let from: [u8; 3] = parts.next().unwrap().as_bytes().try_into().unwrap();
            let mut to = parts.next().unwrap().split(", ");
            let l: [u8; 3] = to.next().unwrap()[1..]
                .to_owned()
                .as_bytes()
                .try_into()
                .unwrap();
            let mut tmp = to.next().unwrap().chars();
            tmp.next_back();
            let r: [u8; 3] = tmp.as_str().as_bytes().try_into().unwrap();
            if from[2..3] == [b'A'] {
                starts.push(from);
            }
            (from.to_owned(), (l.to_owned(), r.to_owned()))
        })
        .collect();

    starts
        .par_iter()
        .map(|start| {
            solve(
                instructions.chars().cycle(),
                &nodes,
                *start,
                |node: &[u8; 3]| node[2..3] == [b'Z'],
            )
        })
        .reduce(|| 1, lcm)
}

pub(crate) fn a(input: &str) -> usize {
    let mut lines = read(input, |l| l.unwrap());

    let instructions = lines.next().unwrap();
    lines.next();

    let nodes: Nodes = lines
        .map(|line| {
            let mut parts = line.split(" = ");
            let from: [u8; 3] = parts.next().unwrap().as_bytes().try_into().unwrap();
            let mut to = parts.next().unwrap().split(", ");
            let l: [u8; 3] = to.next().unwrap()[1..]
                .to_owned()
                .as_bytes()
                .try_into()
                .unwrap();
            let mut tmp = to.next().unwrap().chars();
            tmp.next_back();
            let r: [u8; 3] = tmp.as_str().as_bytes().try_into().unwrap();
            (from.to_owned(), (l.to_owned(), r.to_owned()))
        })
        .collect();

    const EXIT: [u8; 3] = [b'Z', b'Z', b'Z'];
    solve(
        instructions.chars().cycle(),
        &nodes,
        [b'A', b'A', b'A'],
        |node| *node == EXIT,
    )
}

fn solve(
    directions: impl Iterator<Item = char>,
    nodes: &Nodes,
    start: [u8; 3],
    end: fn(&[u8; 3]) -> bool,
) -> usize {
    let mut cursor: [u8; 3] = start;
    let mut steps = 0;
    let mut ring = directions;

    while !end(&cursor) {
        let c = ring.next().unwrap();
        steps += 1;
        match c {
            'L' => {
                cursor = nodes.get(&cursor).unwrap().0.to_owned();
            }
            'R' => {
                cursor = nodes.get(&cursor).unwrap().1.to_owned();
            }
            _ => {}
        }
    }

    steps
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
