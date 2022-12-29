use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    part_one();
}

fn open_file(filename: &str) -> String {
    // Open the input file
    let path: &Path = Path::new(filename);
    let display = path.display();
    let mut file: File = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(why) = file.read_to_string(&mut contents) {
        panic!("couldn't read {}: {}", display, why);
    }

    contents
}

fn parse_input() {
    let input = open_file("./sample.txt");
    let lines: Vec<&str> = input.lines().collect();
}

fn part_one() {
    println!("Hello, world!");
}

struct Monkey {

}
