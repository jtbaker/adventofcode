use std::fs;
use std::io::{self, BufReader, Error};
use std::str::FromStr;

pub fn read_input(filename: &str) -> Result<BufReader<fs::File>, Error> {
    let file = fs::File::open(filename);
    match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            return Ok(reader);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(e);
        }
    }
}

pub fn parse_and_convert<T: FromStr>(input: String) -> Result<Vec<T>, T::Err> {
    input.split_whitespace().map(|s| s.parse::<T>()).collect()
}
