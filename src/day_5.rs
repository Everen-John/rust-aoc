use regex::Regex;
use std::fs;

fn read_input() -> String {
    let file_path = "public/day5.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let binding = read_input();
    let (boxes, moves) = binding.split_at(binding.find("\n\n").unwrap());
    let mut stacks: [Vec<char>; 9] = Default::default();

    boxes
        .split(|character| character == '\n')
        .rev()
        .skip(1)
        .for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c != &' ')
                .for_each(|(i, c)| stacks[i].push(c))
        });

    // println!("{:?}", stacks);

    println!("moves {}", moves);
    moves.replace("\n\n", "").split("\n").for_each(|line| {
        let mut numbers: Vec<u32> = line
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect();

        let (e, s, n) = (
            numbers.pop().unwrap(),
            numbers.pop().unwrap(),
            numbers.pop().unwrap(),
        );

        println!("e {} s {} n {}", e, s, n);

        for _ in 0..n {
            let item = stacks[(s - 1) as usize].pop().unwrap();
            stacks[(e - 1) as usize].push(item);
        }

        println!("{:?}", stacks);
    });

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
}
//move 4 from 6 to 1 => 4,6,1
