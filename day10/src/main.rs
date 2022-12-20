use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    part_one();
}

fn part_one() {
    let contents: String = open_file("./input.txt");

    let mut cpu: Cpu = Cpu::new();

    for line in contents.lines() {
        cpu.parse_operation(line);
    }

    println!("Part one: {}", cpu.part_one_sum);
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

struct Cpu {
    x: i32,
    cycle: u32,
    part_one_sum: u32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            part_one_sum: 0,
        }
    }

    fn parse_operation(&mut self, operation: &str) {
        let mut parts = operation.split_whitespace();
        let op = parts.next().unwrap();
        let value = parts.next();

        match op {
            "addx" => self.update_register(value.unwrap().parse::<i32>().unwrap()),
            "noop" => self.no_op(),
            _ => panic!("Unknown operation: {}", op),
        }
    }

    fn update_register(&mut self, value: i32) {
        for _ in 0..2 {
            self.step_cycle();
        }
        self.x += value;
    }

    fn no_op(&mut self) {
        self.step_cycle();
    }

    fn step_cycle(&mut self) {
        self.cycle += 1;
        if self.part_one_cpu_cycles() {
            self.part_one_sum += self.get_signal_strength();
        }
    }

    fn get_signal_strength(&self) -> u32 {
        self.x as u32 * self.cycle
    }

    fn part_one_cpu_cycles(&self) -> bool {
        self.cycle == 20
            || self.cycle == 60
            || self.cycle == 100
            || self.cycle == 140
            || self.cycle == 180
            || self.cycle == 220
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let contents: String = open_file("./sample.txt");

        let mut cpu: Cpu = Cpu::new();

        for line in contents.lines() {
            cpu.parse_operation(line);
        }

        assert_eq!(cpu.part_one_sum, 13140);
    }
}
