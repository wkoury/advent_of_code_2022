use std::collections::BinaryHeap;
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

    let mut max_heap: BinaryHeap<u32> = BinaryHeap::<u32>::new();
    let mut count: u32 = 0;

    // Split the string into lines
    for line in contents.lines() {
        if line.eq("") {
            max_heap.push(count);
            count = 0;
        } else {
            // Get the u32 out of the string and add it to count
            let add_me: u32 = line.parse().unwrap();
            count += add_me;
        }
    }

    // Get the max value
    println!("The elf carrying the most calories is carrying {} calories.", max_heap.peek().unwrap());

    let mut top_three_elves_total: u32 = 0;
    for _ in 1..4 {
        top_three_elves_total += max_heap.pop().unwrap();
    }

    println!("The top three elves are carrying, in total, {} calories.", top_three_elves_total);
}
