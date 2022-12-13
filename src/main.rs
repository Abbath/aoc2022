use std::collections::VecDeque;
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
    for line in lines.iter() {
        sum += line.parse::<u64>().unwrap_or_default();
        if line.is_empty() {
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

fn day_05() {
    let file = File::open("05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut crates: Vec<Vec<char>> = vec![];
    let mut sp: bool = false;
    let mut moves: Vec<(u64, u64, u64)> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            sp = true;
            continue;
        }
        if !sp {
            for i in (1..line.len()).step_by(4) {
                let c = line.chars().nth(i).unwrap();
                if c.is_alphabetic() {
                    if crates.len() < i / 4 + 1 {
                        crates.resize(i / 4 + 1, vec![]);
                    }
                    crates[i / 4].push(c);
                }
            }
        } else {
            let words: Vec<&str> = line.split(' ').collect();
            let (n, f, t) = (
                words[1].parse::<u64>().unwrap(),
                words[3].parse::<u64>().unwrap(),
                words[5].parse::<u64>().unwrap(),
            );
            moves.push((n, f, t));
        }
    }
    let mut reversed: Vec<Vec<char>> = vec![];
    for v in crates.iter() {
        reversed.push(v.iter().rev().cloned().collect());
    }
    let mut reversed2 = reversed.clone();
    for (n, f, t) in moves {
        let mut stack: Vec<char> = vec![];
        for _ in 0..n {
            let val = reversed[(f - 1) as usize].pop().unwrap();
            let val2 = reversed2[(f - 1) as usize].pop().unwrap();
            reversed[(t - 1) as usize].push(val);
            stack.push(val2);
        }
        for val in stack.iter().rev() {
            reversed2[(t - 1) as usize].push(val.to_owned());
        }
    }
    for v in reversed.iter() {
        print!("{}", v.last().unwrap());
    }
    print!(" ");
    for v in reversed2.iter() {
        print!("{}", v.last().unwrap());
    }
    println!();
}

fn day_06() {
    let file = File::open("06/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    let mut deque = VecDeque::new();
    let mut res = 0usize;
    let mut deque2 = VecDeque::new();
    let mut res2 = 0usize;
    for (i, c) in line.chars().enumerate() {
        deque.push_back(c);
        if deque.len() == 4 {
            let mut diff = true;
            for (i1, c1) in deque.iter().enumerate() {
                for (i2, c2) in deque.iter().enumerate() {
                    if i1 != i2 && c1 == c2 {
                        diff = false;
                    }
                }
            }
            if diff {
                res = i + 1;
                break;
            }
            deque.pop_front();
        }
    }
    for (i, c) in line.chars().enumerate() {
        deque2.push_back(c);
        if deque2.len() == 14 {
            let mut diff = true;
            for (i1, c1) in deque2.iter().enumerate() {
                for (i2, c2) in deque2.iter().enumerate() {
                    if i1 != i2 && c1 == c2 {
                        diff = false;
                    }
                }
            }
            if diff {
                res2 = i + 1;
                break;
            }
            deque2.pop_front();
        }
    }
    println!("{res} {res2}");
}

fn day_07() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    struct Dir {
        name: String,
        files: Vec<u64>,
        children: Vec<Dir>
    }
    impl Dir {
        fn new(name: String) -> Dir {
            Dir {
                name:  name,
                files: vec![],
                children: vec![]
            }
        }
    }
    let mut curdir: String = String::new();
    let mut stack: Vec<String> = vec![];
    for line in lines.iter() {
        if line.starts_with("$") {
            let words: Vec<&str> = line.split(' ').collect();
            if words[1] == "cd" {
                curdir = words[2].to_string();
            }
        }
        
    }
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
}
