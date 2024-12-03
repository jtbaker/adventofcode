use common;
use std::io::BufRead;

trait RowValidator {
    fn is_valid(&self) -> bool;
}

impl RowValidator for Vec<i32> {
    fn is_valid(&self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let is_increasing = (self[1] - self[0]) > 0;
        for (idx, item) in self.iter().enumerate() {
            if idx + 1 == self.len() {
                break;
            }
            let next = self[idx + 1];
            let diff = next - item;
            let abs_diff = diff.abs();
            if abs_diff < 1 || abs_diff > 3 {
                return false;
            }
            if diff <= 0 && is_increasing || diff >= 0 && !is_increasing {
                return false;
            }
        }
        return true;
    }
}

struct DayO2Solver;

impl DayO2Solver {
    fn solve(&self) -> Result<i32, std::io::Error> {
        let lines = common::read_input("data/inputs/02.txt")?;
        let mut valid_row_count = 0;
        for line in lines.lines() {
            match line {
                Ok(line) => {
                    let contents = common::parse_and_convert::<i32>(line);
                    match contents {
                        Ok(c) => {
                            if c.is_valid() {
                                valid_row_count += 1;
                            }
                        }
                        Err(err) => eprintln!("Failed to parse: {:?}", err),
                    }
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }

        Ok(valid_row_count)
    }
}

fn main() {
    let solver = DayO2Solver;
    match solver.solve() {
        Ok(res) => println!("Day 2. Valid Row Count: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}

// Imperative code refactored to trait usage above
// fn solve_day02() -> Result<i32, std::io::Error> {
//     let lines = common::read_input("data/inputs/02.txt")?;
//     let mut valid_row_count = 0;

//     for line in lines.lines() {
//         match line {
//             Ok(line) => {
//                 let row = line.split_whitespace();
//                 let mut contents = Vec::<i32>::new();
//                 for v in row {
//                     let item = v.parse::<i32>();
//                     match item {
//                         Ok(i) => {
//                             contents.push(i);
//                         }
//                         Err(err) => {
//                             eprintln!("Error: {}", err);
//                         }
//                     }
//                 }
//                 let is_valid = valid_row(contents);
//                 if is_valid {
//                     valid_row_count += 1;
//                 }
//             }
//             Err(err) => {
//                 eprintln!("Error: {}", err);
//             }
//         }
//     }
//     return Ok(valid_row_count);
// }

// fn valid_row(inputs: Vec<i32>) -> bool {
//     let increasing = (inputs[1] - inputs[0]) > 0;
//     for (i, item) in inputs.iter().enumerate() {
//         let l = inputs.len();
//         if i + 1 == l {
//             break;
//         }
//         let next = inputs[i + 1];
//         let diff = next - item;
//         let abs_diff = diff.abs();
//         if abs_diff < 1 || abs_diff > 3 {
//             return false;
//         }
//         if increasing && diff <= 0 || !increasing && diff >= 0 {
//             return false;
//         }
//     }
//     return true;
// }
