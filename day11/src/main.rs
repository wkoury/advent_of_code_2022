#![warn(clippy::all)]

use std::collections::BinaryHeap;
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

fn parse_input() -> Game {
    let mut game: Game = Game::new();

    let input = open_file("./sample.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut monkey_index: isize = -1;

    let mut ii: usize = 0;

    while ii < lines.len() {
        let line = lines[ii].to_string();
        match ii % 7 {
            0 => {
                game.monkeys.push(Monkey::new());
                monkey_index += 1;
            }
            1 => {
                // Get items
                let split_by_colon: Vec<&str> = line.split(": ").collect();
                let split_by_colon_second_half = split_by_colon[1].to_string();
                let split_by_commas: Vec<&str> = split_by_colon_second_half.split(", ").collect();
                game.monkeys[monkey_index as usize].items = split_by_commas
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
            }
            2 => {
                // Derive operation
                let split_by_colon: Vec<&str> = line.split(": ").collect();
                let split_by_colon_second_half = split_by_colon[1].to_string();
                let split_by_equals: Vec<&str> = split_by_colon_second_half.split(" = ").collect();
                let split_by_equals_second_half = split_by_equals[1];
                let words = split_by_equals_second_half
                    .split(' ')
                    .collect::<Vec<&str>>();
                match words[1] {
                    "*" => {
                        game.monkeys[monkey_index as usize].operation = Operation::Multiply;
                    }
                    "+" => {
                        game.monkeys[monkey_index as usize].operation = Operation::Add;
                    }
                    _ => panic!("This should not happen"),
                }
                match words[2] {
                    "old" => {
                        game.monkeys[monkey_index as usize].operand = None;
                    }
                    _ => {
                        let number: u32 = words[2].parse().unwrap();
                        game.monkeys[monkey_index as usize].operand = Some(number);
                    }
                }
            }
            3 => {
                // Divisible by ${NUMBER}. We just want to get the number.
                let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                let number: u32 = words.last().unwrap().parse().unwrap();
                game.monkeys[monkey_index as usize].divisible_by = number;
            }
            4 => {
                // True monkey
                let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                let number: u32 = words.last().unwrap().parse().unwrap();
                game.monkeys[monkey_index as usize].true_monkey = number;
            }
            5 => {
                // False monkey
                let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                let number: u32 = words.last().unwrap().parse().unwrap();
                game.monkeys[monkey_index as usize].false_monkey = number;
            }
            6 => {
                // Do nothing
            }
            _ => panic!("This should not happen"),
        }

        ii += 1;
    }

    game
}

fn part_one() {
    let mut game: Game = parse_input();

    for _ in 0..20 {
        game.individual_monkey();
    }

    println!(
        "The monkey business after 20 rounds is {}.",
        game.get_monkey_business()
    );
}

#[derive(Debug)]
struct Game {
    cycle: usize,
    monkey_index: usize,
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            cycle: 0,
            monkey_index: 0,
            monkeys: Vec::new(),
        }
    }

    pub fn individual_monkey(&mut self) {
        let mut ii: usize = 0;
        while ii < self.monkeys[self.monkey_index].items.len() {
            self.monkeys[self.monkey_index].inspection_count += 1;
            if self.monkeys[self.monkey_index].operand.is_some() {
                match self.monkeys[self.monkey_index].operation {
                    Operation::Multiply => {
                        self.monkeys[self.monkey_index].items[ii] *=
                            self.monkeys[self.monkey_index].operand.unwrap();
                    }
                    Operation::Add => {
                        self.monkeys[self.monkey_index].items[ii] +=
                            self.monkeys[self.monkey_index].operand.unwrap();
                    }
                }
            } else {
                let old = self.monkeys[self.monkey_index].items[ii];
                match self.monkeys[self.monkey_index].operation {
                    Operation::Multiply => {
                        self.monkeys[self.monkey_index].items[ii] *= old;
                    }
                    Operation::Add => {
                        self.monkeys[self.monkey_index].items[ii] += old;
                    }
                }
            }

            self.monkeys[self.monkey_index].items[ii] /= 3;

            #[allow(unused_mut)]
            let mut new_monkey_index;
            if self.monkeys[self.monkey_index].items[ii]
                % self.monkeys[self.monkey_index].divisible_by
                == 0
            {
                new_monkey_index = self.monkeys[self.monkey_index].true_monkey;
            } else {
                new_monkey_index = self.monkeys[self.monkey_index].false_monkey;
            }
            let item_to_push = self.monkeys[self.monkey_index].items[ii];
            self.monkeys[new_monkey_index as usize]
                .items
                .push(item_to_push);
            // Remove the item from the current monkey
            self.monkeys[self.monkey_index].items.remove(ii);
            ii += 1;
        }
        self.monkey_index += 1;
        if self.monkey_index == self.monkeys.len() {
            self.monkey_index = 0;
            self.cycle += 1;
        }
    }

    pub fn get_monkey_business(&self) -> u32 {
        let mut max_heap: BinaryHeap<u32> = BinaryHeap::new();
        for monkey in &self.monkeys {
            max_heap.push(monkey.inspection_count);
        }

        let mut monkey_business: u32 = 0;

        monkey_business += max_heap.pop().unwrap();
        monkey_business *= max_heap.pop().unwrap();

        monkey_business
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    divisible_by: u32,
    true_monkey: u32,
    false_monkey: u32,
    operation: Operation,
    operand: Option<u32>,
    inspection_count: u32,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            divisible_by: 0,
            true_monkey: 0,
            false_monkey: 0,
            operation: Operation::Multiply,
            operand: None,
            inspection_count: 0,
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Multiply,
    Add,
}
