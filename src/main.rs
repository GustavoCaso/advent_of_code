use std::env;
use std::fs;

struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

impl std::fmt::Debug for PathedIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for file {:?}: {}", self.path, self.inner)
    }
}

fn main() -> Result<(), &'static str> {
    let contents = read_input().unwrap();
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

fn read_input() -> Result<String, PathedIoError> {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("src/input.txt");

    match fs::read_to_string(current_dir.as_path()) {
        Ok(s) => Ok(s),
        Err(e) => Err(PathedIoError {
            path: "src/input.txt".to_string(),
            inner: e,
        }),
    }
}
