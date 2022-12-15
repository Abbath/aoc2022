use std::char;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
                }
                None => dropout = true,
            }
        } else {
            skip = false;
            x += val;
        }
        if cycle % 40 == 0 {
            println!();
        }
        if cycle % 40 + 1 == x || cycle % 40 - 1 == x || cycle % 40 == x {
            print!("#");
        } else {
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

fn day_11() {
    let file = File::open("11/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut si: Vec<VecDeque<u64>> = vec![];
    let mut op: Vec<(i64, i64, i64)> = vec![];
    let mut tests: Vec<(u64, u64, u64)> = vec![];
    let mut test = 0u64;
    let mut test_t = 0u64;
    let mut counter = 0usize;
    let mut lcm = 1u64;
    for line in lines {
        let words: Vec<&str> = line.trim().split(' ').collect();
        if words[0] == "Starting" {
            si.push(VecDeque::new());
            if words.len() == 3 {
                si[counter].push_back(words[2].parse::<u64>().unwrap());
            } else {
                for item in words.iter().take(words.len() - 1).skip(2) {
                    si[counter].push_back(item[..item.len() - 1].parse::<u64>().unwrap());
                }
                si[counter].push_back(words.last().unwrap().parse::<u64>().unwrap());
            }
        }
        if words[0] == "Operation:" {
            let op1 = if words[3] == "old" {
                -1
            } else {
                words[3].parse::<i64>().unwrap()
            };
            let op2 = if words[4] == "+" { 0 } else { 1 };
            let op3 = if words[5] == "old" {
                -1
            } else {
                words[5].parse::<i64>().unwrap()
            };
            op.push((op1, op2, op3));
        }
        if words[0] == "Test:" {
            test = words.last().unwrap().parse::<u64>().unwrap();
            lcm *= test;
        }
        if words[0] == "If" && words[1] == "true:" {
            test_t = words.last().unwrap().parse::<u64>().unwrap();
        }
        if words[0] == "If" && words[1] == "false:" {
            let test_f = words.last().unwrap().parse::<u64>().unwrap();
            tests.push((test, test_t, test_f));
            counter += 1;
        }
    }
    let mut si2 = si.clone();
    let mut items: Vec<u64> = vec![0; si.len()];
    for _ in 0..20 {
        for j in 0..si.len() {
            let l = si[j].len();
            for _ in 0..l {
                items[j] += 1;
                let old = si[j].pop_front().unwrap();
                let new = (if op[j].1 == 0 {
                    |x, y| x + y
                } else {
                    |x, y| x * y
                })(
                    if op[j].0 == -1 { old } else { op[j].0 as u64 },
                    if op[j].2 == -1 { old } else { op[j].2 as u64 },
                ) / 3;
                if new % tests[j].0 == 0 {
                    si[tests[j].1 as usize].push_back(new);
                } else {
                    si[tests[j].2 as usize].push_back(new);
                }
            }
        }
    }
    items.sort();
    let mut items2: Vec<u64> = vec![0; si2.len()];
    for _ in 0..10000 {
        for j in 0..si2.len() {
            let l = si2[j].len();
            for _ in 0..l {
                items2[j] += 1;
                let old = si2[j].pop_front().unwrap();
                let new = (if op[j].1 == 0 {
                    |x, y| x + y
                } else {
                    |x, y| x * y
                })(
                    if op[j].0 == -1 { old } else { op[j].0 as u64 },
                    if op[j].2 == -1 { old } else { op[j].2 as u64 },
                ) % lcm;
                if new % tests[j].0 == 0 {
                    si2[tests[j].1 as usize].push_back(new);
                } else {
                    si2[tests[j].2 as usize].push_back(new);
                }
            }
        }
    }
    items2.sort();
    println!(
        "{} {}",
        items.pop().unwrap() * items.pop().unwrap(),
        items2.pop().unwrap() * items2.pop().unwrap()
    );
}

fn day_12() {
    let file = File::open("12/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    let h = lines.len();
    let w = lines[0].len();
    let mut mat = vec![0; w * h];
    let mut start = (0usize, 0usize);
    let mut finish = (0usize, 0usize);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    mat[i * w + j] = 0;
                    start = (i, j);
                }
                'E' => {
                    mat[i * w + j] = 25;
                    finish = (i, j);
                }
                _ => mat[i * w + j] = c as u64 - 'a' as u64,
            }
        }
    }
    let neighbours = |field: &Vec<u64>, idx: usize, h: usize, w: usize| -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let i = idx / w;
        let j = idx % w;
        if i > 0 {
            res.push((
                (i - 1) * w + j,
                if field[idx] as i64 - field[(i - 1) * w + j] as i64 >= -1 {
                    1
                } else {
                    w * h * 10
                },
            ));
        }
        if j > 0 {
            res.push((
                i * w + j - 1,
                if field[idx] as i64 - field[i * w + j - 1] as i64 >= -1 {
                    1
                } else {
                    w * h * 10
                },
            ));
        }
        if i < h - 1 {
            res.push((
                (i + 1) * w + j,
                if field[idx] as i64 - field[(i + 1) * w + j] as i64 >= -1 {
                    1
                } else {
                    w * h * 10
                },
            ));
        }
        if j < w - 1 {
            res.push((
                i * w + j + 1,
                if field[idx] as i64 - field[i * w + j + 1] as i64 >= -1 {
                    1
                } else {
                    w * h * 10
                },
            ));
        }
        res
    };
    fn reconstruct_path(came_from: &HashMap<usize, usize>, current: usize) -> VecDeque<usize> {
        let mut total_path = VecDeque::from(vec![current]);
        let mut curr = current;
        while came_from.contains_key(&curr) {
            curr = came_from[&curr];
            total_path.push_front(curr);
        }
        total_path
    }
    let astar =
        |field: &Vec<u64>, h: usize, w: usize, start: (usize, usize), finish: (usize, usize)| {
            let hh = |u: usize| {
                (finish.0 as i64 - (u / w) as i64).abs() + (finish.1 as i64 - (u % w) as i64).abs()
            };
            let rows = h * w;
            let mut open_set = BinaryHeap::new();
            let mut came_from: HashMap<usize, usize> = HashMap::new();
            open_set.push((Reverse(0), start.0 * w + start.1));
            let mut g_score = vec![rows * 10; rows];
            g_score[start.0 * w + start.1] = 0;
            let mut f_score = vec![rows * 10; rows];
            f_score[start.0 * w + start.1] = finish.0 + finish.1;
            let mut res = 0;
            while let Some((Reverse(_), idx)) = open_set.pop() {
                if idx == finish.0 * w + finish.1 {
                    res = idx;
                    break;
                }
                for (k, v) in neighbours(field, idx, h, w) {
                    let new_score = g_score[idx] + v as usize;
                    if new_score < g_score[k] {
                        came_from.insert(k, idx);
                        g_score[k] = new_score;
                        f_score[k] = new_score + hh(k) as usize;
                        open_set.push((Reverse(f_score[k]), k));
                    }
                }
            }
            let l = reconstruct_path(&came_from, res).len() - 1;
            if l == 0 {
                usize::MAX
            } else {
                l
            }
        };
    print!("{} ", astar(&mat, h, w, start, finish));
    let mut minp = usize::MAX;
    for i in 0..h {
        for j in 0..w {
            if mat[i * w + j] == 0 {
                let s = astar(&mat, h, w, (i, j), finish);
                minp = s.min(minp);
            }
        }
    }
    println!("{}", minp);
}

