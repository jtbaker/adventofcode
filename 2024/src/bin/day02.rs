use common;
use std::io::BufRead;

fn main() {
    let result = solve_day02();
    match result {
        Ok(res) => {
            println!("Day 2. Valid Row Count: {}", res);
        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }
}

fn solve_day02() -> Result<i32, std::io::Error> {
    let lines = common::read_input("data/inputs/02.txt")?;
    let mut valid_row_count = 0;

    for line in lines.lines() {
        match line {
            Ok(line) => {
                let row = line.split_whitespace();
                let mut contents = Vec::<i32>::new();
                for v in row {
                    let item = v.parse::<i32>();
                    match item {
                        Ok(i) => {
                            contents.push(i);
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                        }
                    }
                }
                let is_valid = valid_row(contents);
                if is_valid {
                    valid_row_count += 1;
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    return Ok(valid_row_count);
}

fn valid_row(inputs: Vec<i32>) -> bool {
    let increasing = (inputs[1] - inputs[0]) > 0;
    for (i, item) in inputs.iter().enumerate() {
        let l = inputs.len();
        if i + 1 == l {
            break;
        }
        let next = inputs[i + 1];
        let diff = next - item;
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
        if increasing && diff <= 0 || !increasing && diff >= 0 {
            return false;
        }
    }
    return true;
}
