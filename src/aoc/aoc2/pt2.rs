use std::fs;
use crate::aoc::{aoc2::{Outcome, Shape}, LINE_ENDING};

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => { Self::LOST }
            "Y" => { Self::DRAW }
            "Z" => { Self::WON }
            _ => { Self::LOST } // I know i should use None for this case but kinda lazy rn
        }
    }
}

pub fn jo_ken_po_pt2() {
    let mut total_score: usize = 0;

    if let Ok(file) = fs::read_to_string("src/aoc/aoc2/input") {
        file.split(LINE_ENDING).for_each(|line| {
            let moves: Vec<&str> = line.split(' ').collect();
            if moves.len() > 1 {
                let opponent: Shape = moves[0].into();
                let wtd_outcome: Outcome = moves[1].into();
                let me: Shape = needed_shape(&opponent, &wtd_outcome);
                total_score += me.score();
                total_score += me.outcome_vs(&opponent).score();
            }
        });
    } else {
        panic!("File not found")
    }

    println!("AOC2 - PT2: {}", total_score);
}

fn needed_shape(opponent: &Shape, wtd_outcome: &Outcome) -> Shape {
    match opponent {
        Shape::ROCK => {
            match wtd_outcome {
                Outcome::LOST => { Shape::SCISSORS }
                Outcome::WON => { Shape::PAPER }
                Outcome::DRAW => { Shape::ROCK }
            }
        }
        Shape::PAPER => {
            match wtd_outcome {
                Outcome::LOST => { Shape::ROCK }
                Outcome::WON => { Shape::SCISSORS }
                Outcome::DRAW => { Shape::PAPER }
            }
        }
        Shape::SCISSORS => {
            match wtd_outcome {
                Outcome::LOST => { Shape::PAPER }
                Outcome::WON => { Shape::ROCK }
                Outcome::DRAW => { Shape::SCISSORS }
            }
        }
    }
}
