use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let path = Path::new("./input.txt");
    let robots = load_from_file(&path);
    let puzzle = Puzzle {
        robots: robots,
        width: 101,
        height: 103,
    };
    println!("{:?}", puzzle);
    part1(puzzle);
}

struct Puzzle {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counts = HashMap::new();

        // Count occurrences of each coordinate
        for r in &self.robots {
            *counts.entry(&r.p).or_insert(0) += 1;
        }

        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let count = counts.get(&Vec2 { x: x, y: y }).copied().unwrap_or(0);
                if count == 0 {
                    write!(f, ".");
                } else {
                    write!(f, "{}", std::char::from_digit(count, 10).unwrap());
                }
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Robot {
    p: Vec2, // Position
    v: Vec2, // Velosity
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn load_from_file(path: &Path) -> Vec<Robot> {
    let file = File::open(&path).expect("");
    let reader = BufReader::new(file);

    let mut result = vec![];
    let re = Regex::new(r"-?\d+").expect("Hardcoded regex should be valid.");

    for line in reader.lines().map_while(Result::ok) {
        let digits: Vec<i32> = re
            .find_iter(&line)
            .map(|m| m.as_str().parse().expect("TODO"))
            .collect();

        let robot = Robot {
            p: Vec2 {
                x: digits[0],
                y: digits[1],
            },
            v: Vec2 {
                x: digits[2],
                y: digits[3],
            },
        };
        result.push(robot);
    }
    return result;
}

fn part1(puzzle: Puzzle) {
    let mut puzzle = puzzle;
    let end_time = 6876;
    println!("{:?}", puzzle.robots[0]);
    for r in &mut puzzle.robots {
        r.p.x = ((r.p.x + r.v.x * end_time) % puzzle.width + puzzle.width) % puzzle.width;
        r.p.y = ((r.p.y + r.v.y * end_time) % puzzle.height  + puzzle.height) % puzzle.height;
    }
    println!("{:?}", puzzle.robots[0]);

    println!("{:?}", puzzle);
    calculate_safety_factor(puzzle);
}

fn calculate_safety_factor(puzzle: Puzzle) -> i32 {
    let mut factors = vec![0, 0, 0, 0];
    for robot in puzzle.robots {
        match robot.p {
            Vec2 { x: 0..50, y: 0..51 } => factors[0] += 1,
            Vec2 {
                x: 0..50,
                y: 52..103,
            } => factors[1] += 1,
            Vec2 {
                x: 51..101,
                y: 0..51,
            } => factors[2] += 1,
            Vec2 {
                x: 51..101,
                y: 52..103,
            } => factors[3] += 1,
            p => println!("Ignored: {:?}", p),
        };
    }
    println!("{:?}", factors);
    let safety_factor = factors.iter().product();
    println!("{:?}", safety_factor);

    safety_factor
}

fn _pp(robots: &Vec<Robot>) {}
