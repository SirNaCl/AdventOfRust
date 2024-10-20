use std::str::FromStr;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug, PartialEq)]
#[repr(i32)]
enum Choice {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
    // END = 4,
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" => Ok(Choice::ROCK),
            "B" => Ok(Choice::PAPER),
            "C" => Ok(Choice::SCISSORS),
            "X" => Ok(Choice::ROCK),
            "Y" => Ok(Choice::PAPER),
            "Z" => Ok(Choice::SCISSORS),
            _ => Err(()),
        }
    }
}

impl Clone for Choice {
    fn clone(&self) -> Choice {
        match self {
            Choice::ROCK => Choice::ROCK,
            Choice::PAPER => Choice::PAPER,
            Choice::SCISSORS => Choice::SCISSORS,
            // Choice::END => Choice::END,
        }
    }
}

pub struct Game {
    opponent: Choice,
    me: Choice,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let s: Vec<&str> = input.split(" ").take(2).collect();
        Ok(Game {
            opponent: Choice::from_str(s[0]).unwrap(),
            me: Choice::from_str(s[1]).unwrap(),
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| Game::from_str(l))
        .map(|g| g.unwrap())
        .collect()
}

fn points(game: &Game) -> i32 {
    let win = game.me.clone() as i32 + 6;
    let tie = game.me.clone() as i32 + 3;
    let loss = game.me.clone() as i32;
    match game.me {
        Choice::ROCK => match game.opponent {
            Choice::ROCK => tie,
            Choice::PAPER => loss,
            Choice::SCISSORS => win,
        },
        Choice::PAPER => match game.opponent {
            Choice::ROCK => win,
            Choice::PAPER => tie,
            Choice::SCISSORS => loss,
        },
        Choice::SCISSORS => match game.opponent {
            Choice::ROCK => loss,
            Choice::PAPER => win,
            Choice::SCISSORS => tie,
        },
    }
}

fn points2(game: &Game) -> i32 {
    match game.me {
        // Lose
        Choice::ROCK => match game.opponent {
            Choice::ROCK => Choice::SCISSORS as i32,
            Choice::PAPER => Choice::ROCK as i32,
            Choice::SCISSORS => Choice::PAPER as i32,
        },
        // Tie
        Choice::PAPER => match game.opponent {
            Choice::ROCK => Choice::ROCK as i32 + 3,
            Choice::PAPER => Choice::PAPER as i32 + 3,
            Choice::SCISSORS => Choice::SCISSORS as i32 + 3,
        },
        // Win
        Choice::SCISSORS => match game.opponent {
            Choice::ROCK => Choice::PAPER as i32 + 6,
            Choice::PAPER => Choice::SCISSORS as i32 + 6,
            Choice::SCISSORS => Choice::ROCK as i32 + 6,
        },
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> i32 {
    input.iter().map(points).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> i32 {
    // For the second part, the "me" field represents
    // the desired outcome
    // Rock->lose
    // Paper->draw
    // scissors->win
    input.iter().map(points2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(solve_part1(&input_generator("A Y\nB X\nC Z")), 15)
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part2(&input_generator("A Y\nB X\nC Z")), 12)
    }
}
