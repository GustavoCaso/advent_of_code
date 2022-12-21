fn main() -> Result<(), &'static str> {
    let contents = include_str!("input.txt");

    let mut groups: Vec<i32> = Vec::new();
    for group in contents.split("\n\n") {
        let mut current: i32 = 0;
        for number in group.lines().map(|v| v.parse::<i32>().unwrap()) {
            current += number;
        }
        groups.push(current);
    }

    groups.sort_by(|a, b| b.cmp(a));

    let max = groups[0];
    let mut top_three = 0;

    top_three += max;
    for i in 1..3 {
        top_three += groups[i];
    }
    println!("Max {}", max);
    println!("Top Three {}", top_three);

    Ok(())
}
