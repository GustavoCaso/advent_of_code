use core::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Lost,
    Draw,
}

impl GameResult {
    fn inherent_points(self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lost => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn inherent_points(self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(self, opponent: Hand) -> bool {
        matches!(
            (self, opponent),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, opponent: Hand) -> GameResult {
        if self.beats(opponent) {
            return GameResult::Win;
        } else if opponent.beats(self) {
            return GameResult::Lost;
        } else {
            GameResult::Draw
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl TryFrom<char> for Hand {
    type Error = ParseHandError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(ParseHandError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    opponent: Hand,
    me: Hand,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(opponent), Some(' '), Some(me), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(ParseHandError)
        };

        Ok(Self {
            opponent: opponent.try_into()?,
            me: me.try_into()?,
        })
    }
}

impl Game {
    fn outcome(self) -> GameResult {
        self.me.outcome(self.opponent)
    }

    fn our_score(self) -> usize {
        self.me.inherent_points() + self.outcome().inherent_points()
    }
}

fn main() -> Result<(), ParseHandError> {
    let mut total_score = 0;
    for game in include_str!("input.txt").lines().map(Game::from_str) {
        total_score += game?.our_score();
    }

    println!("{}", total_score);
    Ok(())
}
