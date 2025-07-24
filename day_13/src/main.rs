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
        .map(calculate_price)
        .fold(0i64, |acc, x| acc + x.unwrap_or(0)); // Wie summiert man i32 richtig auf?
    println!("Solution: {}", total_price);

    let orthogonal_machine = ClawMachine {
        x_a: 1,
        y_a: 0,
        x_b: 0,
        y_b: 1,
        x_p: 3,
        y_p: 5,
    };
    let overshoot_machine = ClawMachine {
        x_a: 2,
        y_a: 0,
        x_b: 0,
        y_b: 2,
        x_p: 3,
        y_p: 5,
    };
    let negative_machine = ClawMachine {
        x_a: 1,
        y_a: 1,
        x_b: 0,
        y_b: 1,
        x_p: 3,
        y_p: 0,
    };
    // let parallel_winning_machine = ClawMachine {
    //     x_a: 1,
    //     y_a: 2,
    //     x_b: 1,
    //     y_b: 2,
    //     x_p: 2,
    //     y_p: 4,
    // };
    // let parallel_overshoot_machine = ClawMachine {
    //     x_a: 2,
    //     y_a: 2,
    //     x_b: 2,
    //     y_b: 2,
    //     x_p: 3,
    //     y_p: 3,
    // };
    // let parallel_missing_machine = ClawMachine {
    //     x_a: 1,
    //     y_a: 2,
    //     x_b: 1,
    //     y_b: 2,
    //     x_p: 1,
    //     y_p: 4,
    // };

    let machines = [
        orthogonal_machine,
        overshoot_machine,
        negative_machine,
        // parallel_winning_machine,
        // parallel_overshoot_machine,
        // parallel_missing_machine,
    ];

    for machine in machines {
        solve(machine);
    }
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

fn parse_lines(lines: &[String]) -> Option<ClawMachine> { // There would be more information if it returned Result ...
    // TODO add some tests
    if lines.len() != 3 {
        return None;
    }
    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("Static regex is static.");
    let re_price = Regex::new(r"X\=(\d+), Y\=(\d+)").expect("Static regex is static.");

    let capture_a = re_button.captures(&lines[0])?;
    let capture_b = re_button.captures(&lines[1])?;
    let capture_p = re_price.captures(&lines[2])?;

    // let offset: i64 = 0;
    let offset: i64 = 10_000_000_000_000; // warum wird i32 vorgschlagen
    Some(ClawMachine {
        x_a: capture_a[1].parse().ok()?,
        y_a: capture_a[2].parse().ok()?,
        x_b: capture_b[1].parse().ok()?,
        y_b: capture_b[2].parse().ok()?,
        x_p: capture_p[1].parse::<i64>().ok()? + offset,
        y_p: capture_p[2].parse::<i64>().ok()? + offset,
    })
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

#[derive(Debug)]
struct Solution {
    number_of_presses_a: i64,
    number_of_presses_b: i64,
}

fn calculate_price(machine: ClawMachine) -> Option<i64> {
    let price_button_a = 3;
    let price_button_b = 1;
    solve(machine)
        .map(|s| s.number_of_presses_a * price_button_a + s.number_of_presses_b * price_button_b)
}

// This solves:
// /x_a  x_b\   /number_of_presses_a\   /x_p\
//|          |°|                     |=|     | 
// \y_a  y_b/   \number_of_presses_b/   \y_p/
fn solve(machine: ClawMachine) -> Option<Solution> {
    let determinant = machine.x_a * machine.y_b - machine.x_b * machine.y_a;
    if determinant == 0 {
        println!("{:?} d={}", machine, determinant);
        panic!(
            "This case could lead to a possible solution (two parallel buttons), however not yet handled"
        )
    } else {
        let dividend_n = machine.y_b * machine.x_p - machine.x_b * machine.y_p;
        let dividend_m = machine.x_a * machine.y_p - machine.y_a * machine.x_p;
        if dividend_n % determinant != 0 || dividend_n % determinant != 0 {
            // number of presses have to be integer
            return None;
        }
        let n = dividend_n / determinant;
        let m = dividend_m / determinant;
        if n < 0 || m < 0 {
            // number of presses have to be positive
            return None;
        }
        println!("{:?} d={}, n={}, m={}", machine, determinant, n, m);
        return Some(Solution {
            number_of_presses_a: n,
            number_of_presses_b: m,
        });
    }
}
