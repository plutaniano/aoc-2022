use core::panic;

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from(letter: char) -> Self {
        match letter {
            'X' | 'A' => Play::Rock,
            'Y' | 'B' => Play::Paper,
            'Z' | 'C' => Play::Scissors,
            _ => panic!("Invalid play: {}", letter),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    fn play_to(self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Draw => self,
            Outcome::Lose => self.wins_against(),
            Outcome::Win => self.loses_to(),
        }
    }
}
enum Outcome {
    Draw,
    Lose,
    Win,
}

impl Outcome {
    fn from(letter: char) -> Outcome {
        match letter {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid outcome: {}", letter),
        }
    }

    fn score(self) -> u32 {
        match self {
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::Win => 6,
        }
    }
}

struct Game {}
impl Game {
    fn chars(line: &str) -> (char, char) {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        chars.next();
        let b = chars.next().unwrap();
        (a, b)
    }

    fn as_plays(line: &str) -> (Play, Play) {
        let (p1, p2) = Game::chars(line);
        (Play::from(p1), Play::from(p2))
    }

    fn as_outcome(line: &str) -> (Play, Outcome) {
        let (play, outcome) = Game::chars(line);
        (Play::from(play), Outcome::from(outcome))
    }

    fn battle(p1: Play, p2: Play) -> u32 {
        use Play::{Paper, Rock, Scissors};
        match (p1, p2) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Scissors) => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.split('\n') {
        let (a, b) = Game::as_plays(line);
        score += b.score();
        score += Game::battle(a, b);
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (opponent_play, outcome) = Game::as_outcome(line);
        let my_play = opponent_play.play_to(&outcome);
        score += my_play.score();
        score += outcome.score();
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
