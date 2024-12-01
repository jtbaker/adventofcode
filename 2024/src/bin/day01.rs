use common;
fn main() {
    println!("Solution for Day 01: {}", solve_day01());
}

fn solve_day01() -> i32 {
    let input = common::read_input("data/inputs/01.txt");
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    let mut diff_sum: i32 = 0;
    for line in input.lines() {
        let mut row = line.split_whitespace();
        if let(Some(first), Some(second)) = (row.next(), row.next()) {
            match first.parse::<i32>() {
                Ok(number)=> left_list.push(number),
                Err(e)=> println!("Failed to parse number: {}", e)
            }
            match second.parse::<i32>() {
                Ok(number)=> right_list.push(number),
                Err(e)=> println!("Failed to parse number: {}", e)
            }
        }
    }
    left_list.sort();
    right_list.sort();

    for (i1, i2) in left_list.iter().zip(right_list.iter()) {
        let diff = (i2 - i1).abs();
        diff_sum += diff;
    }
    return diff_sum;
}
