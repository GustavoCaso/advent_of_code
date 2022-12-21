fn main() -> Result<(), &'static str> {
    let contents = include_str!("input.txt");
    let mut max = 0;
    let mut total_items: Vec<i32> = Vec::new();

    for group in contents.split("\n\n") {
        let mut current: i32 = 0;
        for number in group.lines().map(|v| v.parse::<i32>().unwrap()) {
            current += number;
        }
        total_items.push(current);
    }

    total_items.sort_by(|a, b| b.cmp(a));
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
