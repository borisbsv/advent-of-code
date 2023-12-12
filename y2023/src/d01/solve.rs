use crate::util::read::read;

#[allow(dead_code)]
pub(crate) fn a(input: &str) -> i32 {
    let lines: u32 = read(input, |l| {
        l.unwrap().chars().filter(|c| c.is_ascii_digit()).collect()
    })
    .map(|a: Vec<_>| {
        let t = format!("{}{}", a.first().unwrap(), a.last().unwrap());
        t.parse::<u32>().unwrap()
    })
    .sum();

    lines.try_into().unwrap()
}

pub(crate) fn b(input: &str) -> i32 {
    let lines = read(input, |l| l.unwrap().chars().collect::<Vec<_>>());
    let mut fin: i32 = 0;

    for l in lines {
        let mut left: char = '\0';
        let mut right: char = '\0';
        // let mut iter = l.chars().peekable();
        for i in 0..l.len() {
            let c = l[i];
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if left == '\0' {
                        left = c
                    }
                    right = c
                }
                'o' if try_finish("ne", i, &l) => {
                    if left == '\0' {
                        left = '1'
                    }
                    right = '1'
                }
                't' if try_finish("wo", i, &l) => {
                    if left == '\0' {
                        left = '2'
                    }
                    right = '2'
                }
                't' if try_finish("hree", i, &l) => {
                    if left == '\0' {
                        left = '3'
                    }
                    right = '3'
                }
                'f' if try_finish("our", i, &l) => {
                    if left == '\0' {
                        left = '4'
                    }
                    right = '4'
                }
                'f' if try_finish("ive", i, &l) => {
                    if left == '\0' {
                        left = '5'
                    }
                    right = '5'
                }
                's' if try_finish("ix", i, &l) => {
                    if left == '\0' {
                        left = '6'
                    }
                    right = '6'
                }
                's' if try_finish("even", i, &l) => {
                    if left == '\0' {
                        left = '7'
                    }
                    right = '7'
                }
                'e' if try_finish("ight", i, &l) => {
                    if left == '\0' {
                        left = '8'
                    }
                    right = '8'
                }
                'n' if try_finish("ine", i, &l) => {
                    if left == '\0' {
                        left = '9'
                    }
                    right = '9'
                }
                _ => {}
            }
        }

        let t = format!("{left}{right}");
        fin += t.parse::<i32>().unwrap();
    }

    fin
}

fn try_finish(digit: &str, i: usize, s: &[char]) -> bool {
    for (j, c) in digit.chars().enumerate() {
        let ind = i + j + 1;
        if ind >= s.len() {
            return false;
        }
        if s[ind] != c {
            return false;
        }
    }
    true
}
