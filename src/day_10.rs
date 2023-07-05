use std::fs::{self};

use regex::Regex;

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
                perform_a_cycle(
                    &mut x,
                    &mut cycle,
                    &mut addx_instructions,
                    &mut signal_strength_sum,
                )
            }
        } else {
            perform_a_cycle(
                &mut x,
                &mut cycle,
                &mut addx_instructions,
                &mut signal_strength_sum,
            )
        }

        let instruction_option = instructions.next();
        if let Some(i) = instruction_option {
            instruction = i;
        } else {
            break;
        }
    }

    println!("{}", signal_strength_sum);
}

fn perform_a_cycle(
    x: &mut i32,
    cycle: &mut i32,
    addx_instructions: &mut Vec<Instruction>,
    signal_strength_sum: &mut i32,
) {
    *cycle += 1;

    println!("CYCLE {} START ===================", cycle);

    addx_instructions.iter_mut().for_each(|addx| {
        addx.completion_pointer -= 1;

        if addx.completion_pointer == 0 {
            println!("ADDX VALUE {} SUCCESSFULLY ADDED", addx.value);
            *x += addx.value;
            println!("x IS NOW {}", x);
        }
    });
    addx_instructions.retain(|addx| addx.completion_pointer != 0);

    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => {
            println!("{} * {} = {}", *cycle, *x, *cycle * *x);

            *signal_strength_sum += *cycle * *x;
            println!("Total signal strength: {}", signal_strength_sum);
        }
        _ => (),
    }
    println!("CYCLE {} END ============", cycle);
}

pub fn question_two() {
    let binding = read_input();
    let content: Vec<&str> = binding.split('\n').collect();
    let addx_pattern = Regex::new(r"addx\s(-*[0-9]+)").unwrap();
    let mut cycle = 1;
    let mut x = 1;
    let mut addx_instructions: Vec<Instruction> = Vec::new();
    let mut signal_strength_sum = 0;
    let mut crt_screen: Vec<Vec<String>> = Vec::new();
    crt_screen.push(Vec::new());

    let mut instructions = content.iter();
    let mut instruction = *instructions.next().unwrap();
    loop {
        let caps = addx_pattern.captures(instruction);

        if let Some(ref cap) = caps {
            let addx_value: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            println!("ADDX {}", addx_value);
            addx_instructions.push(Instruction::new(cycle, InstructionType::Addx, addx_value));
        } else {
        }

        if caps.is_some() {
            for _ in 0..2 {
                q2_perform_a_cycle(
                    &mut x,
                    &mut cycle,
                    &mut addx_instructions,
                    &mut signal_strength_sum,
                    &mut crt_screen,
                )
            }
        } else {
            q2_perform_a_cycle(
                &mut x,
                &mut cycle,
                &mut addx_instructions,
                &mut signal_strength_sum,
                &mut crt_screen,
            )
        }

        let instruction_option = instructions.next();
        if let Some(i) = instruction_option {
            instruction = i;
        } else {
            break;
        }
    }

    println!("{}", signal_strength_sum);
    for row in crt_screen {
        println!("{:?}", row.concat());
    }
}

fn q2_perform_a_cycle(
    x: &mut i32,
    cycle: &mut i32,
    addx_instructions: &mut Vec<Instruction>,
    signal_strength_sum: &mut i32,
    crt_screen: &mut Vec<Vec<String>>,
) {
    let row = crt_screen.last_mut().unwrap();

    println!("CYCLE {} START ===================", cycle);

    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => {
            println!("{} * {} = {}", *cycle, *x, *cycle * *x);

            *signal_strength_sum += *cycle * *x;
            println!("Total signal strength: {}", signal_strength_sum);
        }
        _ => (),
    }
    println!("CYCLE {} END ============", cycle);

    addx_instructions.iter_mut().for_each(|addx| {
        addx.completion_pointer -= 1;

        if addx.completion_pointer == 0 {
            println!("ADDX VALUE {} SUCCESSFULLY ADDED", addx.value);
            *x += addx.value;
            println!("x IS NOW {}", x);
        }
    });
    addx_instructions.retain(|addx| addx.completion_pointer != 0);
    if *cycle % 40 <= *x + 1 && *cycle % 40 >= *x - 1 {
        println!("#");
        row.push("#".to_string());
    } else {
        row.push(".".to_string());
    }

    if *cycle % 40 == 0 {
        println!("cycle at {}", cycle);
        crt_screen.push(Vec::new());
    }

    *cycle += 1;
}
