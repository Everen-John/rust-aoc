use std::cmp::Ordering;
use std::fs;

fn read_input() -> String {
    let file_path = "public/day4.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let content = read_input();
    let pairs: Vec<&str> = content.split('\n').collect();
    let mut total_pairs_containing_the_other = 0;

    for pair in pairs {
        let elf_sections: Vec<&str> = pair.split(',').collect();
        let mut elves: Vec<Vec<u8>> = Vec::new();
        for elf_section in elf_sections {
            let section_range_values: Vec<&str> = elf_section.split('-').collect();
            let mut section_range_array: Vec<u8> = Vec::new();
            for i in section_range_values[0].parse::<u8>().unwrap()
                ..=section_range_values[1].parse::<u8>().unwrap()
            {
                section_range_array.push(i);
            }
            elves.push(section_range_array);
        }

        match (elves[0].len()).cmp(&(elves[1].len())) {
            Ordering::Greater => {
                let difference: Vec<&u8> =
                    elves[1].iter().filter(|x| !elves[0].contains(x)).collect();
                if difference.is_empty() {
                    total_pairs_containing_the_other += 1;
                }
            }
            Ordering::Less => {
                let difference: Vec<&u8> =
                    elves[0].iter().filter(|x| !elves[1].contains(x)).collect();
                if difference.is_empty() {
                    total_pairs_containing_the_other += 1;
                }
            }
            Ordering::Equal => {
                let difference: Vec<&u8> =
                    elves[0].iter().filter(|x| !elves[1].contains(x)).collect();
                if difference.is_empty() {
                    total_pairs_containing_the_other += 1;
                }
            }
        }
    }
    println!(
        "Total pairs overlapping completely: {}",
        total_pairs_containing_the_other
    );
}

pub fn question_two() {
    let content = read_input();
    let pairs: Vec<&str> = content.split('\n').collect();
    let mut total_pairs_containing_the_other = 0;

    for pair in pairs {
        let elf_sections: Vec<&str> = pair.split(',').collect();
        let mut elves: Vec<Vec<u8>> = Vec::new();
        for elf_section in elf_sections {
            let section_range_values: Vec<&str> = elf_section.split('-').collect();
            let mut section_range_array: Vec<u8> = Vec::new();
            for i in section_range_values[0].parse::<u8>().unwrap()
                ..=section_range_values[1].parse::<u8>().unwrap()
            {
                section_range_array.push(i);
            }
            elves.push(section_range_array);
        }

        match (elves[0].len()).cmp(&(elves[1].len())) {
            Ordering::Greater => {
                let overlap = elves[1].iter().any(|x| elves[0].contains(x));
                if overlap {
                    total_pairs_containing_the_other += 1;
                }
            }
            Ordering::Less => {
                let overlap = elves[0].iter().any(|x| elves[1].contains(x));
                if overlap {
                    total_pairs_containing_the_other += 1;
                }
            }
            Ordering::Equal => {
                let overlap = elves[0].iter().any(|x| elves[1].contains(x));
                if overlap {
                    total_pairs_containing_the_other += 1;
                }
            }
        }
    }
    println!(
        "Total pairs overlapping on a single section: {}",
        total_pairs_containing_the_other
    );
}
