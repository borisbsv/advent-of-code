use std::{cmp, collections::HashSet};

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    distance(input, 1)
}

pub(crate) fn b(input: &str) -> usize {
    distance(input, 999999)
}

fn distance(input: &str, factor: usize) -> usize {
    let lines = read(input, |l| l.unwrap());
    let mut length = 0;
    let mut width = 0;
    let mut cols_w_gs: HashSet<usize> = HashSet::new();
    let mut rows_w_gs: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row, line) in lines.enumerate() {
        for (col, cell) in line.chars().enumerate() {
            if cell == '#' {
                cols_w_gs.insert(col);
                rows_w_gs.insert(row);
                galaxies.push((row, col));
            }
        }
        length = row;
        width = line.len();
    }

    let mut empty_rows: HashSet<usize> = HashSet::new();
    for i in 0..length {
        if !rows_w_gs.contains(&i) {
            empty_rows.insert(i);
        }
    }
    let mut empty_cols: HashSet<usize> = HashSet::new();
    for i in 0..width {
        if !cols_w_gs.contains(&i) {
            empty_cols.insert(i);
        }
    }
    let mut length: usize = 0;
    for (i, g) in galaxies.iter().enumerate() {
        for og in galaxies.iter().skip(i + 1) {
            let to_add = empty_cols
                .iter()
                .filter(|&&c| {
                    let min = cmp::min(g.1, og.1);
                    let max = cmp::max(g.1, og.1);
                    min < c && c < max
                })
                .count();
            let col_delta = g.1.abs_diff(og.1);
            let col_increase = col_delta + to_add * factor;

            let to_add = empty_rows
                .iter()
                .filter(|&&c| {
                    let min = cmp::min(g.0, og.0);
                    let max = cmp::max(g.0, og.0);
                    min < c && c < max
                })
                .count();
            let row_delta = g.0.abs_diff(og.0);
            let row_increase = row_delta + to_add * factor;

            let increase = col_increase + row_increase;
            length += increase;
        }
    }

    length
}
