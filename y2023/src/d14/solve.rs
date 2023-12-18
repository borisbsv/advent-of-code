use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    let sheet = read(input, |l| l.unwrap().bytes().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut dists: Vec<usize> = vec![];
    let l = sheet.len();

    for (i, row) in sheet.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == b'O' {
                let mut dist = 0;
                for i_p in (0..i).rev() {
                    let o_cell = *sheet.get(i_p).unwrap().get(j).unwrap();
                    if o_cell == b'O' {
                        dist += 1;
                    } else if o_cell == b'#' {
                        dist += i_p + 1;
                        break;
                    }
                }
                dists.push(l - dist);
            }
        }
    }
    dists.iter().sum()
}

#[derive(Debug, Clone, Eq)]
struct Iteration {
    step: usize,
    field: Vec<Vec<char>>,
}

impl Iteration {
    fn north(&mut self) {
        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                if self.field[y][x] != 'O' {
                    continue;
                }
                for y in (1..=y).rev() {
                    match self.field[y - 1][x] {
                        '#' | 'O' => break,
                        _ => {
                            self.field[y - 1][x] = 'O';
                            self.field[y][x] = '.';
                        }
                    }
                }
            }
        }
    }

    fn east(&mut self) {
        for y in (0..self.field.len()).rev() {
            for x in (0..self.field[y].len()).rev() {
                if self.field[y][x] != 'O' {
                    continue;
                }
                for x in x..self.field[y].len() - 1 {
                    match self.field[y][x + 1] {
                        '#' | 'O' => break,
                        _ => {
                            self.field[y][x + 1] = 'O';
                            self.field[y][x] = '.';
                        }
                    }
                }
            }
        }
    }

    fn south(&mut self) {
        for y in (0..self.field.len()).rev() {
            for x in (0..self.field[y].len()).rev() {
                if self.field[y][x] != 'O' {
                    continue;
                }
                for y in y..self.field.len() - 1 {
                    match self.field[y + 1][x] {
                        '#' | 'O' => break,
                        _ => {
                            self.field[y + 1][x] = 'O';
                            self.field[y][x] = '.';
                        }
                    }
                }
            }
        }
    }

    fn west(&mut self) {
        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                if self.field[y][x] != 'O' {
                    continue;
                }
                for x in (1..=x).rev() {
                    match self.field[y][x - 1] {
                        '#' | 'O' => break,
                        _ => {
                            self.field[y][x - 1] = 'O';
                            self.field[y][x] = '.';
                        }
                    }
                }
            }
        }
    }

    fn next(&mut self) {
        self.north();
        self.west();
        self.south();
        self.east();
        self.step += 1;
    }

    fn weights(&self) -> usize {
        self.field
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .map(|c| if *c == 'O' { self.field.len() - y } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Hash for Iteration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field.hash(state);
    }
}

impl PartialEq for Iteration {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

pub(crate) fn b(input: &str) -> usize {
    let sheet = read(input, |l| l.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut step = Iteration {
        step: 0,
        field: sheet,
    };

    let mut visited: HashSet<Iteration> = HashSet::new();

    for i in 0..1_000_000_000 {
        visited.insert(step.clone());
        step.next();

        if let Some(s) = visited.get(&step) {
            let iter_range = i - s.step + 1;
            let remaining = 1_000_000_000 - i;
            let remaining = remaining % iter_range;
            return visited
                .iter()
                .find(|v| v.step == (s.step + remaining - 1))
                .unwrap()
                .weights();
        }
    }
    0
}
