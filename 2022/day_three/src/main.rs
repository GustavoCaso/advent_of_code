use std::assert_eq;

struct Priority;

impl Priority {
    fn value(c: char) -> u32 {
        match c {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 14,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => todo!(),
        }
    }
}

fn main() {
    let mut total: u32 = 0;
    let content = include_str!("input.txt");
    let mut total_matches = 0;
    for line in content.lines() {
        let (first_part, second_part) = line.split_at(line.len() / 2);
        assert_eq!(first_part.len(), second_part.len());

        for c in first_part.chars() {
            if second_part.contains(c) {
                total_matches += 1;
                println!(
                    "{}, {}, {}, {}, {}",
                    total_matches,
                    first_part,
                    second_part,
                    c,
                    Priority::value(c)
                );
                total += Priority::value(c);
                break;
            }
        }
    }
    println!("{}", total);
}
