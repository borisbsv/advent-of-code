use std::collections::HashMap;

use crate::util::read::read;

use itertools::Itertools;

pub(crate) fn a(input: &str) -> i32 {
    let mut lists: (Vec<i32>, Vec<i32>) = read(input, |l| {
        l.unwrap()
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
    .unzip();
    lists.0.sort();
    lists.1.sort();

    let mut dist = 0;
    for (i, lhs) in lists.0.iter().enumerate() {
        let rhs = lists.1.get(i).unwrap();
        dist += (lhs - rhs).abs();
    }

    dist
}

pub(crate) fn b(input: &str) -> i32 {
    let lists: (Vec<i32>, Vec<i32>) = read(input, |l| {
        l.unwrap()
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
    .unzip();

    let rhs = lists.1.iter().fold(HashMap::new(), |mut acc, e| {
        let counter = acc.entry(e).or_insert(0);
        *counter += 1;
        acc
    });

    lists.0.iter().fold(0, |mut acc, e| {
        acc += e * rhs.get(e).unwrap_or(&0);
        acc
    })
}
