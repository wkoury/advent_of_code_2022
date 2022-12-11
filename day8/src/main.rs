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

    let length = contents.lines().count();

    // Remove all of the whitespace from contents
    contents.retain(|c| !c.is_whitespace());

    let mut ii: usize = 0;

    let mut visible_trees_count: u32 = 0;

    while ii < length * length {
        let tree_height_char = contents.chars().nth(ii).unwrap();
        let tree_height = tree_height_char.to_digit(10).unwrap();
        let x = (ii % length) as i32;
        let y = (ii / length) as i32;

        let mut visible_from_right = true;
        let mut visible_from_left = true;
        let mut visible_from_bottom = true;
        let mut visible_from_top = true;

        let mut jj: i32 = ii as i32 + 1;
        while jj < length as i32 * (y + 1) {
            let other_tree_height = contents
                .chars()
                .nth(jj as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
            if tree_height <= other_tree_height {
                visible_from_right = false;
                break;
            }

            jj += 1;
        }
        jj = ii as i32 - 1;
        while jj >= length as i32 * y {
            let other_tree_height = contents
                .chars()
                .nth(jj as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
            if tree_height <= other_tree_height {
                visible_from_left = false;
                break;
            }

            jj -= 1;
        }

        jj = ii as i32 + length as i32;
        while jj < length as i32 * length as i32 {
            let other_tree_height = contents
                .chars()
                .nth(jj as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
            if tree_height <= other_tree_height {
                visible_from_bottom = false;
                break;
            }

            jj += length as i32;
        }

        jj = ii as i32 - length as i32;
        while jj >= 0 {
            let other_tree_height = contents
                .chars()
                .nth(jj as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
            if tree_height <= other_tree_height {
                visible_from_top = false;
                break;
            }

            jj -= length as i32;
        }

        // Check if a tree is at the edge of the grid
        if x == 0
            || y == 0
            || x == length as i32 - 1
            || y == length as i32 - 1
            || visible_from_bottom
            || visible_from_left
            || visible_from_right
            || visible_from_top
        {
            visible_trees_count += 1;
        }

        ii += 1;
    }

    println!("Visible trees: {}", visible_trees_count);
}
