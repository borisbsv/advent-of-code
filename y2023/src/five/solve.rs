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

type Maps = Vec<Vec<Map>>;

impl Map {
    fn overlaps(&self, seed: &Seeds) -> Seeds {
        max(seed.start, self.src)..min(seed.end, self.src + self.rng)
    }
}

pub trait RangeExt {
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeExt for Range<i64> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
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

    let maps = mappers(lines);

    for m in maps {
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
        let to_apply = maps.iter().find(|&m| {
            let intersection = m.overlaps(&seed);
            !intersection.is_empty()
        });

        let Some(m) = to_apply else {
            output.push(seed);
            continue;
        };

        let Map { src, rng, delta } = m;

        let Seeds {
            start: seed_start,
            end: seed_end,
        } = seed;

        let end = src + rng;

        let intersection = m.overlaps(&seed);

        output.push(Range {
            start: intersection.start + delta,
            end: intersection.end + delta,
        });

        if seed_start < *src {
            input.push(Range {
                start: seed_start,
                end: m.src - 1,
            });
        }

        if seed_end > end {
            input.push(Range {
                start: intersection.end,
                end: seed_end,
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

fn mappers(maps: impl Iterator<Item = String>) -> Maps {
    let mut seed_soil: Vec<Map> = vec![];
    let mut soil_fert: Vec<Map> = vec![];
    let mut fert_water: Vec<Map> = vec![];
    let mut water_light: Vec<Map> = vec![];
    let mut light_temp: Vec<Map> = vec![];
    let mut temp_hum: Vec<Map> = vec![];
    let mut hum_loc: Vec<Map> = vec![];

    let mut mapping = &mut seed_soil;

    for line in maps {
        match line.as_str() {
            "seed-to-soil map:" => {}
            "soil-to-fertilizer map:" => mapping = &mut soil_fert,
            "fertilizer-to-water map:" => mapping = &mut fert_water,
            "water-to-light map:" => mapping = &mut water_light,
            "light-to-temperature map:" => mapping = &mut light_temp,
            "temperature-to-humidity map:" => mapping = &mut temp_hum,
            "humidity-to-location map:" => mapping = &mut hum_loc,
            "" => {}
            _ => {
                let parts = line
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                let src = *parts.get(1).unwrap();
                let dst = *parts.first().unwrap();

                mapping.push(Map {
                    src,
                    rng: *parts.get(2).unwrap(),
                    delta: dst - src,
                })
            }
        }
    }

    vec![
        seed_soil,
        soil_fert,
        fert_water,
        water_light,
        light_temp,
        temp_hum,
        hum_loc,
    ]
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
