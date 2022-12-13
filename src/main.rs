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
    println!("{}", max_num1);
    println!("{}", max_num1 + max_num2 + max_num3);
}

fn main() {
    day_01();
}
