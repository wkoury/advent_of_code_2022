use std::collections::BTreeMap;
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

    dbg!(&contents);

    let lines_count = contents.split('\n').count();
    let lines = contents.lines();
    let mut ii: usize = 1;
    let mut dirs: BTreeMap<String, u32> = BTreeMap::new();
    let mut working_dir: Vec<String> = vec![String::from("~")];

    while ii < lines_count {
        let line = lines.clone().nth(ii).unwrap();
        let words = line.split(' ').collect::<Vec<&str>>();
        let working_directory = working_dir.join("/");

        dbg!(&line);
        dbg!(&working_dir);
        dbg!(&working_directory);

        if words.len() == 3 && words[0] == "$" && words[1] == "cd" {
            if words[2] != ".." {
                let path = words[2];
                working_dir.push(path.to_string());
            } else {
                working_dir.pop();
            }
        } else if words[0] == "dir" {
            let new_directory = format!("{}/{}", working_dir.join("/"), words[1]);
            dirs.insert(new_directory, 0);
        } else if words[0] != "$" {
            let file_size_to_add: u32 = words[0].parse().unwrap();

            let current_directory_size = dirs.get(&working_directory);
            if let Some(..) = current_directory_size {
                let new_size = current_directory_size.unwrap() + file_size_to_add;
                dirs.insert(working_directory, new_size);
            } else {
                dirs.insert(working_directory, file_size_to_add);
            }
        }

        ii += 1;
    }

    dbg!(&dirs);

    let computed_dirs: BTreeMap<String, u32> = dirs
        .iter()
        .map(|(k, _v)| (k.clone(), get_full_directory_size(dirs.clone(), k.clone())))
        .collect();

    dbg!(&computed_dirs);

    let mut part_1_sum: u32 = 0;

    for dir in computed_dirs {
        if dir.1 < 100000 {
            part_1_sum += dir.1;
        }
    }

    println!(
        "The sum of the directories with less than 100000 bytes is {}",
        part_1_sum
    );
}

fn get_full_directory_size(dirs: BTreeMap<String, u32>, path: String) -> u32 {
    let mut ret: u32 = 0;

    for dir in dirs {
        if dir.0.contains(&path) {
            ret += dir.1;
        }
    }

    ret
}
