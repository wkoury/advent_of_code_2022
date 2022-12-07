#![warn(clippy::all)]

use std::collections::HashSet;
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

    let line = contents.lines().nth(0).unwrap();
    let mut done: bool = false;

    let mut ii: usize = 0;

    while ii < line.len() && !done {
        let substr = &line[ii..ii + 4];
        dbg!(substr);

        if all_characters_are_different(substr) {
            done = true;
        }

        ii += 1;
    }

    println!(
        "{} characters need to be processed before the first start-of-packet marker is detected.",
        ii + 3
    );
}

fn all_characters_are_different(str: &str) -> bool {
    let mut set: HashSet<char> = HashSet::<char>::new();

    for char in str.chars() {
        set.insert(char);
    }

    set.len() == 4
}
