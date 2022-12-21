use std::env;
use std::fs;

fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/input.txt");

    let contents = fs::read_to_string(current_dir.as_path()).expect("Fail to read input.txt");
    max_value(contents.clone());
    top_three(contents.clone());
}

fn max_value(s: String) {
    let mut current: i32 = 0;
    let mut max = 0;
    for line in s.lines() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }
    println!("The max value is {}", max);
}

fn top_three(s: String) {
    let mut current: i32 = 0;
    let mut total_items: Vec<i32> = Vec::new();
    for line in s.lines() {
        if line == "" {
            total_items.push(current);
            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }
    total_items.sort_by(|a, b| b.cmp(a));
    let first = total_items[0];
    let second = total_items[1];
    let third = total_items[2];
    println!("The first one is {}", first);
    println!("The second one is {}", second);
    println!("The third one is {}", third);
    println!("The total value is {}", first + second + third);
}
