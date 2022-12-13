use std::char;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let file = File::open("07/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut stack: Vec<String> = vec![];
    let mut bins: HashMap<String, u64> = HashMap::new();
    for line in lines.iter() {
        let words: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') && words[1] == "cd" {
            if words[2] == ".." {
                stack.pop();
            } else if words[2] != "/" {
                stack.push(words[2].to_string());
                let key = "/".to_string() + &stack.join("/").to_owned();
                *bins.entry(key).or_insert(0) = 0;
            }
        }
        if let Ok(n) = words[0].parse::<u64>() {
            let key = "/".to_string() + &stack.join("/").to_owned();
            *bins.entry(key.clone()).or_insert(0) += n;
            let mut to_add: Vec<(String, u64)> = vec![];
            for k in bins.keys() {
                if k != &key && key.starts_with(k) {
                    to_add.push((k.to_string(), n));
                }
            }
            for (k, v) in to_add {
                *bins.entry(k).or_default() += v;
            }
        }
    }
    let cap = 70000000u64;
    let needed = 30000000u64;
    let free_space = cap - bins["/"];
    let mut sum = 0u64;
    let mut smallest = bins["/"];
    for bin in bins.values() {
        if *bin < 100000u64 {
            sum += bin;
        }
        if free_space + *bin >= needed && *bin < smallest {
            smallest = *bin;
        }
    }
    println!("{sum} {smallest}");
}

fn day_08() {
    let file = File::open("08/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut mat: Vec<u64> = Vec::new();
    mat.resize(lines.len() * lines[0].len(), 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            mat[i * line.len() + j] = c as u64 - '0' as u64;
        }
    }
    let h = lines.len();
    let w = lines[0].len();
    let mut sum = 0u64;
    let mut max_score = 0u64;
    for i in 0..h {
        for j in 0..w {
            if i == 0 || j == 0 || i == h - 1 || j == w - 1 {
                sum += 1;
                continue;
            }
            let a = mat.iter().skip(i * w).take(j).all(|x| *x < mat[i * w + j]);
            let b = mat
                .iter()
                .skip(i * w + j + 1)
                .take(w - j - 1)
                .all(|x| *x < mat[i * w + j]);
            let c = mat
                .iter()
                .skip(j)
                .step_by(w)
                .take(i)
                .all(|x| *x < mat[i * w + j]);
            let d = mat
                .iter()
                .skip((i + 1) * w + j)
                .step_by(w)
                .take(h - i - 1)
                .all(|x| *x < mat[i * w + j]);
            if a || b || c || d {
                sum += 1;
            }
            let mut a1 = mat
                .iter()
                .skip(i * w)
                .take(j)
                .rev()
                .take_while(|x| **x < mat[i * w + j])
                .count();
            if a1 < j {
                a1 += 1;
            }
            let mut b1 = mat
                .iter()
                .skip(i * w + j + 1)
                .take(w - j - 1)
                .take_while(|x| **x < mat[i * w + j])
                .count();
            if b1 < w - j - 1 {
                b1 += 1;
            }
            let mut c1 = mat
                .iter()
                .skip(j)
                .step_by(w)
                .take(i)
                .rev()
                .take_while(|x| **x < mat[i * w + j])
                .count();
            if c1 < i {
                c1 += 1;
            }
            let mut d1 = mat
                .iter()
                .skip((i + 1) * w + j)
                .step_by(w)
                .take(h - i - 1)
                .take_while(|x| **x < mat[i * w + j])
                .count();
            if d1 < h - i - 1 {
                d1 += 1;
            }
            let score = a1 * b1 * c1 * d1;
            if score > max_score as usize {
                max_score = score as u64;
            }
        }
    }
    println!("{sum} {max_score}");
}

