use std::fmt;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

mod util;

fn main() {
    println!("┌{}┬{}┬{}┐", "─".repeat(14), "─".repeat(29), "─".repeat(19));
    println!("{}", bench(d01::solve::a, "src/d01/input"));
    println!("{}", bench(d01::solve::b, "src/d01/input"));
    println!("{}", bench(d02::solve::a, "src/d02/input"));
    println!("{}", bench(d02::solve::b, "src/d02/input"));
    println!("{}", bench(d03::solve::a, "src/d03/input"));
    println!("{}", bench(d03::solve::b, "src/d03/input"));
    println!("{}", bench(d04::solve::a, "src/d04/input"));
    println!("{}", bench(d04::solve::b, "src/d04/input"));
    println!("{}", bench(d05::solve::a, "src/d05/input"));
    println!("{}", bench(d05::solve::b, "src/d05/input"));
    println!("{}", bench(d06::solve::a, "src/d06/input"));
    println!("{}", bench(d06::solve::b, "src/d06/input"));
    println!("{}", bench(d07::solve::a, "src/d07/input"));
    println!("{}", bench(d08::solve::a, "src/d08/input"));
    println!("{}", bench(d08::solve::b, "src/d08/input"));
    println!("{}", bench(d09::solve::a, "src/d09/input"));
    println!("{}", bench(d09::solve::b, "src/d09/input"));
    println!("{}", bench(d10::solve::a, "src/d10/input"));
    println!("{}", bench(d10::solve::b, "src/d10/input"));
    println!("└{}┴{}┴{}┘", "─".repeat(14), "─".repeat(29), "─".repeat(19));
}

struct Result(String, String, std::time::Duration);

impl fmt::Display for Result {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "│day {:10}│ result: {:20}│ took: {:12?}│",
            self.0, self.1, self.2
        )
    }
}

fn bench<T: ToString>(f: impl FnOnce(&str) -> T, input: &str) -> Result {
    use std::time::Instant;
    let now = Instant::now();
    let res = f(input);
    let elapsed = now.elapsed();

    Result(
        input.split('/').nth(1).unwrap().to_string(),
        res.to_string(),
        elapsed,
    )
}
