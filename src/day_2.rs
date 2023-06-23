use std::collections::HashMap;
use std::fs;

fn read_input() -> String {
    let file_path = "public/day2.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("AY", 6);
    scores.insert("BZ", 6);
    scores.insert("CX", 6);
    scores.insert("AX", 3);
    scores.insert("BY", 3);
    scores.insert("CZ", 3);

    // let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);
    scores.insert("C", 3);
    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let content = read_input();
    let rounds: Vec<&str> = content.split("\n").collect();

    let mut total_score = 0;
    for round in rounds {
        let actions = round.replace(" ", "");
        let dual_action = actions.as_str();
        let your_action: Vec<&str> = round.split(" ").collect();

        total_score +=
            *scores.get(dual_action).get_or_insert(&0) + scores.get(your_action[1]).unwrap();
    }
    println!("score: {}", total_score)
}

pub fn question_two() {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("AX", 3); //ROCK AND LOSE: 3 + 0 = 3
    scores.insert("AY", 4); //ROCK AND DRAW: 1 + 3 = 4
    scores.insert("AZ", 8); //ROCK AND WIN: 2 + 6 = 8
    scores.insert("BX", 1); //PAPER AND LOSE: 1 + 0 = 1
    scores.insert("BY", 5); //PAPER AND DRAW: 2 + 3 = 5
    scores.insert("BZ", 9); //PAPER AND WIN: 3 + 6 = 9
    scores.insert("CX", 2); //SCISSORS AND LOSE: 2 + 0 = 2
    scores.insert("CY", 6); //SCISSORS AND DRAW: 3 + 3 = 6
    scores.insert("CZ", 7); //SCISSORS AND WIN: 1 + 6 = 7

    let content = read_input();
    let rounds: Vec<&str> = content.split("\n").collect();

    let mut total_score = 0;
    for round in rounds {
        let actions = round.replace(" ", "");

        total_score += *scores.get(actions.as_str()).get_or_insert(&0);
    }
    println!("score: {}", total_score)
}
