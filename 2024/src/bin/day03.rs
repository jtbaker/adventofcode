use common;
use regex::Regex;
use std;
use std::io::BufRead;

fn main() {
    let day03 = Day03;
    match day03.solve() {
        Ok(res) => {
            println!("Day 3 Result: {}", res)
        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }
}

struct Day03;

impl Day03 {
    fn solve(&self) -> Result<i32, std::io::Error> {
        let reader = common::read_input("data/inputs/03.txt")?;
        let mut sum_product = 0;
        for line in reader.lines() {
            match line {
                Ok(line_contents) => {
                    let row_product = Self::parse_line(&line_contents);
                    sum_product += row_product;
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
        Ok(sum_product)
    }
    fn parse_line(input: &str) -> i32 {
        let mul_pattern = Regex::new(r"mul\([0-9]{1,},[0-9]{1,}\)").unwrap();
        let strip_pattern = Regex::new(r"mul|\(|\)").unwrap();
        let mut row_product = 0;
        let matches: Vec<&str> = mul_pattern
            .find_iter(input)
            .map(|mat| mat.as_str())
            .collect();
        for m in matches {
            let contents: Vec<i32> = strip_pattern
                .replace_all(m, "")
                .split(",")
                .map(|s| s.trim())
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            if contents.len() >= 2 {
                let prod = contents[0] * contents[1];
                row_product += prod;
            }
        }
        return row_product;
    }
}
