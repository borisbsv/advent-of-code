use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::Map,
};

pub fn read<T, F>(file: &str, mapper: F) -> Map<Lines<BufReader<File>>, F>
where
    F: FnMut(Result<String, io::Error>) -> T,
{
    let file = File::open(file).expect("to open the file");
    io::BufReader::new(file).lines().map(mapper)
}
