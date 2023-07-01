use std::fs::{self};

fn read_input() -> String {
    let file_path = "public/day8.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

pub fn question_one() {
    let content = read_input();
    let (x, y, grid) = process_content(&content);

    let mut total_visible_trees = 0;
    total_visible_trees += y * 2;
    for i in 1..(x - 1) {
        total_visible_trees += 2;
        for j in 1..(y - 1) {
            let current_tree = grid.get(i).unwrap().get(j).unwrap();

            let top_tree = grid
                .iter()
                .take(i)
                .any(|row| row.get(j).unwrap() >= current_tree);

            let right_tree = grid
                .get(i)
                .unwrap()
                .iter()
                .skip(j + 1)
                .any(|column| column >= current_tree);

            let bottom_tree = grid
                .iter()
                .skip(i + 1)
                .any(|row| row.get(j).unwrap() >= current_tree);

            let left_tree = grid
                .get(i)
                .unwrap()
                .iter()
                .take(j)
                .any(|column| column >= current_tree);

            if !top_tree || !bottom_tree || !left_tree || !right_tree {
                total_visible_trees += 1;
            }
        }
    }
    println!("total visible trees: {}", total_visible_trees);
}

//  -> (usize, usize, Vec<Vec<u8>>)
fn process_content(content: &str) -> (usize, usize, Vec<Vec<u8>>) {
    let lines: Vec<&str> = content.split('\n').collect();
    let x = lines.len();
    let y = lines.get(1).unwrap().len();
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for _ in 0..x {
        grid.push(Vec::new());
    }

    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().for_each(|c| {
            grid.get_mut(i)
                .unwrap()
                .push(c.to_string().parse().unwrap())
        })
    });
    (x, y, grid)
}

pub fn question_two() {
    let content = read_input();
    let (x, y, grid) = process_content(&content);

    let mut total_visible_trees = 0;
    total_visible_trees += y * 2;
    let mut highest_scenic_score = 0;
    for i in 1..(x - 1) {
        total_visible_trees += 2;
        for j in 1..(y - 1) {
            let current_tree = grid.get(i).unwrap().get(j).unwrap();
            let mut top_score = 0;
            let mut right_score = 0;
            let mut bottom_score = 0;
            let mut left_score = 0;

            let top_tree = grid.iter().take(i).rev().any(|row| {
                top_score += 1;
                row.get(j).unwrap() >= current_tree
            });

            let right_tree = grid.get(i).unwrap().iter().skip(j + 1).any(|column| {
                right_score += 1;
                column >= current_tree
            });

            let bottom_tree = grid.iter().skip(i + 1).any(|row| {
                bottom_score += 1;
                row.get(j).unwrap() >= current_tree
            });

            let left_tree = grid.get(i).unwrap().iter().take(j).rev().any(|column| {
                left_score += 1;
                column >= current_tree
            });

            if !top_tree || !bottom_tree || !left_tree || !right_tree {
                total_visible_trees += 1;
                let scenic_score = top_score * right_score * bottom_score * left_score;
                if scenic_score > highest_scenic_score {
                    highest_scenic_score = scenic_score;
                }
            }
        }
    }
    println!("total visible trees: {}", total_visible_trees);
    println!("highest_scenic_score: {}", highest_scenic_score);
}
