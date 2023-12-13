use std::{cmp, collections::HashMap};

use crate::util::read::read;

#[derive(Hash, Clone, PartialEq, Debug, Eq)]
struct Record {
    springs: Vec<char>,
    groups: Vec<usize>,
}

pub(crate) fn a(input: &str) -> usize {
    let records = read(input, |l| {
        let binding = l.unwrap();
        let mut line = binding.split_whitespace();
        Record {
            springs: line.next().unwrap().chars().collect(),
            groups: line
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    });

    let mut memo: HashMap<Record, usize> = HashMap::new();

    records.map(|r| arrangements(&mut memo, &r)).sum()
}

pub(crate) fn b(input: &str) -> usize {
    let records = read(input, |l| {
        let binding = l.unwrap();
        let mut line = binding.split_whitespace();
        let ss = line.next().unwrap().chars().collect::<Vec<_>>();
        let springs = ss
            .iter()
            .cloned()
            .chain(['?'].iter().cloned())
            .cycle()
            .take(ss.len() * 5 + 4)
            .collect();
        let gs = line
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let groups = gs.iter().cloned().cycle().take(gs.len() * 5).collect();
        Record { springs, groups }
    });

    let mut memo: HashMap<Record, usize> = HashMap::new();

    records.map(|r| arrangements(&mut memo, &r)).sum()
}

fn arrangements(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    if let Some(&v) = memo.get(record) {
        return v;
    }

    if record.groups.is_empty() {
        let increment = match !record.springs.contains(&'#') {
            true => 1,
            false => 0,
        };
        memo.insert(record.clone(), increment);
        return increment;
    }

    // not enough spaces left to fill
    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    if record.springs[0] == '.' {
        let next = arrangements(
            memo,
            &Record {
                springs: record.springs[1..].to_vec(),
                groups: record.groups.clone(),
            },
        );
        memo.insert(record.clone(), next);
        return next;
    }

    // actual arrangement validity check
    let mut arrs = 0;
    let start = record.groups[0];
    let end = cmp::min(start + 1, record.springs.len());
    let all_non_op = record.springs[0..start].iter().all(|&s| s != '.');
    let can_recurse = (start < record.springs.len() && record.springs[start] != '#')
        || record.springs.len() <= start;

    if all_non_op && can_recurse {
        arrs = arrangements(
            memo,
            &Record {
                springs: record.springs[end..].to_vec(),
                groups: record.groups[1..].to_vec(),
            },
        );
    }

    if *record.springs.first().unwrap() == '?' {
        arrs += arrangements(
            memo,
            &Record {
                springs: record.springs[1..].to_vec(),
                groups: record.groups.clone(),
            },
        );
    }

    memo.insert(record.clone(), arrs);
    arrs
}
