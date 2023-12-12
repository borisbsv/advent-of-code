use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::read::read;

type Matrix = HashMap<Node, [Pole; 2]>;

trait Mtx {
    fn to_loop(&self, start: Node) -> HashSet<Node>;
    fn from_vec(other: &Vec<Vec<u8>>) -> (Self, Node)
    where
        Self: std::marker::Sized;
}

impl Mtx for Matrix {
    fn to_loop(&self, start: Node) -> HashSet<Node> {
        let mut visited = HashSet::new();
        let mut pipes = VecDeque::new();

        pipes.push_back(start);

        while let Some(pipe) = pipes.pop_front() {
            if visited.contains(&pipe) {
                continue;
            }

            visited.insert(pipe);

            for n in self[&pipe].map(|pole| pipe.neighbor(pole)) {
                if self.contains_key(&n) {
                    pipes.push_back(n);
                }
            }
        }
        visited
    }

    fn from_vec(other: &Vec<Vec<u8>>) -> (Self, Node) {
        let mut matrix = Matrix::new();
        let mut current = Node::default();
        for (i, row) in other.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let node = (i, j);
                if cell == b'S' {
                    current = node;
                    let pipe = s_to_pipe(&other, (i, j));
                    matrix.insert(node, Pole::poles(pipe));
                } else if cell != b'.' {
                    matrix.insert(node, Pole::poles(cell));
                }
            }
        }
        (matrix, current)
    }
}

type Node = (usize, usize);

trait NodeT {
    fn neighbor(&self, pole: Pole) -> Self;
}

impl NodeT for Node {
    fn neighbor(&self, pole: Pole) -> Self {
        match pole {
            Pole::North => (self.0 - 1, self.1),
            Pole::South => (self.0 + 1, self.1),
            Pole::East => (self.0, self.1 + 1),
            Pole::West => (self.0, self.1 - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pole {
    North,
    South,
    East,
    West,
}

impl Pole {
    fn poles(c: u8) -> [Pole; 2] {
        match c {
            b'|' => [Pole::North, Pole::South],
            b'-' => [Pole::East, Pole::West],
            b'L' => [Pole::North, Pole::East],
            b'J' => [Pole::North, Pole::West],
            b'7' => [Pole::South, Pole::West],
            b'F' => [Pole::South, Pole::East],
            _ => {
                unreachable!("{}", c as char)
            }
        }
    }
}

pub(crate) fn a(input: &str) -> usize {
    let input = read(input, |l| l.unwrap().bytes().collect::<Vec<_>>()).collect::<Vec<_>>();

    let (matrix, start) = Matrix::from_vec(&input);

    matrix.to_loop(start).len() / 2
}

pub(crate) fn b(input: &str) -> usize {
    let input = read(input, |l| l.unwrap().bytes().collect::<Vec<_>>()).collect::<Vec<_>>();

    let (matrix, start) = Matrix::from_vec(&input);
    let looped = matrix.to_loop(start);
    let mut count = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let node = (i, j);
            if looped.contains(&node) {
                continue;
            }

            // Go west (arbitrarily chosen)
            let wests = (0..node.1)
                .filter_map(|n| {
                    let visit = &(node.0, n);
                    if looped.contains(visit) {
                        matrix.get(visit)
                    } else {
                        None
                    }
                })
                // Do we have a bend in the pipe - this also works with South
                // when going west/east - we simply need to check for 90° from direction.
                .filter(|n| n.contains(&Pole::North))
                .count();

            // If we crossed an odd number of pipes
            if wests & 1 == 1 {
                count += 1;
            }
        }
    }
    count
}

fn s_to_pipe(input: &[Vec<u8>], coord: Node) -> u8 {
    const NORTH: [u8; 3] = [b'|', b'F', b'7'];
    const EAST: [u8; 3] = [b'-', b'J', b'7'];
    const SOUTH: [u8; 3] = [b'|', b'J', b'L'];
    const WEST: [u8; 3] = [b'-', b'L', b'F'];
    let n = coord.neighbor(Pole::North);
    let e = coord.neighbor(Pole::East);
    let s = coord.neighbor(Pole::South);
    let w = coord.neighbor(Pole::West);
    let has_n = NORTH.contains(input.get(n.0).unwrap().get(n.1).unwrap());
    let has_e = EAST.contains(input.get(e.0).unwrap().get(e.1).unwrap());
    let has_s = SOUTH.contains(input.get(s.0).unwrap().get(s.1).unwrap());
    let has_w = WEST.contains(input.get(w.0).unwrap().get(w.1).unwrap());
    if has_n && has_s {
        b'|'
    } else if has_e && has_w {
        b'-'
    } else if has_n && has_e {
        b'L'
    } else if has_n && has_w {
        b'J'
    } else if has_s && has_w {
        b'7'
    } else if has_s && has_e {
        b'F'
    } else {
        unreachable!("{:?}", coord);
    }
}
