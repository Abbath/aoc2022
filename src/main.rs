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
    let max2 = max_num1 + max_num2 + max_num3;
    println!("{max_num1} {max2}");
}

fn day_02() {
    enum Shape {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }
    enum Outcome {
        Win = 6,
        Lose = 0,
        Draw = 3,
    }
    fn calculate(a: Shape, b: Shape) -> u64 {
        match (a, b) {
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
            (_, _) => 3,
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
            _ => panic!("No such pattern {}", a),
        }
    }
    fn translate2(a: &str) -> Outcome {
        match a {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("No such pattern {}", a),
        }
    }
    let file = File::open("02/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut sum = 0u64;
    let mut sum2 = 0u64;
    for line in lines.iter() {
        let words: Vec<&str> = line.split(' ').collect();
        sum += calculate(translate(words[1]), translate(words[0])) + translate(words[1]) as u64;
        sum2 += calculate2(translate(words[0]), translate2(words[1])) as u64
            + translate2(words[1]) as u64;
    }
    println!("{sum} {sum2}");
}

fn day_03() {
    let file = File::open("03/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut sum = 0u64;
    let mut sum2 = 0u64;
    let mut ls = vec![String::new(), String::new(), String::new()];
    for (i, line) in lines.iter().enumerate() {
        ls[i % 3] = line.to_string();
        if i % 3 == 2 {
            'outer: for c1 in ls[0].chars() {
                for c2 in ls[1].chars() {
                    for c3 in ls[2].chars() {
                        if c1 == c2 && c2 == c3 {
                            sum2 += if c1.is_lowercase() {
                                c1 as u64 - 'a' as u64 + 1
                            } else {
                                c1 as u64 - 'A' as u64 + 27
                            };
                            break 'outer;
                        }
                    }
                }
            }
        }
        let fh = line[0..line.len() / 2].to_string();
        let sh = line[line.len() / 2..].to_string();
        'outer: for c1 in fh.chars() {
            for c2 in sh.chars() {
                if c1 == c2 {
                    sum += if c1.is_lowercase() {
                        c1 as u64 - 'a' as u64 + 1
                    } else {
                        c1 as u64 - 'A' as u64 + 27
                    };
                    break 'outer;
                }
            }
        }
    }
    println!("{sum} {sum2}")
}

fn day_04() {
    let file = File::open("04/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut sum = 0u64;
    let mut sum2 = 0u64;
    for line in lines {
        let pair: Vec<&str> = line.split(',').collect();
        let fp: Vec<u64> = pair[0]
            .split('-')
            .flat_map(|s| s.to_owned().parse::<u64>())
            .collect();
        let sp: Vec<u64> = pair[1]
            .split('-')
            .flat_map(|s| s.to_owned().parse::<u64>())
            .collect();
        if fp[0] <= sp[0] && fp[1] >= sp[1] || sp[0] <= fp[0] && sp[1] >= fp[1] {
            sum += 1;
        }
        if fp[1] >= sp[0] && fp[0] < sp[1] || sp[1] >= fp[0] && sp[0] < fp[1] {
            sum2 += 1;
        }
    }
    println!("{sum} {sum2}")
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
}