fn day_13() {
    let file = File::open("13/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Packet {
        List(Vec<Packet>),
        Num(u64),
    }
    use Packet::{List, Num};
    fn parse_int(s: &[char], offset: usize) -> (Packet, usize) {
        let mut res = String::from("");
        let mut i = offset;
        let mut exit = false;
        loop {
            if i == s.len() {
                i -= 1;
                break;
            }
            let c = s[i];
            if c.is_ascii_digit() {
                if exit {
                    break;
                }
                res.push(c);
            }
            if c == '[' && exit {
                break;
            }
            if c == ',' {
                exit = true;
            }
            if c == ']' {
                break;
            }
            i += 1;
        }
        (Num(res.parse().unwrap()), i)
    }
    fn parse_list(s: &[char], offset: usize) -> (Packet, usize) {
        let mut new_offset = offset + 1;
        if s[offset] == '[' {
            let mut l = vec![];
            loop {
                if s[new_offset] == '[' {
                    let (p1, o1) = parse_list(s, new_offset);
                    new_offset = o1;
                    l.push(p1);
                } else if s[new_offset] == ']' {
                    new_offset += 1;
                    if new_offset < s.len() && s[new_offset] == ',' {
                        new_offset += 1;
                    }
                    break;
                } else {
                    let (n1, o1) = parse_int(s, new_offset);
                    new_offset = o1;
                    l.push(n1);
                };
                if s[new_offset] == ']' {
                    new_offset += 1;
                    if new_offset < s.len() && s[new_offset] == ',' {
                        new_offset += 1;
                    }
                    break;
                }
            }
            (List(l), new_offset)
        } else {
            panic!("ZHEPA!");
        }
    }
    fn compare(p1: &Packet, p2: &Packet) -> Option<bool> {
        match (p1, p2) {
            (Num(a), Num(b)) => match a.cmp(b) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Greater => Some(false),
                std::cmp::Ordering::Equal => None,
            },
            (Num(_), List(_)) => compare(&List(vec![p1.clone()]), p2),
            (List(_), Num(_)) => compare(p1, &List(vec![p2.clone()])),
            (List(a), List(b)) => {
                for i in 0..a.len().max(b.len()) {
                    if i == a.len() {
                        return Some(true);
                    }
                    if i == b.len() {
                        return Some(false);
                    }
                    if let Some(x) = compare(&a[i], &b[i]) {
                        return Some(x);
                    }
                }
                None
            }
        }
    }
    let mut packets = vec![];
    let mut big_packets = vec![];
    let mut counter = 1;
    let mut sum = 0;
    for line in lines.iter() {
        if !line.trim().is_empty() {
            let chars: Vec<char> = line.chars().collect();
            packets.push(parse_list(&chars, 0));
        } else {
            big_packets.push(packets[0].0.clone());
            big_packets.push(packets[1].0.clone());
            let res = compare(&packets[0].0, &packets[1].0);
            if let Some(x) = res {
                if x {
                    sum += counter;
                }
            }
            counter += 1;
            packets.clear();
        }
    }
    big_packets.push(packets[0].0.clone());
    big_packets.push(packets[1].0.clone());
    let res = compare(&packets[0].0, &packets[1].0);
    if let Some(x) = res {
        if x {
            sum += counter;
        }
    }
    packets.clear();
    let packet2 = List(vec![List(vec![Num(2)])]);
    let packet6 = List(vec![List(vec![Num(6)])]);
    big_packets.push(packet2.clone());
    big_packets.push(packet6.clone());
    for i in 0..big_packets.len() {
        for j in 0..big_packets.len() {
            if i != j && !compare(&big_packets[i], &big_packets[j]).unwrap() {
                let tmp = big_packets[i].clone();
                big_packets[i] = big_packets[j].clone();
                big_packets[j] = tmp;
            }
        }
    }
    let mut prod = 1;
    for (i, p) in big_packets.iter().rev().enumerate() {
        if *p == packet2 || *p == packet6 {
            prod *= i + 1;
        }
    }
    println!("{sum} {prod}");
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
    day_11();
    day_12();
    day_13();
}
