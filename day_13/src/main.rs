use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use regex::Regex;

fn main() {
    let path = Path::new("./input.txt");
    let machines = load_from_file(&path);
    println!("{}", machines.len());
    for machine in machines {
        solve(machine);
    }

    let orthogonal_machine = ClawMachine{ x_a: 1, y_a: 0, x_b: 0, y_b: 1, x_p: 3, y_p: 5 };
    let overshoot_machine = ClawMachine{ x_a: 2, y_a: 0, x_b: 0, y_b: 2, x_p: 3, y_p: 5 };
    let negative_machine = ClawMachine{ x_a: 1, y_a: 1, x_b: 0, y_b: 1, x_p: 3, y_p: 0 };
    let parallel_winning_machine = ClawMachine{ x_a: 1, y_a: 2, x_b: 1, y_b: 2, x_p: 2, y_p: 4 };
    let parallel_overshoot_machine = ClawMachine{ x_a: 2, y_a: 2, x_b: 2, y_b: 2, x_p: 3, y_p: 3 };
    let parallel_missing_machine = ClawMachine{ x_a: 1, y_a: 2, x_b: 1, y_b: 2, x_p: 1, y_p: 4 };

    let machines = [
        orthogonal_machine,
        overshoot_machine,
        negative_machine,
        parallel_winning_machine,
        parallel_overshoot_machine,
        parallel_missing_machine
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
        }
        else {
            block.push(line);
        }
    }
    if !block.is_empty() {
        process_block(&block, &mut result);
    }
    return result;
}

fn process_block(lines: &[String], result: &mut Vec<ClawMachine>) { // TODOs Error handlin in seperat method && some test...
    if lines.len() != 3 {
        panic!("error while reading the file, no complete machine");
    }
    let re_button = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("Static regex is static.");
    let re_price = Regex::new(r"X\=(\d+), Y\=(\d+)").expect("Static regex is static.");

    let capture_a = re_button.captures(&lines[0]).expect("TODO handle");
    let capture_b = re_button.captures(&lines[1]).expect("TODO handle");
    let capture_p = re_price.captures(&lines[2]).expect("TODO handle");


    let a: i32 = capture_a[1].parse().expect("TODO handle");
    let machine = ClawMachine {
        x_a: capture_a[1].parse().expect("TODO handle"),
        y_a: capture_a[2].parse().expect("TODO handle"),
        x_b: capture_b[1].parse().expect("TODO handle"),
        y_b: capture_b[2].parse().expect("TODO handle"),
        x_p: capture_p[1].parse().expect("TODO handle"),
        y_p: capture_p[2].parse().expect("TODO handle")
    };
    // println!("{:?}", machine);
    result.push(machine);
}

#[derive(Debug)]
struct ClawMachine {
    x_a: i32,
    y_a: i32,
    x_b: i32,
    y_b: i32,
    x_p: i32,
    y_p: i32,
}

fn price(machine: ClawMachine) -> i32{
    return 0;
}

fn solve(machine: ClawMachine) {
    let determinant = machine.x_a * machine.y_b - machine.x_b * machine.y_a;
    if determinant == 0 {
        println!("{:?} d={}", machine, determinant);
    }
    else {
        let n = ((machine.y_b * machine.x_p - machine.x_b * machine.y_p) as f32) / (determinant as f32);
        let m = ((machine.x_a * machine.y_p - machine.y_a * machine.x_p) as f32) / (determinant as f32);
        println!("{:?} d={}, n={}, m={}", machine, determinant, n, m);
    }
}
