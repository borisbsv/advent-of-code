use std::collections::HashMap;

use crate::util::read::read;

#[allow(dead_code)]
pub(crate) fn a(input: &str) -> i32 {
    let mtx: Vec<Vec<char>> = read(input, |l| l.unwrap().chars().collect()).collect();
    let mut parts: Vec<i32> = vec![];

    for (i, row) in mtx.iter().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut start = 0;
        for (j, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                if number.is_empty() {
                    start = j;
                }
                number.push(*cell);
                continue;
            }

            if !number.is_empty() && neighbors(&mtx, i, start, j - 1) {
                let n = number.iter().collect::<String>().parse::<i32>().unwrap();
                // println!("pushing {n}");
                parts.push(n);
            }
            number.clear();
        }

        if !number.is_empty() && neighbors(&mtx, i, start, row.len() - 1) {
            let n = number.iter().collect::<String>().parse::<i32>().unwrap();
            // println!("pushing {n}");
            parts.push(n);
        }
    }

    parts.iter().sum()
}

#[allow(dead_code)]
pub(crate) fn b(input: &str) -> i32 {
    let mtx: Vec<Vec<char>> = read(input, |l| l.unwrap().chars().collect()).collect();
    let mut cogs: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, row) in mtx.iter().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut start = 0;
        for (j, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                if number.is_empty() {
                    start = j;
                }
                number.push(*cell);
                continue;
            }

            if !number.is_empty() {
                let nbs = cog_neighbors(&mtx, i, start, j - 1);
                let n = number.iter().collect::<String>().parse::<i32>().unwrap();
                for c in nbs.iter() {
                    cogs.entry(*c).or_default().push(n);
                }
            }
            number.clear();
        }

        if !number.is_empty() {
            let nbs = cog_neighbors(&mtx, i, start, row.len() - 1);
            let n = number.iter().collect::<String>().parse::<i32>().unwrap();
            for c in nbs.iter() {
                cogs.entry(*c).or_default().push(n);
            }
        }
    }

    cogs.values()
        .map(|ns| {
            if ns.len() != 2 {
                return 0;
            }
            ns.first().unwrap() * ns.get(1).unwrap()
        })
        .sum()
}

#[allow(dead_code)]
fn neighbors(mtx: &Vec<Vec<char>>, row: usize, left: usize, right: usize) -> bool {
    // println!("nbs for {} {} {}", row, left, right);
    if row > 0 {
        let r = mtx.get(row - 1).unwrap();
        let start = if left > 0 { left - 1 } else { left };
        let end = if right < r.len() - 1 {
            right + 1
        } else {
            right
        };

        // println!("pre {:?}", r[start..end + 1].to_owned());
        for c in r[start..end + 1].iter() {
            if !c.is_ascii_digit() && *c != '.' {
                return true;
            }
        }
    }

    if row < mtx.len() - 2 {
        let r = mtx.get(row + 1).unwrap();
        let start = if left > 0 { left - 1 } else { left };
        let end = if right < r.len() - 1 {
            right + 1
        } else {
            right
        };

        // println!("after {:?}", r[start..end + 1].to_owned());
        for c in r[start..end + 1].iter() {
            if !c.is_ascii_digit() && *c != '.' {
                return true;
            }
        }
    }

    let r = mtx.get(row).unwrap();
    if left > 0 {
        let c = r.get(left - 1).unwrap();
        // println!("left {} {}", left - 1, c);
        if !c.is_ascii_digit() && *c != '.' {
            return true;
        }
    }

    if right < r.len() - 1 {
        let c = r.get(right + 1).unwrap();
        // println!("right {} {}", right + 1, c);
        if !c.is_ascii_digit() && *c != '.' {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn cog_neighbors(
    mtx: &Vec<Vec<char>>,
    row: usize,
    left: usize,
    right: usize,
) -> Vec<(usize, usize)> {
    let mut nbs: Vec<(usize, usize)> = vec![];
    // println!("nbs for {} {} {}", row, left, right);
    if row > 0 {
        let r = mtx.get(row - 1).unwrap();
        let start = if left > 0 { left - 1 } else { left };
        let end = if right < r.len() - 1 {
            right + 1
        } else {
            right
        };

        // println!("pre {:?}", r[start..end + 1].to_owned());
        for (i, c) in r[start..end + 1].iter().enumerate() {
            if *c == '*' {
                nbs.push((row - 1, i + start));
            }
        }
    }

    if row < mtx.len() - 2 {
        let r = mtx.get(row + 1).unwrap();
        let start = if left > 0 { left - 1 } else { left };
        let end = if right < r.len() - 1 {
            right + 1
        } else {
            right
        };

        // println!("after {:?}", r[start..end + 1].to_owned());
        for (i, c) in r[start..end + 1].iter().enumerate() {
            if *c == '*' {
                nbs.push((row + 1, i + start));
            }
        }
    }

    let r = mtx.get(row).unwrap();
    if left > 0 {
        let c = r.get(left - 1).unwrap();
        // println!("left {} {}", left - 1, c);
        if *c == '*' {
            nbs.push((row, left - 1));
        }
    }

    if right < r.len() - 1 {
        let c = r.get(right + 1).unwrap();
        // println!("right {} {}", right + 1, c);
        if *c == '*' {
            nbs.push((row, right + 1));
        }
    }

    nbs
}
