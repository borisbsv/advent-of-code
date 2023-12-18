use std::collections::VecDeque;

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    read(input, |l| l.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(hash)
        .sum()
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0usize, |acc, b| (acc + b as usize) * 17 % 256)
}

pub(crate) fn b(input: &str) -> usize {
    let mut boxes: [VecDeque<(&str, usize)>; 256] = std::array::from_fn(|_| VecDeque::new());
    let binding = read(input, |l| l.unwrap()).next().unwrap();
    let _ = binding
        .split(',')
        .map(|i| {
            if i.ends_with('-') {
                let label = i.strip_suffix('-').unwrap();
                let h = hash(label);
                boxes[h].retain(|x| x.0 != label);
                return;
            }

            let mut parts = i.split('=');
            let label = parts.next().unwrap();
            let h = hash(label);
            let focal_len = parts.next().unwrap().parse::<usize>().unwrap();

            if let Some(index) = boxes[h].iter().position(|(l, _)| l == &label) {
                boxes[h][index] = (label, focal_len);
            } else {
                boxes[h].push_front((label, focal_len));
            }
        })
        .collect::<Vec<_>>();

    boxes
        .iter()
        .enumerate()
        .map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(|(i, (_, focal_len))| (1 + bi) * (b.len() - i) * focal_len)
                .sum::<usize>()
        })
        .sum()
}
