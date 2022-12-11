use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod rope;

use rope::Direction;
use rope::Rope;

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

    let mut rope: Rope = Rope::new();

    dbg!(&rope);

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        dbg!(&words);
        let direction_char: char = words[0].chars().next().unwrap();
        let direction: Direction = match direction_char {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Invalid direction: {}", direction_char),
        };

        // This distance is a magnitude, so stay as a u32 and we'll convert in the move_tail function
        let magnitude: u32 = words[1].parse().unwrap();

        rope.move_tail(direction, magnitude);
    }

    let part_1: usize = rope.get_visited_positions_count();
    println!(
        "The tail has visited a total of {} unique positions.",
        part_1
    );
}
