use std::fs;
use std::io::{self, BufReader, Error};

pub fn read_input(filename: &str) -> Result<BufReader<fs::File>, Error> {
    let file = fs::File::open(filename);
    match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            return Ok(reader);
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(e);
        }
    }
}
