enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lost,
    Draw,
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

struct GameV2 {
    opponent: Hand,
    result: Result,
}

impl GameV2 {
    fn new(hand: String) -> Self {
        let values = hand.split(' ').collect::<Vec<&str>>();
        let oponent_hand = match values[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            &_ => todo!(),
        };
        let result = match values[1] {
            "X" => Result::Lost,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            &_ => todo!(),
        };
        Self {
            opponent: oponent_hand,
            result: result,
        }
    }

    fn result(&self) -> i16 {
        match self.result {
            Result::Lost => match self.opponent {
                Hand::Rock => 0 + 3,
                Hand::Paper => 0 + 1,
                Hand::Scissors => 0 + 2,
            },
            Result::Win => match self.opponent {
                Hand::Rock => 6 + 2,
                Hand::Paper => 6 + 3,
                Hand::Scissors => 6 + 1,
            },
            Result::Draw => match self.opponent {
                Hand::Rock => 3 + 1,
                Hand::Paper => 3 + 2,
                Hand::Scissors => 3 + 3,
            },
        }
    }
}

fn main() {
    let hands = include_str!("input.txt");
    part_one(hands);
    part_two(hands)
}

fn part_one(hands: &str) {
    let mut results = 0;
    for hand in hands.lines() {
        let game = Game::new(hand.to_string());
        results += game.result();
    }

    println!("{}", results);
}

fn part_two(hands: &str) {
    let mut results = 0;
    for hand in hands.lines() {
        let game = GameV2::new(hand.to_string());
        results += game.result();
    }

    println!("{}", results);
}
