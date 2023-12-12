use std::fmt;

mod eight;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
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
    println!("{}", bench(four::solve::a, "src/four/input"));
    println!("{}", bench(four::solve::b, "src/four/input"));
    println!("{}", bench(five::solve::a, "src/five/input"));
    println!("{}", bench(five::solve::b, "src/five/input"));
    println!("{}", bench(six::solve::a, "src/six/input"));
    println!("{}", bench(six::solve::b, "src/six/input"));
    println!("{}", bench(seven::solve::a, "src/seven/input"));
    println!("{}", bench(eight::solve::a, "src/eight/input"));
    println!("{}", bench(eight::solve::b, "src/eight/input"));
    println!("{}", bench(nine::solve::a, "src/nine/input"));
    println!("{}", bench(nine::solve::b, "src/nine/input"));
    println!("{}", bench(ten::solve::a, "src/ten/input"));
    println!("{}", bench(ten::solve::b, "src/ten/input"));
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
