use std::fs;

struct Elf {
    calories: Vec<u32>,
}

trait HasFood {
    fn total_calories(&self) -> u32;
}

impl HasFood for Elf {
    fn total_calories(&self) -> u32 {
        let val: u32 = self.calories.iter().sum();
        val
    }
}

pub fn question_one() {
    let file_path = "public/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file!");

    let elf_calorie_sets: Vec<&str> = contents.split("\n\n").collect();
    let mut elves: Vec<Elf> = Vec::new();

    for calorie_set in elf_calorie_sets {
        elves.push(Elf {
            calories: calorie_set
                .split("\n")
                .map(|e| e.parse().unwrap())
                .collect(),
        })
    }

    let mut highest_calorie = 0;
    for elf in elves {
        let total_calories = elf.total_calories();
        if total_calories > highest_calorie {
            highest_calorie = total_calories;
        }
    }
    println!("Highest Calories: {}", highest_calorie);
}

pub fn question_two() {
    let file_path = "public/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file!");

    let elf_calorie_sets: Vec<&str> = contents.split("\n\n").collect();
    let mut elves: Vec<Elf> = Vec::new();

    for calorie_set in elf_calorie_sets {
        elves.push(Elf {
            calories: calorie_set
                .split("\n")
                .map(|e| e.parse().unwrap())
                .collect(),
        })
    }

    let mut calorie_values: Vec<u32> = Vec::new();
    for elf in elves {
        calorie_values.push(elf.total_calories());
        calorie_values.sort_by(|a, b| b.cmp(a));
    }
    let mut total_calories = 0;
    for i in 0..=2 {
        total_calories += calorie_values[i];
    }
    println!("Total Top 3 calorie values: {}", total_calories);
}
