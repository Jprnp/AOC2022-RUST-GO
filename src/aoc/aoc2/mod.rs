use std::fs;

use crate::aoc::LINE_ENDING;

pub mod pt2;

enum Outcome {
    LOST,
    WON,
    DRAW
}

impl Outcome {
    pub fn score(&self) -> usize {
        match self {
            Self::LOST => { 0 }
            Self::WON => { 6 }
            Self::DRAW => { 3 }
        }
    }
}

#[derive(PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS
}

impl Shape {
    pub fn score(&self) -> usize {
        match self {
            Self::ROCK => { 1 }
            Self::PAPER => { 2 }
            Self::SCISSORS => { 3 }
        }
    }

    pub fn outcome_vs(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::DRAW;
        }

        match self {
            Self::ROCK => { if other == &Self::SCISSORS { Outcome::WON } else { Outcome::LOST } }
            Self::PAPER => { if other == &Self::ROCK { Outcome::WON } else { Outcome::LOST } }
            Self::SCISSORS => { if other == &Self::PAPER { Outcome::WON } else { Outcome::LOST } }
        }
    }
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A"|"X" => { Self::ROCK }
            "B"|"Y" => { Self::PAPER }
            "C"|"Z" => { Self::SCISSORS }
            _ => { Self::ROCK } // I know i should use None for this case but kinda lazy rn
        }
    }
}

pub fn jo_ken_po() {
    let mut total_score: usize = 0;

    if let Ok(file) = fs::read_to_string("src/aoc/aoc2/input") {
        file.split(LINE_ENDING).for_each(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            if moves.len() > 1 {
                let opponent: Shape = moves[0].into();
                let me: Shape = moves[1].into();
                total_score += me.score();
                total_score += me.outcome_vs(&opponent).score();
            }
        });
    } else {
        panic!("File not found")
    }

    println!("AOC2 - PT1: {}", total_score);
}
