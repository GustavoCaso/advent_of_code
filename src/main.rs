use std::env;
use std::fs;

fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/input.txt");

    let contents = fs::read_to_string(current_dir.as_path()).expect("Fail to read input.txt");
    let mut current: i32 = 0;
    let mut total_items: Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line == "" {
            total_items.push(current);
            current = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            current += number;
        }
    }
    total_items.sort_by(|a, b| b.cmp(a));
    let mut max = 0;
    let mut top_three = 0;
    for i in 0..3 {
        let item = total_items[i];
        if i == 0 {
            max = item;
        }
        top_three += item;
    }
    println!("Max {}", max);
    println!("Top Three {}", top_three);
}
