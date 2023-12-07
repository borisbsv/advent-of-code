use std::{
    cmp::{max, min},
    ops::Range,
};

use crate::util::read::read;

type Seeds = Range<i64>;

#[derive(Default, Debug)]
struct Map {
    src: i64,
    rng: i64,
    delta: i64,
}

impl Map {
    fn overlaps(&self, seed: &Seeds) -> Seeds {
        max(seed.start, self.src)..min(seed.end, self.src + self.rng)
    }
}

type Maps = Vec<Vec<Map>>;

fn mappers(maps: impl Iterator<Item = String>) -> Maps {
    let mut parsed: Maps = vec![];

    for line in maps {
        match line.as_str() {
            "seed-to-soil map:" => parsed.push(vec![]),
            "soil-to-fertilizer map:" => parsed.push(vec![]),
            "fertilizer-to-water map:" => parsed.push(vec![]),
            "water-to-light map:" => parsed.push(vec![]),
            "light-to-temperature map:" => parsed.push(vec![]),
            "temperature-to-humidity map:" => parsed.push(vec![]),
            "humidity-to-location map:" => parsed.push(vec![]),
            "" => {}
            _ => {
                let parts = line
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                let src = *parts.get(1).unwrap();
                let dst = *parts.first().unwrap();

                parsed.last_mut().unwrap().push(Map {
                    src,
                    rng: *parts.get(2).unwrap(),
                    delta: dst - src,
                })
            }
        }
    }

    parsed
}

pub(crate) fn b(input: &str) -> i64 {
    let mut lines = read(input, |l| l.unwrap());
    let binding = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seeds = binding
        .chunks(2)
        .map(|r| {
            let s = *r.first().unwrap();
            let e = *r.first().unwrap() + *r.get(1).unwrap();
            s..e
        })
        .collect::<Vec<Seeds>>();

    for m in mappers(lines) {
        seeds = seeds
            .into_iter()
            .flat_map(|seed| map(seed, &m))
            .collect::<Vec<Seeds>>();
    }

    seeds.iter().map(|s| s.start).min().unwrap()
}

fn map(range: Seeds, maps: &[Map]) -> Vec<Seeds> {
    let mut input = vec![range];
    let mut output = vec![];

    while let Some(seed) = input.pop() {
        let to_apply = maps.iter().find(|&m| !m.overlaps(&seed).is_empty());

        let Some(m) = to_apply else {
            output.push(seed);
            continue;
        };

        let Map { src, rng, delta } = m;
        let Seeds {
            start: seeds_start,
            end: seeds_end,
        } = seed;
        let end = src + rng;
        let intersection = m.overlaps(&seed);

        output.push(Range {
            start: intersection.start + delta,
            end: intersection.end + delta,
        });

        if seeds_start < *src {
            input.push(Range {
                start: seeds_start,
                end: m.src - 1,
            });
        }

        if seeds_end > end {
            input.push(Range {
                start: intersection.end,
                end: seeds_end,
            });
        }
    }

    output
}

pub(crate) fn a(input: &str) -> i64 {
    let mut lines = read(input, |l| l.unwrap());
    let input = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps = mappers(lines);

    min_location(input.into_iter(), &maps)
}

fn min_location(input: impl Iterator<Item = i64>, maps: &Maps) -> i64 {
    let mut min: i64 = i64::MAX;
    for p in input {
        let mut cursor = p;
        for map in maps {
            cursor = delta(cursor, map);
        }
        if min > cursor {
            min = cursor
        }
    }
    min
}

fn delta(cursor: i64, map: &Vec<Map>) -> i64 {
    for m in map.as_slice() {
        if m.src <= cursor && cursor <= m.src + m.rng {
            return cursor + m.delta;
        }
    }
    cursor
}
