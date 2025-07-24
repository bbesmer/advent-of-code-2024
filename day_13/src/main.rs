use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let path = Path::new("./input.txt");
    let machines = load_from_file(&path);
    println!("Number of machines: {}", machines.len());

    let total_price: i64 = machines
        .into_iter()
        .map(fix_unit_conversion_error) // i.e. part 2
        .filter_map(solve)
        .map(calculate_tokens)
        .sum();

    println!("Solution: {}", total_price);
}

#[derive(Debug)]
struct ClawMachine {
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_p: i64,
    y_p: i64,
}

#[derive(Debug, PartialEq)]
struct Solution {
    presses_on_a: i64,
    presses_on_b: i64,
}

fn load_from_file(path: &Path) -> Vec<ClawMachine> {
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut result = vec![];
    let mut block = vec![];
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            process_block(&block, &mut result);
            block.clear();
        } else {
            block.push(line);
        }
    }
    if !block.is_empty() {
        process_block(&block, &mut result);
    }
    return result;
}

fn process_block(lines: &[String], result: &mut Vec<ClawMachine>) {
    result.push(
        parse_lines(lines).unwrap_or_else(|| panic!("Error while parsing machine {:?}", lines)),
    );
}

// There would be more information if it returned Result ...
fn parse_lines(lines: &[String]) -> Option<ClawMachine> {
    if lines.len() != 3 {
        return None;
    }
    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("Static regex is static.");
    let re_price = Regex::new(r"X\=(\d+), Y\=(\d+)").expect("Static regex is static.");

    let capture_a = re_button.captures(&lines[0])?;
    let capture_b = re_button.captures(&lines[1])?;
    let capture_p = re_price.captures(&lines[2])?;

    Some(ClawMachine {
        x_a: capture_a[1].parse().ok()?,
        y_a: capture_a[2].parse().ok()?,
        x_b: capture_b[1].parse().ok()?,
        y_b: capture_b[2].parse().ok()?,
        x_p: capture_p[1].parse().ok()?,
        y_p: capture_p[2].parse().ok()?,
    })
}

fn fix_unit_conversion_error(machine: ClawMachine) -> ClawMachine {
    let offset: i64 = 10_000_000_000_000;
    let mut machine = machine;
    machine.x_p += offset;
    machine.y_p += offset;
    machine
}

fn calculate_tokens(solution: Solution) -> i64 {
    let price_button_a = 3;
    let price_button_b = 1;

    solution.presses_on_a * price_button_a + solution.presses_on_b * price_button_b
}

// This solves:
// /x_a  x_b\   /presses_on_a\   /x_p\
//|          |*|              |=|     |
// \y_a  y_b/   \presses_on_b/   \y_p/
fn solve(machine: ClawMachine) -> Option<Solution> {
    let determinant = machine.x_a * machine.y_b - machine.x_b * machine.y_a;
    if determinant == 0 {
        println!("{:?} d={}", machine, determinant);
        panic!(
            "This case could lead to a possible solution (two parallel buttons), however not yet handled" // TODO implement
        )
    } else {
        let dividend_n = machine.y_b * machine.x_p - machine.x_b * machine.y_p;
        let dividend_m = machine.x_a * machine.y_p - machine.y_a * machine.x_p;
        // number of presses have to be integer
        if dividend_n % determinant != 0 || dividend_m % determinant != 0 {
            return None;
        }

        let n = dividend_n / determinant;
        let m = dividend_m / determinant;
        // number of presses have to be positive
        if n < 0 || m < 0 {
            return None;
        }

        // println!("{:?} d={}, n={}, m={}", machine, determinant, n, m);
        return Some(Solution {
            presses_on_a: n,
            presses_on_b: m,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let sample_lines = vec![
            String::from("Button A: X+1, Y+2"),
            String::from("Button B: X+3, Y+4"),
            String::from("Prize: X=5, Y=6"),
        ];

        let machine = parse_lines(&sample_lines).expect("Parsing failed");
        assert_eq!(machine.x_a, 1);
        assert_eq!(machine.y_a, 2);
        assert_eq!(machine.x_b, 3);
        assert_eq!(machine.y_b, 4);
        assert_eq!(machine.x_p, 5);
        assert_eq!(machine.y_p, 6);
    }

    #[test]
    fn test_orthogonal_machine() {
        let machine = ClawMachine {
            x_a: 1,
            y_a: 0,
            x_b: 0,
            y_b: 1,
            x_p: 3,
            y_p: 5,
        };
        let result = solve(machine);
        assert_eq!(
            result,
            Some(Solution {
                presses_on_a: 3,
                presses_on_b: 5,
            })
        );
    }

    #[test]
    fn test_overshoot_machine() {
        let machine = ClawMachine {
            x_a: 2,
            y_a: 0,
            x_b: 0,
            y_b: 2,
            x_p: 3,
            y_p: 5,
        };
        assert_eq!(solve(machine), None);
    }

    #[test]
    fn test_negative_machine() {
        let machine = ClawMachine {
            x_a: 1,
            y_a: 1,
            x_b: 0,
            y_b: 1,
            x_p: 3,
            y_p: 0,
        };
        assert_eq!(solve(machine), None);
    }

    #[test]
    #[should_panic(expected = "This case could lead to a possible solution")]
    fn test_parallel_winning_machine() {
        let machine = ClawMachine {
            x_a: 1,
            y_a: 2,
            x_b: 1,
            y_b: 2,
            x_p: 2,
            y_p: 4,
        };
        solve(machine);
    }

    #[test]
    #[should_panic(expected = "This case could lead to a possible solution")]
    fn test_parallel_overshoot_machine() {
        let machine = ClawMachine {
            x_a: 2,
            y_a: 2,
            x_b: 2,
            y_b: 2,
            x_p: 3,
            y_p: 3,
        };
        solve(machine);
    }

    #[test]
    #[should_panic(expected = "This case could lead to a possible solution")]
    fn test_parallel_missing_machine() {
        let machine = ClawMachine {
            x_a: 1,
            y_a: 2,
            x_b: 1,
            y_b: 2,
            x_p: 1,
            y_p: 4,
        };
        solve(machine);
    }
}
