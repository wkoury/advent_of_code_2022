#![warn(clippy::all)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Hello, world!");

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

    let mut number_of_pairs: u32 = 0;

    for line in contents.lines() {
        let pair = AssignmentPair::new(line.to_string());

        if pair.range_contains_range() {
            number_of_pairs += 1;
        }
    }

    println!(
        "There are {} pairs where one range fully contains the other.",
        number_of_pairs
    );
}

#[derive(Debug)]
struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

impl AssignmentPair {
    fn new(input: String) -> AssignmentPair {
        // Split the input by comma
        let split: Vec<&str> = input.split(',').collect();
        // Get left from split[0]

        let binding = split[0].to_string();
        let left_vec: Vec<&str> = binding.split('-').collect();
        let left = Assignment::new(
            left_vec[0].to_string().parse().unwrap(),
            left_vec[1].to_string().parse().unwrap(),
        );

        let binding = split[1].to_string();
        let right_vec: Vec<&str> = binding.split('-').collect();
        let right = Assignment::new(
            right_vec[0].to_string().parse().unwrap(),
            right_vec[1].to_string().parse().unwrap(),
        );

        AssignmentPair { left, right }
    }

    fn range_contains_range(self) -> bool {
        (self.left.start >= self.right.start && self.left.end <= self.right.end)
            || (self.right.start >= self.left.start && self.right.end <= self.left.end)
    }
}

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Assignment {
        Assignment { start, end }
    }
}
