fn main() -> Result<(), &'static str> {
    let contents = include_str!("input.txt");
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

    Ok(())
}
