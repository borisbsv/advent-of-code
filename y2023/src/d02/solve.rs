use std::collections::HashMap;

use crate::util::read::read;

struct Game {
    id: i32,
    r: i32,
    g: i32,
    b: i32,
}

const R: i32 = 12;
const B: i32 = 14;
const G: i32 = 13;

pub(crate) fn a(input: &str) -> i32 {
    maxes(input)
        .map(|g| {
            if g.r <= R && g.b <= B && g.g <= G {
                return g.id;
            }
            0
        })
        .sum()
}

#[allow(dead_code)]
pub(crate) fn b(input: &str) -> i32 {
    maxes(input).map(|g: Game| g.r * g.g * g.b).sum()
}

fn maxes(input: &str) -> impl Iterator<Item = Game> {
    let t = read(input, |l| {
        let binding = l.unwrap();
        let parts: Vec<_> = binding.split(": ").collect();
        let id = parts[0][5..].parse::<i32>().unwrap();
        let mut maxes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        let shows = parts[1].split("; ");
        for s in shows {
            s.split(", ").for_each(|p| {
                let g: Vec<_> = p.split(' ').collect();
                let num = g.first().unwrap().parse::<i32>().unwrap();
                let binding = g.get(1).unwrap();
                if maxes[binding] < num {
                    *maxes.get_mut(binding).unwrap() = num;
                }
            });
        }

        Game {
            id,
            r: maxes["red"],
            g: maxes["green"],
            b: maxes["blue"],
        }
    });
    t
}
