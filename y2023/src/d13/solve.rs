use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::util::read::read;

pub(crate) fn b(input: &str) -> usize {
    let input = read(input, |l| l.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let maps = input.split(|l| l.is_empty()).collect::<Vec<_>>();

    maps.iter().map(|m| solve_for_delta(m)).sum()
}

pub(crate) fn a(input: &str) -> usize {
    let input = read(input, |l| l.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let maps = input.split(|l| l.is_empty()).collect::<Vec<_>>();

    maps.iter().map(|m| solve_for(m)).sum()
}

fn delta(a: &Vec<char>, b: &Vec<char>) -> usize {
    a.par_iter()
        .zip(b)
        .map(|(x, y)| if x != y { 1 } else { 0 })
        .sum()
}

fn solve_for_delta(field: &[Vec<char>]) -> usize {
    let mut cols: Vec<Vec<char>> = vec![];
    let mut rows: Vec<Vec<char>> = vec![];
    let width = field.first().unwrap().len();
    let mut building_cols: Vec<Vec<char>> = vec![vec![]; width];
    let mut current_row: Vec<char> = vec![];
    for line in field {
        for (i, char) in line.iter().enumerate() {
            current_row.push(*char);
            building_cols[i].push(*char);
        }

        rows.push(line.to_vec());
    }
    for c in building_cols {
        cols.push(c);
    }

    if let Some(res) = mirror_delta(&cols, 1) {
        return res;
    }

    if let Some(res) = mirror_delta(&rows, 100) {
        return res;
    }

    0
}

fn mirror_delta(v: &[Vec<char>], multiplier: usize) -> Option<usize> {
    let def: Vec<char> = vec![];
    for (i, col) in v.iter().enumerate() {
        let next = v.get(i + 1).unwrap_or(&def);
        let mut hit_smudge = false;
        let mut compare = |lhs: &Vec<char>, rhs: &Vec<char>| -> bool {
            let d = delta(lhs, rhs);
            if hit_smudge {
                d == 0
            } else if d == 0 {
                true
            } else if d == 1 {
                hit_smudge = true;
                true
            } else {
                false
            }
        };

        if compare(col, next) {
            let mut j = i - 1;
            let mut k = i + 2;
            let mut is_mirror = true;

            while let Some(r) = v.get(j) {
                if let Some(or) = v.get(k) {
                    if !compare(r, or) {
                        is_mirror = false;
                        break;
                    }
                    k += 1;
                }
                j -= 1;
            }

            if is_mirror && hit_smudge {
                return Some((i + 1) * multiplier);
            }
        }
    }

    None
}

fn solve_for(field: &[Vec<char>]) -> usize {
    let mut cols: Vec<u64> = vec![];
    let mut rows: Vec<u64> = vec![];
    let width = field.first().unwrap().len();
    let mut building_cols: Vec<Vec<char>> = vec![vec![]; width];
    let mut current_row: Vec<char> = vec![];
    for line in field {
        for (i, char) in line.iter().enumerate() {
            current_row.push(*char);
            building_cols[i].push(*char);
        }

        let mut hash = DefaultHasher::new();
        line.hash(&mut hash);
        rows.push(hash.finish());
    }
    for c in building_cols {
        let mut hash = DefaultHasher::new();
        c.hash(&mut hash);
        cols.push(hash.finish());
    }

    if let Some(res) = mirror(&cols, 1) {
        return res;
    }
    if let Some(res) = mirror(&rows, 100) {
        return res;
    }

    0
}

fn mirror(v: &[u64], multiplier: usize) -> Option<usize> {
    for (i, col) in v.iter().enumerate() {
        let next = v.get(i + 1).unwrap_or(&0);

        if col == next {
            let mut j = i - 1;
            let mut k = i + 2;
            let mut is_mirror = true;

            while let Some(r) = v.get(j) {
                if let Some(or) = v.get(k) {
                    if r != or {
                        is_mirror = false;
                        break;
                    }
                    k += 1;
                }
                j -= 1;
            }

            if is_mirror {
                return Some((i + 1) * multiplier);
            }
        }
    }

    None
}
