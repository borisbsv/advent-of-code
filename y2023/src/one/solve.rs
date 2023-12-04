use std::iter::Peekable;
use std::str::Chars;

use crate::util::read::read;

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
    let lines = read(input, |l| l.unwrap());
    let mut fin: i32 = 0;

    for l in lines {
        let mut left: char = '\0';
        let mut right: char = '\0';
        let mut iter = l.chars().peekable();
        while let Some(c) = iter.next() {
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if left == '\0' {
                        left = c
                    }
                    right = c
                }
                'o' if try_finish("ne", &mut iter) => {
                    if left == '\0' {
                        left = '1'
                    }
                    right = '1'
                }
                't' if try_finish("wo", &mut iter) => {
                    if left == '\0' {
                        left = '2'
                    }
                    right = '2'
                }
                't' if try_finish("hree", &mut iter) => {
                    if left == '\0' {
                        left = '3'
                    }
                    right = '3'
                }
                'f' if try_finish("our", &mut iter) => {
                    if left == '\0' {
                        left = '4'
                    }
                    right = '4'
                }
                'f' if try_finish("ive", &mut iter) => {
                    if left == '\0' {
                        left = '5'
                    }
                    right = '5'
                }
                's' if try_finish("ix", &mut iter) => {
                    if left == '\0' {
                        left = '6'
                    }
                    right = '6'
                }
                's' if try_finish("even", &mut iter) => {
                    if left == '\0' {
                        left = '7'
                    }
                    right = '7'
                }
                'e' if try_finish("ight", &mut iter) => {
                    if left == '\0' {
                        left = '8'
                    }
                    right = '8'
                }
                'n' if try_finish("ine", &mut iter) => {
                    if left == '\0' {
                        left = '9'
                    }
                    right = '9'
                }
                _ => {}
            }
        }

        let t = format!("{left}{right}");
        println!("|||||{t}|||||");
        fin += t.parse::<i32>().unwrap();
    }

    fin
}

fn try_finish(digit: &str, iter: &mut Peekable<Chars<'_>>) -> bool {
    // println!("----");
    // println!("try_finish {}", digit);
    for c in digit.chars() {
        // println!("{}|{}", c, iter.peek().unwrap_or(&'\0'));
        if iter.peek() != Some(&c) {
            return false;
        }
        iter.next();
    }
    true
}
