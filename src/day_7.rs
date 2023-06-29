use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs};

fn read_input() -> String {
    let file_path = "public/day7.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}
lazy_static! {
    static ref CD_PATTERN: Regex = Regex::new(r"\$\s(cd)\s(\w+|\.\.|/)").unwrap();
    static ref FILESIZE_PATTERN: Regex = Regex::new(r"(\d+)\s(.*)").unwrap();
}

pub fn question_one() {
    let contents = read_input();
    let mut directory_stack: Vec<String> = Vec::new();
    let mut directory_size_map: HashMap<String, u128> = HashMap::new();

    contents.split('\n').for_each(|instruction| {
        if CD_PATTERN.is_match(instruction) {
            handle_cd(instruction, &mut directory_stack);
        } else if FILESIZE_PATTERN.is_match(instruction) {
            handle_filesize(instruction, &mut directory_stack, &mut directory_size_map);
        }
    });

    let mut total_size = 0;
    for key in directory_size_map.keys() {
        let size = directory_size_map.get(key).unwrap();

        if directory_size_map.get(key).unwrap() <= &100000 {
            total_size += size;
            println!("Added dir {} with size {}", key, size);
            println!("CUMULATED SIZE: {}", total_size);
        }
    }
    println!("total size: {}", total_size);
}

fn handle_cd(instruction: &str, directory_stack: &mut Vec<String>) {
    let caps = CD_PATTERN.captures(instruction).unwrap();
    let dir_name = caps.get(2).unwrap().as_str().to_string();

    if dir_name == ".." {
        directory_stack.pop();
    } else if dir_name == "/" {
        directory_stack.clear();
        directory_stack.push("/".to_string());
    } else {
        directory_stack.push(dir_name);
    }
}

fn handle_filesize(
    instruction: &str,
    directory_stack: &mut [String],
    directory_size_map: &mut HashMap<String, u128>,
) {
    let caps = FILESIZE_PATTERN.captures(instruction).unwrap();
    let new_filesize: u128 = caps.get(1).unwrap().as_str().parse().unwrap();

    for (i, directory) in directory_stack.iter().enumerate() {
        let dir_name = directory_stack[0..i].join("/");
        match directory_size_map.get(directory) {
            Some(existing_filesize) => {
                directory_size_map.insert(dir_name, existing_filesize + new_filesize);
            }
            None => {
                directory_size_map.insert(dir_name, new_filesize);
            }
        }
    }
}

// use std::iter::Peekable;

// pub fn question_one() {
//     let (d, mut s) = (include_bytes!("../public/day7.txt"), 0);
//     sh(&mut d.split(|b| b == &b'\n').peekable(), &mut s);
//     println!("{}", s);
// }

// fn sh(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>, sum: &mut u64) -> u64 {
//     let mut size = 0;
//     while let Some(i) = lines.next() {
//         match i {
//             b"$ cd .." => break,
//             _ if &i[0..3] == b"$ l" => {
//                 size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
//                     .filter(|i| i[0] != b'd')
//                     .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
//                     .sum()
//             }
//             _ => size += sh(lines, sum),
//         }
//     }
//     if size <= 100_000 {
//         *sum += size;
//     }
//     size
// }

//1367870
