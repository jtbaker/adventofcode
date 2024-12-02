use std::io::{self, BufRead};

use common;
fn main() -> Result<(), io::Error> {
    let result = solve_day01()?;
    println!("Solution for Day 01: {}", result);
    Ok(())
}

fn solve_day01() -> Result<i32, std::io::Error> {
    let input = common::read_input("data/inputs/01.txt")?;
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    let mut diff_sum: i32 = 0;
    for line in input.lines() {
        match line {
            Ok(line) => {
                let mut row = line.split_whitespace();
                if let (Some(first), Some(second)) = (row.next(), row.next()) {
                    match first.parse::<i32>() {
                        Ok(number) => left_list.push(number),
                        Err(e) => println!("Failed to parse number: {}", e),
                    }
                    match second.parse::<i32>() {
                        Ok(number) => right_list.push(number),
                        Err(e) => println!("Failed to parse number: {}", e),
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    left_list.sort();
    right_list.sort();

    for (i1, i2) in left_list.iter().zip(right_list.iter()) {
        let diff = (i2 - i1).abs();
        diff_sum += diff;
    }
    return Ok(diff_sum);
}
