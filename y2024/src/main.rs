use std::fmt;

mod d01;
mod d02;

mod util;

fn main() {
    println!("┌{}┬{}┬{}┐", "─".repeat(14), "─".repeat(29), "─".repeat(19));
    println!("{}", bench(d01::solve::a, "src/d01/input"));
    println!("{}", bench(d01::solve::b, "src/d01/input"));
    println!("{}", bench(d02::solve::a, "src/d02/input"));
    println!("{}", bench(d02::solve::b, "src/d02/input2"));
    println!("└{}┴{}┴{}┘", "─".repeat(14), "─".repeat(29), "─".repeat(19));
    println!("Hello, world!");
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

fn bench<T, F>(f: F, input: &str) -> Result
where
    T: ToString,
    F: FnOnce(&str) -> T,
{
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
