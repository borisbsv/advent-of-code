use std::fmt;

mod one;
mod three;
mod two;

mod util;

fn main() {
    println!("┌{}┬{}┬{}┐", "─".repeat(14), "─".repeat(29), "─".repeat(19));
    println!("{}", bench(one::solve::a, "src/one/input"));
    println!("{}", bench(one::solve::b, "src/one/input"));
    println!("{}", bench(two::solve::a, "src/two/input"));
    println!("{}", bench(two::solve::b, "src/two/input"));
    println!("{}", bench(three::solve::a, "src/three/input"));
    println!("{}", bench(three::solve::b, "src/three/input"));
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
