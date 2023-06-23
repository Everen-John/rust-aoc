use std::collections::HashMap;
use std::fs;

fn read_input() -> String {
    let file_path = "public/day3.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let content = read_input();
    let rucksacks: Vec<&str> = content.split('\n').collect();

    let mut priority_total = 0;
    let priorities = item_priority_map();
    for rucksack in rucksacks {
        let (segment_one, segment_two) = rucksack.split_at(rucksack.len() / 2);
        let mut items: HashMap<char, bool> = HashMap::new();

        for item in segment_one.chars() {
            items.insert(item, true);
        }

        for item in segment_two.chars() {
            let check = items.get(&item).get_or_insert(&false).clone();
            if check {
                priority_total += priorities.get(&item).unwrap();
                break;
            }
        }
    }
    println!("priority total: {}", priority_total);
}

pub fn question_two() {
    let content = read_input();
    let rucksacks: Vec<&str> = content.split('\n').collect();
    let priorities = item_priority_map();
    let mut priority_total = 0;

    for i in (2..rucksacks.len()).step_by(3) {
        let mut rucksack_group: [&str; 3] = Default::default();
        rucksack_group.clone_from_slice(&rucksacks[i - 2..=i]);

        let item_type = rucksack_group[0]
            .find(|b| rucksack_group[1].contains(b) && rucksack_group[2].contains(b))
            .unwrap();
        priority_total += priorities
            .get(&rucksack_group[0].chars().nth(item_type).unwrap())
            .unwrap();
    }

    println!("priority total: {}", priority_total);
}

fn item_priority_map() -> HashMap<char, u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    priorities.insert('a', 1);
    priorities.insert('b', 2);
    priorities.insert('c', 3);
    priorities.insert('d', 4);
    priorities.insert('e', 5);
    priorities.insert('f', 6);
    priorities.insert('g', 7);
    priorities.insert('h', 8);
    priorities.insert('i', 9);
    priorities.insert('j', 10);
    priorities.insert('k', 11);
    priorities.insert('l', 12);
    priorities.insert('m', 13);
    priorities.insert('n', 14);
    priorities.insert('o', 15);
    priorities.insert('p', 16);
    priorities.insert('q', 17);
    priorities.insert('r', 18);
    priorities.insert('s', 19);
    priorities.insert('t', 20);
    priorities.insert('u', 21);
    priorities.insert('v', 22);
    priorities.insert('w', 23);
    priorities.insert('x', 24);
    priorities.insert('y', 25);
    priorities.insert('z', 26);
    priorities.insert('A', 27);
    priorities.insert('B', 28);
    priorities.insert('C', 29);
    priorities.insert('D', 30);
    priorities.insert('E', 31);
    priorities.insert('F', 32);
    priorities.insert('G', 33);
    priorities.insert('H', 34);
    priorities.insert('I', 35);
    priorities.insert('J', 36);
    priorities.insert('K', 37);
    priorities.insert('L', 38);
    priorities.insert('M', 39);
    priorities.insert('N', 40);
    priorities.insert('O', 41);
    priorities.insert('P', 42);
    priorities.insert('Q', 43);
    priorities.insert('R', 44);
    priorities.insert('S', 45);
    priorities.insert('T', 46);
    priorities.insert('U', 47);
    priorities.insert('V', 48);
    priorities.insert('W', 49);
    priorities.insert('X', 50);
    priorities.insert('Y', 51);
    priorities.insert('Z', 52);

    priorities
}
