use crate::util::read::read;

pub(crate) fn a(input: &str) -> i64 {
    let lines = read(input, |l| {
        let line = l
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        extrapolate(&line, |a| *a.last().unwrap(), &std::ops::Add::<i64>::add)
    });
    lines.sum()
}

pub(crate) fn b(input: &str) -> i64 {
    let lines = read(input, |l| {
        let line = l
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        extrapolate(&line, |a| *a.first().unwrap(), &std::ops::Sub::<i64>::sub)
    });
    lines.sum()
}

fn extrapolate<F, M>(ns: &[i64], side: M, op: &F) -> i64
where
    F: Fn(i64, i64) -> i64,
    M: Fn(&[i64]) -> i64,
{
    let deltas: Vec<i64> = ns.windows(2).map(|w| w[1] - w[0]).collect();
    if deltas.iter().all(|d| *d == 0) {
        side(ns)
    } else {
        op(side(ns), extrapolate(&deltas, side, op))
    }
}
