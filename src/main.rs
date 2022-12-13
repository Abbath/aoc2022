use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day_01() {
    let file = File::open("01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut max_num1 = 0u64;
    let mut max_num2 = 0u64;
    let mut max_num3 = 0u64;

    let mut sum = 0u64;
    for (n, line) in lines.iter().enumerate() {
        sum += line.parse::<u64>().unwrap_or_default();
        if line.is_empty() || n == lines.len() - 1 {
            if sum > max_num1 {
                max_num3 = max_num2;
                max_num2 = max_num1;
                max_num1 = sum;
            } else if sum > max_num2 {
                max_num3 = max_num2;
                max_num2 = sum;
            } else if sum > max_num3 {
                max_num3 = sum;
            }
            sum = 0;
            continue;
        }
    }
    println!("{} {}", max_num1, max_num1 + max_num2 + max_num3);
}

fn day_02() {
    enum Shape {
        Rock = 1,
        Paper = 2,
        Scissors = 3
    }
    enum Outcome {
        Win = 6,
        Lose = 0,
        Draw = 3
    }
    fn calculate(a: Shape, b: Shape) -> u64 {
        match (a, b) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Paper, Shape::Scissors) => 0,    
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
        }
    }
    fn calculate2(a: Shape, b: Outcome) -> Shape {
        match (a, b) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,    
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
    fn translate(a: &str) -> Shape {
        match a {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("No such pattern {}", a)
        }
    }
    fn translate2(a: &str) -> Outcome {
        match a {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("No such pattern {}", a)
        }
    }
    let file = File::open("02/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut sum = 0u64;
    let mut sum2 = 0u64;
    for line in lines.iter() {
        let words: Vec<&str> = line.split(" ").collect();
        sum += calculate(translate(words[1]), translate(words[0])) + translate(words[1]) as u64;
        sum2 += calculate2(translate(words[0]), translate2(words[1])) as u64 + translate2(words[1]) as u64;
    }
    println!("{} {}", sum, sum2);
}

fn main() {
    day_01();
    day_02();
}
