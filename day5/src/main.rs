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

    // I hate the idea of parsing this. I'm tired. I'm doing it manually.
    let stack1: Vec<char> = vec!['R', 'G', 'H', 'Q', 'S', 'B', 'T', 'N'];
    let stack2: Vec<char> = vec!['H', 'S', 'F', 'D', 'P', 'Z', 'J'];
    let stack3: Vec<char> = vec!['Z', 'H', 'V'];
    let stack4: Vec<char> = vec!['M', 'Z', 'J', 'F', 'G', 'H'];
    let stack5: Vec<char> = vec!['T', 'Z', 'C', 'D', 'L', 'M', 'S', 'R'];
    let stack6: Vec<char> = vec!['M', 'T', 'W', 'V', 'H', 'Z', 'J'];
    let stack7: Vec<char> = vec!['T', 'F', 'P', 'L', 'Z'];
    let stack8: Vec<char> = vec!['Q', 'V', 'W', 'S'];
    let stack9: Vec<char> = vec!['W', 'H', 'L', 'M', 'T', 'D', 'N', 'C'];

    let mut stacks: Vec<Vec<char>> = vec![
        stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9,
    ];

    let lines_length = contents.split('\n').count();
    let lines = contents.lines();
    let mut ii: usize = 10;

    while ii < lines_length - 1 {
        // Split the line into words
        let words = lines
            .clone()
            .nth(ii)
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>();

        let amount_to_move: usize = words[1].parse().unwrap();
        let raw_from_stack_index: usize = words[3].parse().unwrap();
        let raw_to_stack_index: usize = words[5].parse().unwrap();
        let from_stack_index: usize = raw_from_stack_index - 1;
        let to_stack_index: usize = raw_to_stack_index - 1;

        for _ in 0..amount_to_move {
            let tmp = stacks[from_stack_index].pop();
            if let Some(..) = tmp {
                stacks[to_stack_index].push(tmp.unwrap());
            }
        }

        ii += 1;
    }

    // Now let's get the top of each stack
    for stack in stacks {
        print!("{}", stack.clone().pop().unwrap());
    }
    println!();
}
