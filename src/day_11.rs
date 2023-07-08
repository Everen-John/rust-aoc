use std::fs::{self, read};

use regex::Regex;

fn read_input() -> String {
    let file_path = "public/day11.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

struct Monkey {
    items: Vec<Item>,
    total_items_inspected: u64,
    operation: Box<dyn Fn(u32) -> u32>,
    subject: i32,
}

impl Monkey {}

#[derive(Debug)]
struct Item {
    worry_level: i32,
}

pub fn question_one() {
    let binding = read_input();
    let content: Vec<&str> = binding.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Get each monkey's starts first
    for monkey_content in content {
        let mut item_vec: Vec<Item> = Vec::new();
        let mut op: Box<dyn Fn(u32) -> u32>;

        let starting_item_line = Regex::new(r"(Starting items:.*)").unwrap();
        let starting_item_line = starting_item_line
            .captures(monkey_content)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let starting_item_pattern = Regex::new(r"(\d+)").unwrap();

        for capture in starting_item_pattern.captures_iter(starting_item_line) {
            if let Some(captured_item) = capture.get(1) {
                // println!("{}", captured_item.as_str());
                // monkeys.last_mut().unwrap().items.push(Item {
                //     worry_level: captured_item.as_str().parse().ok().unwrap(),
                // })
                item_vec.push(Item {
                    worry_level: captured_item.as_str().parse().ok().unwrap(),
                });
            }
        }

        let operation = Regex::new(r"(Operation: new = old (.+))").unwrap();
        let operation = operation
            .captures(monkey_content)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str();
        println!("operation: {}", operation);
        let operator = &operation[..1];
        let subject = &operation[2..];

        // monkeys.push(Monkey {
        //     items: item_vec,
        //     total_items_inspected: 0,
        //     operation: todo!(),
        // });
    }

    // println!("{:?}", monkeys);
}
