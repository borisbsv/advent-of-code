use crate::util::read::read;

pub(crate) fn a(input: &str) -> i64 {
    let mut lines = read(input, |l| l.unwrap());
    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|p| p.parse::<i64>().ok())
        .collect();
    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|p| p.parse::<i64>().ok())
        .collect();

    victories(times, distances)
}

pub(crate) fn b(input: &str) -> i64 {
    let mut lines = read(input, |l| l.unwrap());
    let time: f64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .collect::<String>()
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let distance: f64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .collect::<String>()
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let x1 = (time / 2.0) - f64::sqrt((time / 2.0).powf(2.0) - distance).ceil();
    let x2 = (time / 2.0) + f64::sqrt((time / 2.0).powf(2.0) - distance).floor();
    x2 as i64 - x1 as i64
}

#[allow(dead_code)]
pub(crate) fn b_naive(input: &str) -> i64 {
    let mut lines = read(input, |l| l.unwrap());
    let time: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .collect::<String>()
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let to_beat: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .collect::<String>()
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let mut hold: i64 = 0;
    loop {
        let travelled = (time - hold) * hold;
        if travelled > to_beat {
            break;
        }
        hold += 1;
    }

    let mut local_victories: i64 = 0;
    loop {
        let travelled = (time - hold) * hold;
        if travelled <= to_beat {
            break;
        }
        hold += 1;
        local_victories += 1;
    }
    local_victories
}

fn victories(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let mut victories: Vec<i64> = Vec::with_capacity(times.len());
    for (i, time) in times.iter().enumerate() {
        let to_beat = distances.get(i).unwrap();
        let mut hold: i64 = 0;
        loop {
            let travelled = (time - hold) * hold;
            if travelled > *to_beat {
                break;
            }
            hold += 1;
        }

        let mut local_victories: i64 = 0;
        loop {
            let travelled = (time - hold) * hold;
            if travelled <= *to_beat {
                break;
            }
            hold += 1;
            local_victories += 1;
        }

        victories.push(local_victories);
    }

    victories.into_iter().product()
}
