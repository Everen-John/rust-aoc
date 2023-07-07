use std::fs::{self};

fn read_input() -> String {
    let file_path = "public/day2.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {}
