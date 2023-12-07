use std::collections::VecDeque;

use crate::util::read::read;

struct Game {
    mine: Vec<i32>,
    winning: Vec<i32>,
    copies: i32,
}

pub(crate) fn a(input: &str) -> i32 {
    read(input, |l| {
        let binding = l.unwrap();
        binding
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .map(|l| {
                l.split(' ')
                    .filter_map(|l| l.trim().parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>()
    })
    .map(|card| {
        let winning = card.first().unwrap();
        let mine = card.get(1).unwrap();

        let mut total = 0;
        for c in mine.iter() {
            if !winning.contains(c) {
                continue;
            }
            if total == 0 {
                total = 1;
                continue;
            }

            total *= 2;
        }
        total
    })
    .sum::<i32>()
}

pub(crate) fn b(input: &str) -> i32 {
    let mut list = read(input, |l| {
        let binding1 = l.unwrap();
        let binding = binding1.split(": ").nth(1).unwrap();
        let mut parts = binding.split(" | ");
        let winning = parts
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|l| l.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>();
        let mine = parts
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|l| l.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>();

        Game {
            mine,
            winning,
            copies: 1,
        }
    })
    .collect::<VecDeque<_>>();

    let mut count: i32 = 0;
    while !list.is_empty() {
        let card = list.pop_front().unwrap();
        count += card.copies;

        let mut matches = 0;
        for c in card.mine.iter() {
            if card.winning.contains(c) {
                matches += 1;
            }
        }
        for m in 0..matches {
            list.get_mut(m).unwrap().copies += card.copies;
        }
    }
    count
}