fn day_09() {
    let file = File::open("09/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    let mut positions2: HashSet<(i64, i64)> = HashSet::new();
    let mut head = vec![0i64, 0i64];
    let mut tail = vec![0i64, 0i64];
    let mut long_tail: Vec<Vec<i64>> = vec![vec![0i64, 0i64]; 10];
    positions.insert((tail[0], tail[1]));
    positions2.insert((long_tail[9][0], long_tail[9][1]));
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        let dir = words[0];
        let dist = words[1].parse::<i64>().unwrap();
        for _ in 0..dist {
            match dir {
                "R" => head[0] += 1,
                "L" => head[0] -= 1,
                "U" => head[1] += 1,
                "D" => head[1] -= 1,
                _ => panic!("Wrong direction!"),
            }
            match dir {
                "R" => long_tail[0][0] += 1,
                "L" => long_tail[0][0] -= 1,
                "U" => long_tail[0][1] += 1,
                "D" => long_tail[0][1] -= 1,
                _ => panic!("Wrong direction!"),
            }

            if (head[0] - tail[0]).abs() == 2 && head[1] == tail[1] {
                if head[0] > tail[0] {
                    tail[0] += 1;
                } else {
                    tail[0] -= 1;
                }
            } else if (head[1] - tail[1]).abs() == 2 && head[0] == tail[0] {
                if head[1] > tail[1] {
                    tail[1] += 1;
                } else {
                    tail[1] -= 1;
                }
            } else if (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
                if head[0] > tail[0] {
                    tail[0] += 1;
                } else {
                    tail[0] -= 1;
                }
                if head[1] > tail[1] {
                    tail[1] += 1;
                } else {
                    tail[1] -= 1;
                }
            }

            for i in 1..10 {
                if (long_tail[i - 1][0] - long_tail[i][0]).abs() == 2
                    && long_tail[i - 1][1] == long_tail[i][1]
                {
                    if long_tail[i - 1][0] > long_tail[i][0] {
                        long_tail[i][0] += 1;
                    } else {
                        long_tail[i][0] -= 1;
                    }
                } else if (long_tail[i - 1][1] - long_tail[i][1]).abs() == 2
                    && long_tail[i - 1][0] == long_tail[i][0]
                {
                    if long_tail[i - 1][1] > long_tail[i][1] {
                        long_tail[i][1] += 1;
                    } else {
                        long_tail[i][1] -= 1;
                    }
                } else if (long_tail[i - 1][0] - long_tail[i][0]).abs() > 1
                    || (long_tail[i - 1][1] - long_tail[i][1]).abs() > 1
                {
                    if long_tail[i - 1][0] > long_tail[i][0] {
                        long_tail[i][0] += 1;
                    } else {
                        long_tail[i][0] -= 1;
                    }
                    if long_tail[i - 1][1] > long_tail[i][1] {
                        long_tail[i][1] += 1;
                    } else {
                        long_tail[i][1] -= 1;
                    }
                }
            }
            positions2.insert((long_tail[9][0], long_tail[9][1]));
            positions.insert((tail[0], tail[1]));
        }
    }
    println!("{} {}", positions.len(), positions2.len());
}

fn day_10() {
    let file = File::open("10/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut x = 1i64;
    let mut cycle = 1i64;
    let mut iter = lines.iter();
    let mut skip = false;
    let mut val = 0i64;
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0i64;
    let mut dropout = false;
    loop {
        if cycles.contains(&cycle) {
            sum += x * cycle as i64;
        }
        if !skip {
            let lineop = iter.next();
            match lineop {
                Some(line) => {
                    let words: Vec<&str> = line.split(' ').collect();
                    if words[0] == "addx" {
                        skip = true;
                        val = words[1].parse::<i64>().unwrap();
                    }
                },
                None => dropout = true,
            }
        }else{
            skip = false;
            x += val;
        }
        if cycle % 40 == 0 {
            println!();
        }
        if cycle % 40 + 1 == x  || cycle % 40 - 1 == x  || cycle % 40 == x  {
            print!("#");
        }else{
            print!(".");
        }
        if cycle == 1 {
            print!("#");
        }
        cycle += 1; 
        if dropout {
            break;
        }
    }
    println!("{sum}");
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
    day_09();
    day_10();
}
