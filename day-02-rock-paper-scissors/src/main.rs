// https://adventofcode.com/2022/day/2

use std::{fs, str::FromStr};

const INPUT_FILE: &str = "input.txt";

#[cfg(windows)]
const LINE_SEPARATOR: &str = "\r\n";
#[cfg(not(windows))]
const LINE_SEPARATOR: &str = "\n";

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(ParseMoveError),
        }
    }
}

impl Move {
    fn against(self: &Self, opponent: &Move) -> RoundOutcome {
        if *self == *opponent {
            RoundOutcome::Draw
        } else if *self == Move::Rock && *opponent == Move::Scissors {
            RoundOutcome::Win
        } else if *self == Move::Paper && *opponent == Move::Rock {
            RoundOutcome::Win
        } else if *self == Move::Scissors && *opponent == Move::Paper {
            RoundOutcome::Win
        } else {
            RoundOutcome::Lost
        }
    }
}

#[derive(Debug, PartialEq)]
enum RoundOutcome {
    Lost,
    Draw,
    Win,
}

#[derive(Debug)]
struct ParseRoundOutcomeError;

impl FromStr for RoundOutcome {
    type Err = ParseRoundOutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundOutcome::Lost),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            _ => Err(ParseRoundOutcomeError),
        }
    }
}

const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect(&format!("Cannot read {INPUT_FILE} file"));
    let games = input.split(LINE_SEPARATOR);
    let total_score = games
        .map(|game| {
            let game: Vec<_> = game.split(" ").collect();
            let opponent_move = game
                .get(0)
                .expect("Cannot find opponent move")
                .parse::<Move>()
                .expect("Cannot parse opponent move");
            let outcome = game
                .get(1)
                .expect("Cannot round outcome")
                .parse::<RoundOutcome>()
                .expect("Cannot parse round outcome");
            let player_move = ALL_MOVES
                .iter()
                .find(|&m| m.against(&opponent_move) == outcome)
                .expect("Cannot determine player move");

            return match player_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
            } + match outcome {
                RoundOutcome::Win => 6,
                RoundOutcome::Lost => 0,
                RoundOutcome::Draw => 3,
            };
        })
        .sum::<i32>();

    println!("My total score is {total_score}");
}

#[cfg(test)]
mod tests {
    use crate::{Move, RoundOutcome};

    #[test]
    fn rock_against_rock_should_outcome_draw() {
        assert_eq!(Move::Rock.against(&Move::Rock), RoundOutcome::Draw);
    }

    #[test]
    fn rock_against_paper_should_outcome_lost() {
        assert_eq!(Move::Rock.against(&Move::Paper), RoundOutcome::Lost);
    }

    #[test]
    fn rock_against_scissors_should_outcome_win() {
        assert_eq!(Move::Rock.against(&Move::Scissors), RoundOutcome::Win);
    }

    #[test]
    fn paper_against_rock_should_outcome_win() {
        assert_eq!(Move::Paper.against(&Move::Rock), RoundOutcome::Win);
    }

    #[test]
    fn paper_against_paper_should_outcome_draw() {
        assert_eq!(Move::Paper.against(&Move::Paper), RoundOutcome::Draw);
    }

    #[test]
    fn paper_against_scissors_should_outcome_lost() {
        assert_eq!(Move::Paper.against(&Move::Scissors), RoundOutcome::Lost);
    }

    #[test]
    fn scissors_against_rock_should_outcome_lost() {
        assert_eq!(Move::Scissors.against(&Move::Rock), RoundOutcome::Lost);
    }

    #[test]
    fn scissors_against_paper_should_outcome_win() {
        assert_eq!(Move::Scissors.against(&Move::Paper), RoundOutcome::Win);
    }

    #[test]
    fn scissors_against_scissors_should_outcome_draw() {
        assert_eq!(Move::Scissors.against(&Move::Scissors), RoundOutcome::Draw);
    }
}
