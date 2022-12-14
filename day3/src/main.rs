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

    let mut sum: u32 = 0;

    let lines_length = contents.split('\n').count();
    let lines = contents.lines();
    let mut ii: usize = 0;

    let mut similar_char: char = '0';

    while ii < lines_length - 1 {
        // Get the line and the following 2 lines
        let first_line = lines.clone().nth(ii).unwrap();
        let second_line = lines.clone().nth(ii + 1).unwrap();
        let third_line = lines.clone().nth(ii + 2).unwrap();

        for char1 in first_line.chars() {
            for char2 in second_line.chars() {
                for char3 in third_line.chars() {
                    if char1 == char2 && char2 == char3 {
                        similar_char = char1;
                    }
                }
            }
        }

        sum += get_priority_from_char(similar_char);

        ii += 3;
    }

    println!("The sum of the priorities is: {}", sum);
}

fn get_priority_from_char(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
