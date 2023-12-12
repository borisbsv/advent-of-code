use std::{cmp::Ordering, collections::HashMap, iter::zip};

use crate::util::read::read;

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    KindFive,
    KindFour,
    FullHouse,
    KindThree,
    PairTwo,
    Pair,
    High,
    None,
}

impl Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Greater
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: String,
    bid: usize,
    typ: Hand,
}

impl Game {
    fn new(hand: String, bid: usize) -> Self {
        Game {
            typ: Game::hand_type(&hand),
            hand,
            bid,
        }
    }

    fn hand_type(hand: &str) -> Hand {
        let mut cards: HashMap<char, usize> = HashMap::new();
        let mut js: usize = 0;
        for c in hand.chars() {
            if c == 'J' {
                js += 1;
                continue;
            }
            *cards.entry(c).or_default() += 1;
        }

        let mut vals: Vec<usize> = cards.values().copied().collect();
        vals.sort_by(|a, b| b.cmp(a));

        if js == 5 {
            return Hand::KindFive;
        }
        vals[0] += js;

        match vals[..] {
            [1, 1, 1, 1, 1] => Hand::High,
            [2, 1, 1, 1] => Hand::Pair,
            [3, 1, 1] => Hand::KindThree,
            [2, 2, 1] => Hand::PairTwo,
            [3, 2] => Hand::FullHouse,
            [4, 1] => Hand::KindFour,
            [5] => Hand::KindFive,
            _ => Hand::None,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.typ != other.typ {
            return self.typ.cmp(&other.typ);
        }

        let iter = zip(self.hand.chars(), other.hand.chars());

        for (c1, c2) in iter {
            match value(c1).cmp(&value(c2)) {
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
            }
        }

        Ordering::Equal
    }
}

fn value(card: char) -> Option<usize> {
    match card {
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        '9' => Some(8),
        'T' => Some(9),
        'J' => Some(0),
        'Q' => Some(11),
        'K' => Some(12),
        'A' => Some(13),
        _ => None,
    }
}

pub(crate) fn a(input: &str) -> usize {
    let mut hands = read(input, |l| {
        let binding = l.unwrap();
        let mut t = binding.split(' ');
        Game::new(
            t.next().unwrap().to_owned(),
            t.next().unwrap().parse::<usize>().unwrap(),
        )
    })
    .collect::<Vec<Game>>();
    hands.sort_by(|a, b| a.cmp(b));

    // hands.iter().for_each(|h| println!("{h:?}"));

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (i + 1) * h.bid)
}
