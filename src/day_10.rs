use std::fs::{self, read};

use regex::{Captures, Regex};

fn read_input() -> String {
    let file_path = "public/day10.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

enum InstructionType {
    Noop,
    Addx,
}
struct Instruction {
    instruction_type: InstructionType,
    completion_pointer: i32,
    value: i32,
}

impl Instruction {
    fn new(start_cycle: i32, instruction_type: InstructionType, value: i32) -> Instruction {
        match instruction_type {
            InstructionType::Noop => Instruction {
                instruction_type: InstructionType::Noop,
                completion_pointer: start_cycle,
                value: 0,
            },
            InstructionType::Addx => Instruction {
                instruction_type: InstructionType::Addx,
                completion_pointer: 2,
                value,
            },
        }
    }
}

pub fn question_one() {
    let binding = read_input();
    let content: Vec<&str> = binding.split('\n').collect();
    let addx_pattern = Regex::new(r"addx\s(-*[0-9]+)").unwrap();
    let mut cycle = 1;
    let mut x = 1;
    let mut addx_instructions: Vec<Instruction> = Vec::new();
    let mut signal_strength_sum = 0;

    let mut instructions = content.iter();
    let mut instruction = *instructions.next().unwrap();
    loop {
        println!("CYCLE {} START======================", cycle);
        let caps = addx_pattern.captures(instruction);

        if let Some(ref cap) = caps {
            let addx_value: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            println!("ADDX {}", addx_value);
            addx_instructions.push(Instruction::new(cycle, InstructionType::Addx, addx_value));
        } else {
        }

        if caps.is_some() {
            for _ in 0..2 {
                cycle += 1;

                addx_instructions.iter_mut().for_each(|addx| {
                    addx.completion_pointer -= 1;

                    if addx.completion_pointer == 0 {
                        println!("ADDX VALUE {} SUCCESSFULLY ADDED", addx.value);
                        x += addx.value;
                        println!("x IS NOW {}", x);
                    }
                });
                addx_instructions.retain(|addx| addx.completion_pointer != 0);

                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        println!("{} * {} = {}", cycle, x, cycle * x);

                        signal_strength_sum += cycle * x;
                        println!("Total signal strength: {}", signal_strength_sum);
                    }
                    _ => (),
                }
            }
        } else {
            cycle += 1;

            addx_instructions.iter_mut().for_each(|addx| {
                addx.completion_pointer -= 1;

                if addx.completion_pointer == 0 {
                    println!("ADDX VALUE {} SUCCESSFULLY ADDED", addx.value);
                    x += addx.value;
                    println!("x IS NOW {}", x);
                }
            });
            addx_instructions.retain(|addx| addx.completion_pointer != 0);
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    println!("{} * {} = {}", cycle, x, cycle * x);

                    signal_strength_sum += cycle * x;
                    println!("Total signal strength: {}", signal_strength_sum);
                }
                _ => (),
            }
        }

        println!("CYCLE {} END============", cycle);

        let instruction_option = instructions.next();
        if let Some(i) = instruction_option {
            instruction = i;
        } else {
            break;
        }
    }

    println!("{}", signal_strength_sum);
}
