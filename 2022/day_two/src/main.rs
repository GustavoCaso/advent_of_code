enum Hand {
    Rock,
    Paper,
    Scissors,
}

struct Game {
    opponent: Hand,
    me: Hand,
}

impl Game {
    fn new(hand: String) -> Self {
        let values = hand.split(' ').collect::<Vec<&str>>();
        let oponent_hand = match values[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            &_ => todo!(),
        };
        let me_hand = match values[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            &_ => todo!(),
        };
        Self {
            opponent: oponent_hand,
            me: me_hand,
        }
    }

    fn result(&self) -> i16 {
        match self.me {
            Hand::Rock => match self.opponent {
                Hand::Rock => 3 + 1,
                Hand::Paper => 0 + 1,
                Hand::Scissors => 6 + 1,
            },
            Hand::Paper => match self.opponent {
                Hand::Rock => 6 + 2,
                Hand::Paper => 3 + 2,
                Hand::Scissors => 0 + 2,
            },
            Hand::Scissors => match self.opponent {
                Hand::Rock => 0 + 3,
                Hand::Paper => 6 + 3,
                Hand::Scissors => 3 + 3,
            },
        }
    }
}

fn main() {
    let mut results = 0;
    let hands = include_str!("input.txt");
    for hand in hands.lines() {
        let game = Game::new(hand.to_string());
        results += game.result();
    }

    println!("{}", results);
}
