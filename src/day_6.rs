use std::collections::HashSet;
use std::{fs, marker};

fn read_input() -> String {
    let file_path = "public/day6.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let content = read_input();

    let mut p1: usize = 0;
    let mut p2: usize = 3;

    while p2 <= content.len() {
        let protocol_string = &content[p1..=p2];
        let mut letter_hashset: HashSet<char> = HashSet::new();
        let mut marker_failed: bool = true;
        protocol_string.chars().enumerate().any(|(i, l)| {
            marker_failed = !letter_hashset.insert(l);
            marker_failed
        });
        if marker_failed {
            p1 += 1;
            p2 += 1;
        } else {
            println!("p2 found : {} after value: {}", p2 + 1, protocol_string);
            return;
        }
    }
}

pub fn question_two() {
    let content = read_input();

    let mut p1: usize = 0;
    let mut p2: usize = 13;

    while p2 <= content.len() {
        let protocol_string = &content[p1..=p2];
        let mut letter_hashset: HashSet<char> = HashSet::new();
        let mut marker_failed: bool = true;
        protocol_string.chars().enumerate().any(|(i, l)| {
            marker_failed = !letter_hashset.insert(l);
            marker_failed
        });
        if marker_failed {
            p1 += 1;
            p2 += 1;
        } else {
            println!("p2 found : {} after value: {}", p2 + 1, protocol_string);
            return;
        }
    }
}
