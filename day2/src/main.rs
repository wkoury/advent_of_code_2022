#![warn(clippy::all)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Open the input file
    let path: &Path = Path::new("./input.txt");
    let display = path.display();
    let mut file: File = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(why) = file.read_to_string(&mut contents) {
        panic!("couldn't read {}: {}", display, why);
    }

    let mut my_total_points: u32 = 0;

    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let game: Game = Game::new(
            parts[0].chars().nth(0).unwrap(),
            parts[1].chars().nth(0).unwrap(),
        );

        my_total_points += game.compute_my_game_points();
    }

    dbg!(my_total_points);
}

pub struct Game {
    opponent_choice: Choice,
    my_choice: Choice,
}

impl Game {
    pub fn new(opponent_choice: char, my_choice: char) -> Game {
        Game {
            opponent_choice: convert_opponent_code_to_choice(opponent_choice),
            my_choice: convert_my_code_to_choice(my_choice),
        }
    }

    pub fn compute_my_game_points(self) -> u32 {
        let mut ret: u32 = 0;

        ret += match self.my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        let result: Result = self.compute_game_result();

        ret += match result {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Draw => 3,
        };

        ret
    }

    pub fn compute_game_result(self) -> Result {
        let mut ret: Result = Result::Draw;

        if self.my_choice == self.opponent_choice {
            ret = Result::Draw;
        } else if self.my_choice == Choice::Rock && self.opponent_choice == Choice::Scissors {
            ret = Result::Win;
        } else if self.my_choice == Choice::Paper && self.opponent_choice == Choice::Rock {
            ret = Result::Win;
        } else if self.my_choice == Choice::Scissors && self.opponent_choice == Choice::Paper {
            ret = Result::Win;
        } else {
            ret = Result::Lose;
        }

        ret
    }
}

#[derive(PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub fn convert_opponent_code_to_choice(code: char) -> Choice {
    match code {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => panic!("Invalid code"),
    }
}

pub fn convert_my_code_to_choice(code: char) -> Choice {
    match code {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scissors,
        _ => panic!("Invalid code"),
    }
}

pub enum Result {
    Win,
    Lose,
    Draw,
}
