use crate::util::read::read;

use itertools::Itertools;

pub(crate) fn a(input: &str) -> i32 {
    solve(input, false)
}

pub(crate) fn b(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, skip: bool) -> i32 {
    read(input, |l| {
        let mut skipped: bool = false;
        let mut decreasing: bool = false;
        let binding = l.unwrap();
        let it = binding.split(' ');
        let start: Vec<_> = it.clone().take(3).collect();
        let mut next = it.tuple_windows::<(_, _)>();

        let l = start[0].parse::<i32>().unwrap();
        let m = start[1].parse::<i32>().unwrap();
        let r = start[2].parse::<i32>().unwrap();
        let ldelta = l - m;
        let mdelta = l - r;
        let rdelta = m - r;
        if safe(true, ldelta) {
            decreasing = true;
        } else if safe(false, ldelta) {
            decreasing = false;
        } else if safe(true, mdelta) {
            decreasing = true;
            next.next();
        } else if safe(false, rdelta) {
            decreasing = false;
            next.next();
        } else if safe(true, rdelta) {
            decreasing = true;
            next.next();
        } else if safe(false, rdelta) {
            decreasing = false;
            next.next();
        }

        while let Some((lhs, rhs)) = next.next() {
            let ilhs = lhs.parse::<i32>().unwrap();
            let irhs = rhs.parse::<i32>().unwrap();
            let delta = ilhs - irhs;

            if !safe(decreasing, delta) {
                if !skip || skipped {
                    return 0;
                }

                skipped = true;
                let next = next.next();
                if let Some((_, rhs)) = next {
                    let irhs = rhs.parse::<i32>().unwrap();
                    let delta = ilhs - irhs;
                    if !safe(decreasing, delta) {
                        println!("ret 0");
                        return 0;
                    }
                }
            }
        }

        println!("VALID {:?}", binding);
        1
    })
    .sum::<i32>()
}

fn safe(decreasing: bool, delta: i32) -> bool {
    if delta.abs() > 3 || delta == 0 {
        return false;
    }

    if decreasing && delta < 0 {
        return false;
    }

    if !decreasing && delta > 0 {
        return false;
    }
    true
}
