use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

use crate::util::read::read;

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    FMirror,
    BMirror,
    HSplitter,
    VSplitter,
    Wall,
}

type Coord = (usize, usize);

#[derive(Default, Eq, Hash, PartialEq, Clone, Debug)]
struct Move {
    dir: Dir,
    coord: Coord,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Empty => ".".to_string(),
            Cell::FMirror => "/".to_string(),
            Cell::BMirror => "\\".to_string(),
            Cell::HSplitter => "-".to_string(),
            Cell::VSplitter => "|".to_string(),
            Cell::Wall => "#".to_string(),
        }
    }
}

impl Cell {
    fn next_dirs(&self, from: Dir) -> Vec<Dir> {
        let binding = Vec::from([from.clone()]);
        match self {
            Cell::Empty => binding,
            Cell::FMirror => match from {
                Dir::Right => [Dir::Up].to_vec(),
                Dir::Down => [Dir::Left].to_vec(),
                Dir::Left => [Dir::Down].to_vec(),
                Dir::Up => [Dir::Right].to_vec(),
            },
            Cell::BMirror => match from {
                Dir::Right => [Dir::Down].to_vec(),
                Dir::Down => [Dir::Right].to_vec(),
                Dir::Left => [Dir::Up].to_vec(),
                Dir::Up => [Dir::Left].to_vec(),
            },
            Cell::HSplitter => match from {
                Dir::Right | Dir::Left => binding,
                Dir::Down | Dir::Up => [Dir::Right, Dir::Left].to_vec(),
            },
            Cell::VSplitter => match from {
                Dir::Right | Dir::Left => [Dir::Up, Dir::Down].to_vec(),
                Dir::Down | Dir::Up => binding,
            },
            Cell::Wall => unreachable!("we shouldn't be trying to go from wall"),
        }
    }
}

impl From<char> for Cell {
    fn from(s: char) -> Self {
        match s {
            '-' => Cell::HSplitter,
            '|' => Cell::VSplitter,
            '/' => Cell::FMirror,
            '\\' => Cell::BMirror,
            '#' => Cell::Wall,
            _ => Cell::Empty,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Hash, Eq)]
enum Dir {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
        }
    }
}

pub(crate) fn a(input: &str) -> usize {
    let m = read(input, |l| {
        l.unwrap().chars().map(Cell::from).collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    solve(&m, Default::default())
}

pub(crate) fn b(input: &str) -> usize {
    let m = read(input, |l| {
        l.unwrap().chars().map(Cell::from).collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let mut solution: usize = 0;

    for i in 0..m.first().unwrap().len() {
        solution = max(
            solution,
            solve(
                &m,
                Move {
                    coord: (0, i),
                    dir: Dir::Down,
                },
            ),
        );
    }

    let last = m.len() - 1;
    for i in 0..m.first().unwrap().len() {
        solution = max(
            solution,
            solve(
                &m,
                Move {
                    coord: (last, i),
                    dir: Dir::Up,
                },
            ),
        );
    }

    for i in 0..m.len() {
        solution = max(
            solution,
            solve(
                &m,
                Move {
                    coord: (i, 0),
                    dir: Dir::Right,
                },
            ),
        );
    }

    let last = m.first().unwrap().len() - 1;
    for i in 0..m.len() {
        solution = max(
            solution,
            solve(
                &m,
                Move {
                    coord: (i, last),
                    dir: Dir::Left,
                },
            ),
        );
    }

    solution
}

fn solve(mtx: &[Vec<Cell>], start: Move) -> usize {
    let mut energized: HashSet<Move> = HashSet::new();

    let mut queue: VecDeque<Move> = VecDeque::from([start]);
    while let Some(mov) = queue.pop_front() {
        if energized.contains(&mov) {
            continue;
        }
        energized.insert(mov.clone());
        for next in step(mtx, mov) {
            queue.push_back(next);
        }
    }

    HashSet::<Coord>::from_iter(energized.iter().map(|i| i.coord)).len()
}

fn step(mtx: &[Vec<Cell>], mov: Move) -> Vec<Move> {
    let Move { coord, dir } = mov;
    if let Some(row) = mtx.get(coord.0) {
        if let Some(cell) = row.get(coord.1) {
            let dirs = cell.next_dirs(dir);
            let mut nexts: Vec<Move> = vec![];
            for dir in dirs {
                let delta = dir.delta();
                let x = coord.0 as isize + delta.0;
                let y = coord.1 as isize + delta.1;
                if x < 0 || x >= mtx.len() as isize {
                    continue;
                }
                if y < 0 || y >= mtx.first().unwrap().len() as isize {
                    continue;
                }

                nexts.push(Move {
                    coord: (x as usize, y as usize),
                    dir,
                });
            }
            return nexts;
        }
    }
    Default::default()
}
